#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Measure where a distributed executable's bytes go.

The command resolves a named revision into a temporary worktree, builds the
shipped artifact (or reads a supplied one), builds the same revision again with
`strip` disabled for that build only, attributes the name-retaining build with
the pinned `cargo-bloat`, and writes `attribution.json` and `attribution.md`.

The stripped executable's file size and the attribution build's `.text` total
are separate fields filled by separate reads. They are never substituted for
each other, and a host build or a release archive is never accepted as
evidence about the supported platform.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

PLATFORM = "x86_64-pc-windows-gnu"
UNREADABLE_EXIT = 1
UNRESOLVED_EXIT = 2
GENERATOR = "cargo-bloat"
TWO_NUMBER_SENTENCE = (
    "The size attribution is not the stripped executable's file size: the two\n"
    "numbers are different quantities and are never substituted for each other.\n"
    "The attribution's numbers are estimates of the `.text` section, not a\n"
    "byte-exact accounting."
)


class MeasurementError(Exception):
    """A measurement that cannot produce an attribution."""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run(
    command: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    check: bool = False,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        env=env,
        text=True,
        capture_output=True,
        check=check,
    )


def first_line(text: str) -> str:
    for line in text.splitlines():
        line = line.strip()
        if line:
            return line
    return ""


def error_detail(text: str, worktree: Path, fallback: str) -> str:
    """Keep the useful tail of a tool failure without the temp worktree path."""
    lines = [line.rstrip() for line in text.strip().splitlines() if line.strip()]
    if not lines:
        return fallback
    return "\n".join(lines[-20:]).replace(str(worktree), "<worktree>")


def resolve_revision(repo: Path, revision: str) -> str | None:
    result = run(
        ["git", "rev-parse", "--verify", f"{revision}^{{commit}}"],
        cwd=repo,
        env=dict(os.environ),
    )
    if result.returncode != 0:
        print(
            f"measure-executable-size: the revision {revision!r} could not be "
            f"resolved: {first_line(result.stderr) or 'git rev-parse failed'}",
            file=sys.stderr,
        )
        return None
    return first_line(result.stdout)


def build_environment(target_dir: Path, *, strip_names: bool) -> dict[str, str]:
    env = dict(os.environ)
    for key in (
        "CARGO_TARGET_DIR",
        "CARGO_BUILD_TARGET",
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_RUSTFLAGS",
    ):
        env.pop(key, None)
    for key in list(env):
        if key.startswith("CARGO_PROFILE_"):
            env.pop(key, None)
    env["CARGO_TARGET_DIR"] = str(target_dir)
    if strip_names:
        env["CARGO_PROFILE_RELEASE_STRIP"] = "false"
    return env


def create_worktree(repo: Path, revision: str, root: Path) -> Path:
    worktree = root / "worktree"
    result = run(
        ["git", "worktree", "add", "--detach", str(worktree), revision],
        cwd=repo,
        env=dict(os.environ),
    )
    if result.returncode != 0:
        raise MeasurementError(
            "the measurement worktree could not be created: "
            f"{first_line(result.stderr) or 'git worktree add failed'}"
        )
    return worktree


def remove_worktree(repo: Path, worktree: Path) -> None:
    run(
        ["git", "worktree", "remove", "--force", str(worktree)],
        cwd=repo,
        env=dict(os.environ),
    )
    run(["git", "worktree", "prune"], cwd=repo, env=dict(os.environ))


def artifact_description(data: bytes) -> str:
    if len(data) < 2 or data[:2] != b"MZ":
        return "not a PE file"
    if len(data) < 64:
        return "truncated DOS header"
    pe_offset = struct.unpack_from("<I", data, 0x3C)[0]
    if len(data) < pe_offset + 6 or data[pe_offset : pe_offset + 4] != b"PE\0\0":
        return "not a PE image"
    machine = struct.unpack_from("<H", data, pe_offset + 4)[0]
    return f"PE machine 0x{machine:04x}"


def is_supported_executable(path: Path) -> bool:
    try:
        with path.open("rb") as handle:
            dos_header = handle.read(64)
            if len(dos_header) < 64 or dos_header[:2] != b"MZ":
                return False
            pe_offset = struct.unpack_from("<I", dos_header, 0x3C)[0]
            handle.seek(pe_offset)
            pe_header = handle.read(6)
            if len(pe_header) < 6 or pe_header[:4] != b"PE\0\0":
                return False
            machine = struct.unpack_from("<H", pe_header, 4)[0]
            return machine == 0x8664
    except OSError:
        return False


def find_built_executable(target_dir: Path) -> Path | None:
    release = target_dir / PLATFORM / "release"
    if not release.is_dir():
        return None
    candidates = [
        path
        for path in sorted(release.glob("*.exe"))
        if path.name != "build-script-build.exe" and not path.name.startswith("build_script")
    ]
    if not candidates:
        return None
    return candidates[0]


def read_sections(executable: Path) -> list[dict[str, object]] | None:
    objdump = shutil.which("x86_64-w64-mingw32-objdump")
    if objdump is None:
        print(
            "measure-executable-size: x86_64-w64-mingw32-objdump is not "
            "available; section sizes will be omitted",
            file=sys.stderr,
        )
        return None
    result = run([objdump, "-h", str(executable)], cwd=executable.parent, env=dict(os.environ))
    if result.returncode != 0:
        print(
            "measure-executable-size: x86_64-w64-mingw32-objdump could not "
            f"read the artifact: {first_line(result.stderr) or 'no output'}",
            file=sys.stderr,
        )
        return None
    sections: list[dict[str, object]] = []
    for line in result.stdout.splitlines():
        fields = line.split()
        if len(fields) < 3:
            continue
        if not fields[0].isdigit():
            continue
        name = fields[1]
        try:
            size = int(fields[2], 16)
        except ValueError:
            continue
        if size == 0:
            continue
        sections.append({"name": name, "bytes": size})
    return sections


def supported_sections(executable: Path) -> list[dict[str, object]] | None:
    """The section table, or None when it cannot name the contract's `.text`."""
    sections = read_sections(executable)
    if sections is None:
        return None
    if not any(section["name"] == ".text" for section in sections):
        return None
    return sections


def build_shipped(
    worktree: Path, target_dir: Path, env: dict[str, str]
) -> tuple[Path | None, str | None]:
    result = run(
        [
            "cargo",
            "build",
            "--release",
            "--locked",
            "--target",
            PLATFORM,
        ],
        cwd=worktree,
        env=env,
    )
    if result.returncode != 0:
        detail = error_detail(
            result.stderr or result.stdout, worktree, "cargo build failed"
        )
        return None, f"the build could not be read: {detail}"
    executable = find_built_executable(target_dir)
    if executable is None:
        return None, "the build could not be read: no executable was produced"
    return executable, None


def generator_version(env: dict[str, str], cwd: Path) -> str | None:
    try:
        result = run(["cargo", "bloat", "--version"], cwd=cwd, env=env)
    except FileNotFoundError:
        return None
    if result.returncode != 0:
        return None
    line = first_line(result.stdout)
    # `cargo-bloat 0.12.1` or `cargo bloat 0.12.1`.
    parts = line.split()
    return parts[-1] if parts else None


def run_bloat(
    worktree: Path,
    env: dict[str, str],
    *,
    crates: bool,
) -> dict[str, object]:
    command = [
        "cargo",
        "bloat",
        "--release",
        "--locked",
        "--target",
        PLATFORM,
        "--message-format",
        "json",
        "-n",
        "0",
    ]
    if crates:
        command.append("--crates")
    try:
        result = run(command, cwd=worktree, env=env)
    except FileNotFoundError as error:
        raise MeasurementError(f"the generator is not available: {error}") from error
    if result.returncode != 0:
        detail = error_detail(
            result.stderr or result.stdout, worktree, "cargo bloat failed"
        )
        raise MeasurementError(
            "the build cannot retain the names the attribution needs: " + detail
        )
    payload = None
    for line in reversed(result.stdout.splitlines()):
        line = line.strip()
        if not line:
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        break
    if not isinstance(payload, dict):
        raise MeasurementError(
            "the build cannot retain the names the attribution needs: "
            "the generator produced no JSON"
        )
    return payload


def read_build_configuration(worktree: Path) -> dict[str, str | None]:
    configuration: dict[str, str | None] = {}
    for name in ("Cargo.toml", ".cargo/config.toml"):
        path = worktree / name
        configuration[name] = sha256_file(path) if path.is_file() else None
    return configuration


def attribution_from_generator(
    crates_payload: dict[str, object], functions_payload: dict[str, object]
) -> dict[str, object]:
    crates = crates_payload.get("crates")
    functions = functions_payload.get("functions")
    if not isinstance(crates, list) or not crates:
        raise MeasurementError(
            "the build cannot retain the names the attribution needs: "
            "the generator reported no crates"
        )
    if not isinstance(functions, list):
        functions = []
    total = crates_payload.get("text-section-size")
    build_file_size = crates_payload.get("file-size")
    if not isinstance(total, int) or not isinstance(build_file_size, int):
        raise MeasurementError(
            "the build cannot retain the names the attribution needs: "
            "the generator reported no totals"
        )
    return {
        "unit": "bytes",
        "total": total,
        "build_file_size": build_file_size,
        "crates": [
            {"name": str(item.get("name", "[Unknown]")), "bytes": int(item["size"])}
            for item in crates
            if isinstance(item, dict) and "size" in item
        ],
        "functions": [
            {
                "crate": str(item.get("crate", "[Unknown]")),
                "name": str(item.get("name", "[Unknown]")),
                "bytes": int(item["size"]),
            }
            for item in functions
            if isinstance(item, dict) and "size" in item
        ],
    }


def markdown_report(report: dict[str, object]) -> str:
    lines: list[str] = ["# Size attribution", ""]
    lines.append(f"- Requested: {report['requested']}")
    lines.append(f"- Revision: {report['revision']}")
    lines.append(f"- Platform: {report['platform']}")
    lines.append(f"- Status: {report['status']}")
    lines.append("")
    lines.append("## Build identity")
    lines.append("")
    lines.append(f"- Toolchain: {report['toolchain']}")
    generator = report["generator"]
    if isinstance(generator, dict):
        lines.append(f"- Generator: {generator.get('name')} {generator.get('version')}")
    configuration = report["build_configuration"]
    if isinstance(configuration, dict):
        for name in ("Cargo.toml", ".cargo/config.toml"):
            value = configuration.get(name)
            lines.append(f"- {name}: {value if value else 'unavailable'}")
    status = report["status"]
    executable = report["executable"]
    attribution = report["attribution"]
    reason = report.get("reason")
    if status == "unreadable":
        lines.append("")
        lines.append("## No attribution")
        lines.append("")
        lines.append(str(reason))
        return "\n".join(lines) + "\n"
    if isinstance(attribution, dict):
        lines.append(f"- Attribution build (file): {attribution['build_file_size']} B")
    lines.append("")
    lines.append("## The two numbers")
    lines.append("")
    if isinstance(executable, dict):
        lines.append(
            "- Stripped executable (file): "
            f"{executable['bytes']} B (sha256 {executable['sha256']})"
        )
    else:
        lines.append(f"- Stripped executable (file): unavailable — {reason}")
    if isinstance(attribution, dict):
        lines.append(
            "- Size attribution (attribution build .text): "
            f"{attribution['total']} B"
        )
    lines.append("")
    lines.append(TWO_NUMBER_SENTENCE)
    if isinstance(attribution, dict):
        lines.append("")
        lines.append("## By crate")
        lines.append("")
        lines.append("| crate | bytes |")
        lines.append("|---|---:|")
        for item in attribution["crates"]:
            lines.append(f"| {item['name']} | {item['bytes']} |")
        lines.append("")
        lines.append("## By function")
        lines.append("")
        functions = sorted(
            attribution["functions"], key=lambda item: item["bytes"], reverse=True
        )
        shown = functions[:25]
        lines.append(f"Largest {len(shown)} of {len(functions)} functions.")
        lines.append("")
        lines.append("| crate | function | bytes |")
        lines.append("|---|---|---:|")
        for item in shown:
            lines.append(f"| {item['crate']} | {item['name']} | {item['bytes']} |")
        lines.append("")
        lines.append("## Sections of the stripped executable")
        lines.append("")
        if isinstance(executable, dict) and executable.get("sections") is not None:
            lines.append("| section | bytes |")
            lines.append("|---|---:|")
            for section in executable["sections"]:
                lines.append(f"| {section['name']} | {section['bytes']} |")
        else:
            lines.append("The section sizes could not be read.")
    return "\n".join(lines) + "\n"


def write_report(out: Path, report: dict[str, object]) -> None:
    out.mkdir(parents=True, exist_ok=True)
    (out / "attribution.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    (out / "attribution.md").write_text(markdown_report(report), encoding="utf-8")


def measure(args: argparse.Namespace) -> int:
    repo = Path(args.repo).resolve()
    out = Path(args.out).resolve()
    revision = resolve_revision(repo, args.revision)
    if revision is None:
        return UNRESOLVED_EXIT

    root = Path(tempfile.mkdtemp(prefix="measure-executable-size-"))
    worktree: Path | None = None
    try:
        worktree = create_worktree(repo, revision, root)
        build_configuration = read_build_configuration(worktree)
        toolchain_result = run(["rustc", "--version"], cwd=worktree, env=dict(os.environ))
        toolchain = first_line(toolchain_result.stdout) or "unavailable"
        version = generator_version(dict(os.environ), worktree)
        if version is None:
            report = {
                "kind": "size-attribution",
                "status": "unreadable",
                "requested": args.revision,
                "revision": revision,
                "platform": PLATFORM,
                "toolchain": toolchain,
                "generator": {"name": GENERATOR, "version": "unavailable"},
                "build_configuration": build_configuration,
                "executable": None,
                "attribution": None,
                "reason": "the generator is not available: cargo bloat --version failed",
            }
            write_report(out, report)
            return UNREADABLE_EXIT
        report: dict[str, object] = {
            "kind": "size-attribution",
            "status": "unreadable",
            "requested": args.revision,
            "revision": revision,
            "platform": PLATFORM,
            "toolchain": toolchain,
            "generator": {"name": GENERATOR, "version": version or "unavailable"},
            "build_configuration": build_configuration,
            "executable": None,
            "attribution": None,
            "reason": None,
        }

        shipped_target = root / "shipped-target"
        executable: Path | None = None
        incomplete_reason: str | None = None

        if args.executable:
            supplied = Path(args.executable).resolve()
            try:
                data = supplied.read_bytes()
            except OSError as error:
                incomplete_reason = (
                    "the executable's file size is unavailable: "
                    + str(error).replace(str(supplied), "<executable>")
                )
            else:
                if not is_supported_executable(supplied):
                    report["reason"] = (
                        "the artifact is not the supported platform's executable: "
                        f"expected {PLATFORM}; observed {artifact_description(data)}"
                    )
                    write_report(out, report)
                    return UNREADABLE_EXIT
                executable = supplied
                report["executable"] = {
                    "source": "supplied",
                    "bytes": len(data),
                    "sha256": sha256_bytes(data),
                    "sections": supported_sections(supplied),
                }
        else:
            shipped_env = build_environment(shipped_target, strip_names=False)
            executable, build_reason = build_shipped(worktree, shipped_target, shipped_env)
            if build_reason is not None:
                report["reason"] = build_reason
                write_report(out, report)
                return UNREADABLE_EXIT
            assert executable is not None
            try:
                data = executable.read_bytes()
            except OSError as error:
                incomplete_reason = f"the executable's file size is unavailable: {error}"
            else:
                report["executable"] = {
                    "source": "built",
                    "bytes": len(data),
                    "sha256": sha256_bytes(data),
                    "sections": supported_sections(executable),
                }

        attribution_env = build_environment(root / "attribution-target", strip_names=True)
        try:
            crates_payload = run_bloat(worktree, attribution_env, crates=True)
            functions_payload = run_bloat(worktree, attribution_env, crates=False)
            attribution = attribution_from_generator(crates_payload, functions_payload)
        except MeasurementError as error:
            report["reason"] = str(error)
            write_report(out, report)
            return UNREADABLE_EXIT

        report["attribution"] = attribution
        if incomplete_reason is not None:
            report["status"] = "incomplete"
            report["reason"] = incomplete_reason
            write_report(out, report)
            return 0

        report["status"] = "complete"
        write_report(out, report)
        return 0
    except MeasurementError as error:
        report = {
            "kind": "size-attribution",
            "status": "unreadable",
            "requested": args.revision,
            "revision": revision,
            "platform": PLATFORM,
            "toolchain": "unavailable",
            "generator": {"name": GENERATOR, "version": "unavailable"},
            "build_configuration": {"Cargo.toml": None, ".cargo/config.toml": None},
            "executable": None,
            "attribution": None,
            "reason": str(error),
        }
        write_report(out, report)
        return UNREADABLE_EXIT
    finally:
        if worktree is not None:
            remove_worktree(repo, worktree)
        shutil.rmtree(root, ignore_errors=True)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="measure_executable_size.py",
        description="Produce a size attribution for the Windows x86_64 executable.",
    )
    parser.add_argument("--revision", required=True, help="the revision to describe")
    parser.add_argument("--repo", default=".", help="the repository (default: current directory)")
    parser.add_argument("--out", default="size-attribution", help="the report directory")
    parser.add_argument(
        "--executable",
        default=None,
        help="the stripped executable to describe instead of building one",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    return measure(parse_args(argv))


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
