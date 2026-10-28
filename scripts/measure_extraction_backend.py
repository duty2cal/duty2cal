#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Measure the Windows GNU executable against the same revision with the
previous extraction and lock restored.

The measurement is one-shot: when the restored base tree no longer names
`pdfplumber` in `crates/duty2cal-pdf/Cargo.toml`'s `[dependencies]`, the run
records `status: "not applicable"` with the landed size and exits 0.
"""

from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ALLOWED_PATHS = ("crates/duty2cal-pdf/", "Cargo.lock")
TARGET = "x86_64-pc-windows-gnu"
EXECUTABLE = f"target/{TARGET}/release/duty2cal.exe"


def run(args: list[str], cwd: Path, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(args, cwd=cwd, check=check, text=True, capture_output=True)


def git(repo: Path, *args: str) -> str:
    return run(["git", *args], repo).stdout.strip()


def names_extractor(manifest: Path) -> bool:
    """Whether the manifest's `[dependencies]` section names pdfplumber."""
    text = manifest.read_text()
    match = re.search(r"^\[dependencies\](.*?)(?=^\[|\Z)", text, re.S | re.M)
    return bool(match and "pdfplumber" in match.group(1))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--base", required=True, help="the resolved base revision")
    parser.add_argument("--landed", required=True, help=f"the landed executable ({EXECUTABLE})")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    repo = Path(args.repo).resolve()
    landed_path = (repo / args.landed).resolve() if not Path(args.landed).is_absolute() else Path(args.landed)
    out = (repo / args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)

    if not landed_path.is_file() or landed_path.suffix != ".exe":
        print(
            f"measure_extraction_backend: {landed_path} is not the Windows target's executable",
            file=sys.stderr,
        )
        return 1

    landed_sha = git(repo, "rev-parse", "HEAD")
    base_sha = git(repo, "rev-parse", args.base)
    landed_size = landed_path.stat().st_size

    worktree = out / "worktree"
    if worktree.exists():
        shutil.rmtree(worktree)
    run(["git", "worktree", "add", "--detach", str(worktree), landed_sha], repo)
    try:
        run(["git", "checkout", base_sha, "--", "crates/duty2cal-pdf", "Cargo.lock"], worktree)
        changed = [
            line[3:]
            for line in run(["git", "status", "--porcelain"], worktree).stdout.splitlines()
        ]
        unexpected = [
            path
            for path in changed
            if not any(path == allowed or path.startswith(allowed) for allowed in ALLOWED_PATHS)
        ]
        if unexpected:
            print(
                "measure_extraction_backend: the restored tree differs outside the allowed paths: "
                + ", ".join(unexpected),
                file=sys.stderr,
            )
            return 1

        manifest = worktree / "crates/duty2cal-pdf/Cargo.toml"
        base_lock = worktree / "Cargo.lock"
        report = {
            "status": "not applicable",
            "landed_revision": landed_sha,
            "base_revision": base_sha,
            "landed_size": landed_size,
            "target": TARGET,
            "rustc": run(["rustc", "-V"], repo).stdout.strip(),
            "landed_lock_sha256": __import__("hashlib").sha256(
                (repo / "Cargo.lock").read_bytes()
            ).hexdigest(),
            "base_lock_sha256": __import__("hashlib").sha256(base_lock.read_bytes()).hexdigest(),
        }

        if names_extractor(manifest):
            build = run(
                ["cargo", "build", "--release", "--target", TARGET, "-p", "duty2cal"],
                worktree,
                check=False,
            )
            if build.returncode != 0:
                print(build.stdout, build.stderr, file=sys.stderr)
                print("measure_extraction_backend: the restored tree does not build", file=sys.stderr)
                return 1
            base_path = worktree / EXECUTABLE
            base_size = base_path.stat().st_size
            if landed_size >= base_size:
                print(
                    f"measure_extraction_backend: the landed executable is not smaller "
                    f"({landed_size} B vs {base_size} B)",
                    file=sys.stderr,
                )
                return 1
            saved = base_size - landed_size
            report.update(
                status="measured",
                base_size=base_size,
                saved_bytes=saved,
                saved_percent=round(saved * 100.0 / base_size, 2),
            )

        (out / "extraction-backend.json").write_text(json.dumps(report, indent=2) + "\n")
        markdown = [
            "# Extraction backend measurement",
            "",
            f"- status: {report['status']}",
            f"- landed revision: `{landed_sha}`",
            f"- base revision: `{base_sha}`",
            f"- landed size: {landed_size} B",
        ]
        if report["status"] == "measured":
            markdown += [
                f"- base size: {report['base_size']} B",
                f"- saved: {report['saved_bytes']} B ({report['saved_percent']}%)",
            ]
        markdown.append("")
        (out / "extraction-backend.md").write_text("\n".join(markdown))
        print("\n".join(markdown))
        return 0
    finally:
        run(["git", "worktree", "remove", "--force", str(worktree)], repo, check=False)


if __name__ == "__main__":
    raise SystemExit(main())
