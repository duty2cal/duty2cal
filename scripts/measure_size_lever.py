#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Measure one or more size levers against a rebuilt baseline.

The command resolves a revision into a temporary worktree, rebuilds the
unchanged release configuration as the baseline, then applies each candidate
lever cumulatively and measures the stripped executable's file size. It records
every decision, refuses a lever that is not a size lever before any build, and
reports the published binary's size separately when supplied.
"""

from __future__ import annotations

import argparse
import json
import shutil
import sys
import tempfile
from pathlib import Path

import measure_executable_size as measurement

PLATFORM = "x86_64-pc-windows-gnu"
UNCHANGED = {"panic": "unwind", "opt_level": "z"}
FORBIDDEN = {
    "nightly": "the stable-toolchain constraint",
    "upx": "the single-file constraint",
    "dynamic-linking": "the static-linking constraint",
}
LEVERS = {
    "panic=abort": ("panic", "abort", "unwind"),
    "opt-level=s": ("opt_level", "s", "z"),
}


class Refused(Exception):
    pass


def read_release_profile(worktree: Path) -> dict[str, str]:
    profile = dict(UNCHANGED)
    in_release = False
    for line in (worktree / "Cargo.toml").read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped == "[profile.release]":
            in_release = True
            continue
        if in_release and stripped.startswith("["):
            break
        if not in_release or not stripped or stripped.startswith("#") or "=" not in stripped:
            continue
        key, _, value = stripped.partition("=")
        key = key.strip()
        value = value.strip().strip('"')
        if key == "panic" and value:
            profile["panic"] = value
        if key == "opt-level" and value:
            profile["opt_level"] = value
    return profile


def parse_levers(tokens: list[str]) -> list[tuple[str, str, str, str]]:
    parsed: list[tuple[str, str, str, str]] = []
    for token in tokens:
        token = token.strip()
        if token in FORBIDDEN:
            raise Refused(f"{token} is not a size lever: {FORBIDDEN[token]}")
        if token not in LEVERS:
            raise Refused(f"{token} is not a supported size lever")
        key, new, old = LEVERS[token]
        parsed.append((token, key, new, old))
    if not parsed:
        raise Refused("at least one lever is required")
    return parsed


def build(
    worktree: Path, config: dict[str, str] | None, target: Path
) -> tuple[Path, int]:
    env = measurement.build_environment(target, strip_names=False)
    if config is not None:
        env["CARGO_PROFILE_RELEASE_PANIC"] = config["panic"]
        env["CARGO_PROFILE_RELEASE_OPT_LEVEL"] = config["opt_level"]
    result = measurement.run(
        ["cargo", "build", "--release", "--locked", "--target", PLATFORM],
        cwd=worktree,
        env=env,
    )
    if result.returncode != 0:
        detail = measurement.error_detail(
            result.stderr or result.stdout, worktree, "cargo build failed"
        )
        raise Refused(f"the build failed: {detail}")
    executable = measurement.find_built_executable(target)
    if executable is None:
        raise Refused("the build produced no executable")
    return executable, executable.stat().st_size


def markdown(report: dict[str, object]) -> str:
    lines = [
        "# Size lever",
        "",
        f"- Requested: {report['requested']}",
        f"- Revision: {report['revision']}",
        f"- Platform: {report['platform']}",
        f"- Levers: {', '.join(report['levers'])}",
        f"- Status: {report['status']}",
    ]
    published = report.get("published_binary_bytes")
    if isinstance(published, int):
        lines.append(f"- Published binary (file): {published} B (reported separately)")
    else:
        lines.append("- Published binary (file): not supplied")
    status = report["status"]
    if status == "failed":
        lines.extend(["", "## No measurement", "", str(report.get("reason"))])
        return "\n".join(lines) + "\n"
    lines.extend(["", "## Baseline", ""])
    lines.append(f"- Baseline (rebuild, unchanged): {report['baseline_bytes']} B")
    lines.extend(
        [
            "",
            "## Cumulative",
            "",
            "| lever | baseline bytes | lever bytes | saved bytes | percent | status |",
            "|---|---:|---:|---:|---:|---|",
        ]
    )
    for item in report["builds"]:
        lines.append(
            f"| {item['lever']} | {item['baseline_bytes']} | {item['lever_bytes']} | "
            f"{item['saved_bytes']} | {item['percent']:.2f}% | {item['status']} |"
        )
    lines.extend(["", "## Landed", ""])
    landed = report.get("landed")
    if isinstance(landed, dict):
        lines.append(f"- Landed (committed configuration): {landed['bytes']} B")
        lines.append(f"- Landed status: {landed['status']}")
    lines.extend(["", "## Failure outcome", ""])
    lines.append("- Panic: the standard Rust panic message on stderr; the process aborts.")
    lines.append(
        "- Conversion failure: a typed failure still attempts a numbered diagnostic naming the source and reason."
    )
    executable = report.get("lever_executable")
    if executable:
        lines.append(f"- Lever executable: {executable}")
    return "\n".join(lines) + "\n"


def measure(args: argparse.Namespace) -> int:
    repo = Path(args.repo).resolve()
    out = Path(args.out).resolve()
    try:
        levers = parse_levers(args.lever.split(","))
    except Refused as error:
        print(f"size-lever: {error}", file=sys.stderr)
        return 2

    revision = measurement.resolve_revision(repo, args.revision)
    if revision is None:
        return 2

    root = Path(tempfile.mkdtemp(prefix="measure-size-lever-"))
    worktree: Path | None = None
    try:
        worktree = measurement.create_worktree(repo, revision, root)
        current = read_release_profile(worktree)
        baseline_executable, baseline_bytes = build(
            worktree, dict(UNCHANGED), root / "baseline"
        )
        report: dict[str, object] = {
            "kind": "size-lever",
            "status": "rejected",
            "requested": args.revision,
            "revision": revision,
            "platform": PLATFORM,
            "levers": [name for name, _, _, _ in levers],
            "published_binary_bytes": args.published_size,
            "baseline_bytes": baseline_bytes,
            "baseline_sha256": measurement.sha256_file(baseline_executable),
            "builds": [],
            "landed": None,
            "lever_executable": None,
            "failure_outcome": {
                "panic": "standard Rust panic message on stderr; process aborts",
                "conversion": "typed failure still attempts a numbered diagnostic",
            },
            "reason": None,
        }
        previous_executable = baseline_executable
        previous_bytes = baseline_bytes
        previous_config = dict(UNCHANGED)
        accepted = False

        for token, key, new, old in levers:
            baseline_config = dict(current)
            baseline_config[key] = old
            if baseline_config != previous_config:
                previous_executable, previous_bytes = build(
                    worktree, baseline_config, root / f"baseline-{token.replace('=', '-')}"
                )
                previous_config = baseline_config
            lever_config = dict(current)
            lever_config[key] = new
            lever_executable, lever_bytes = build(
                worktree, lever_config, root / f"lever-{token.replace('=', '-')}"
            )
            saved_bytes = previous_bytes - lever_bytes
            percent = (saved_bytes / previous_bytes * 100.0) if previous_bytes else 0.0
            status = "applied" if saved_bytes > 0 else "rejected"
            report["builds"].append(
                {
                    "lever": token,
                    "baseline_bytes": previous_bytes,
                    "lever_bytes": lever_bytes,
                    "saved_bytes": saved_bytes,
                    "percent": percent,
                    "status": status,
                }
            )
            if status == "applied":
                accepted = True
                current = lever_config
                previous_executable = lever_executable
                previous_bytes = lever_bytes
                previous_config = lever_config

        landed_executable, landed_bytes = build(worktree, None, root / "landed")
        landed_status = "smaller" if landed_bytes < baseline_bytes else "not_smaller"
        report["landed"] = {
            "bytes": landed_bytes,
            "sha256": measurement.sha256_file(landed_executable),
            "status": landed_status,
        }
        if accepted and landed_status == "smaller":
            report["status"] = "applied"
        elif accepted:
            report["status"] = "not_landed"
        else:
            report["status"] = "rejected"

        out.mkdir(parents=True, exist_ok=True)
        preserved = out / "duty2cal-lever.exe"
        shutil.copy2(landed_executable, preserved)
        report["lever_executable"] = str(preserved)

        out.mkdir(parents=True, exist_ok=True)
        (out / "size-lever.json").write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
        (out / "size-lever.md").write_text(markdown(report), encoding="utf-8")
        return 0
    except (Refused, measurement.MeasurementError) as error:
        report = {
            "kind": "size-lever",
            "status": "failed",
            "requested": args.revision,
            "revision": revision,
            "platform": PLATFORM,
            "levers": [name for name, _, _, _ in levers],
            "published_binary_bytes": args.published_size,
            "baseline_bytes": None,
            "builds": [],
            "landed": None,
            "lever_executable": None,
            "failure_outcome": None,
            "reason": str(error),
        }
        out.mkdir(parents=True, exist_ok=True)
        (out / "size-lever.json").write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
        (out / "size-lever.md").write_text(markdown(report), encoding="utf-8")
        return 1
    finally:
        if worktree is not None:
            measurement.remove_worktree(repo, worktree)
        shutil.rmtree(root, ignore_errors=True)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="measure_size_lever.py",
        description="Measure one or more size levers against a rebuilt baseline.",
    )
    parser.add_argument("--revision", default="HEAD", help="the revision to measure")
    parser.add_argument("--repo", default=".", help="the repository (default: current directory)")
    parser.add_argument("--out", default="size-lever", help="the report directory")
    parser.add_argument("--lever", required=True, help="comma-separated lever names")
    parser.add_argument(
        "--published-size",
        type=int,
        default=None,
        help="the published binary's file size, reported separately",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    return measure(parse_args(argv))


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
