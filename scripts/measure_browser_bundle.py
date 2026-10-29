#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Measure the browser bundle against the same revision with the previous file-structure layer and lock restored.

Same one-shot status rule as `measure_extraction_backend.py`: a restored base
tree whose `crates/duty2cal-pdf/Cargo.toml` `[dependencies]` no longer name
`lopdf` records `status: "not applicable"` with the landed sizes and exits
0. When measured, the WASM module must be smaller; the inlined module's size is
reported beside it.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

ALLOWED_PATHS = ("crates/duty2cal-pdf/", "Cargo.lock")
ARTIFACTS = ("web/pkg/duty2cal_wasm_bg.wasm", "web/pkg/wasm-inline.js")


def run(args: list[str], cwd: Path, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(args, cwd=cwd, check=check, text=True, capture_output=True)


def git(repo: Path, *args: str) -> str:
    return run(["git", *args], repo).stdout.strip()


def names_previous_layer(manifest: Path) -> bool:
    """Whether the manifest's `[dependencies]` section names the removed file-structure library."""
    text = manifest.read_text()
    match = re.search(r"^\[dependencies\](.*?)(?=^\[|\Z)", text, re.S | re.M)
    return bool(match and "lopdf" in match.group(1))


def sizes(root: Path) -> dict[str, int]:
    return {name: (root / name).stat().st_size for name in ARTIFACTS}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--base", required=True, help="the resolved base revision")
    parser.add_argument("--out", required=True)
    args = parser.parse_args()

    repo = Path(args.repo).resolve()
    out = (repo / args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)

    for name in ARTIFACTS:
        if not (repo / name).is_file():
            print(f"measure_browser_bundle: the landed bundle has no {name}", file=sys.stderr)
            return 1

    landed_sha = git(repo, "rev-parse", "HEAD")
    base_sha = git(repo, "rev-parse", args.base)
    landed_sizes = sizes(repo)

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
                "measure_browser_bundle: the restored tree differs outside the allowed paths: "
                + ", ".join(unexpected),
                file=sys.stderr,
            )
            return 1

        report = {
            "status": "not applicable",
            "landed_revision": landed_sha,
            "base_revision": base_sha,
            "landed_sizes": landed_sizes,
            "rustc": run(["rustc", "-V"], repo).stdout.strip(),
            "landed_lock_sha256": hashlib.sha256((repo / "Cargo.lock").read_bytes()).hexdigest(),
            "base_lock_sha256": hashlib.sha256(
                (worktree / "Cargo.lock").read_bytes()
            ).hexdigest(),
        }

        if names_previous_layer(worktree / "crates/duty2cal-pdf/Cargo.toml"):
            build = run(["bash", "scripts/build-web.sh"], worktree, check=False)
            if build.returncode != 0:
                print(build.stdout, build.stderr, file=sys.stderr)
                print("measure_browser_bundle: the restored tree does not build", file=sys.stderr)
                return 1
            base_sizes = sizes(worktree)
            if landed_sizes[ARTIFACTS[0]] >= base_sizes[ARTIFACTS[0]]:
                print(
                    "measure_browser_bundle: the landed WASM module is not smaller "
                    f"({landed_sizes[ARTIFACTS[0]]} B vs {base_sizes[ARTIFACTS[0]]} B)",
                    file=sys.stderr,
                )
                return 1
            report.update(status="measured", base_sizes=base_sizes)
            report["saved_bytes"] = {
                name: base_sizes[name] - landed_sizes[name] for name in ARTIFACTS
            }

        (out / "browser-bundle.json").write_text(json.dumps(report, indent=2) + "\n")
        markdown = [
            "# Browser bundle measurement",
            "",
            f"- status: {report['status']}",
            f"- landed revision: `{landed_sha}`",
            f"- base revision: `{base_sha}`",
        ]
        for name in ARTIFACTS:
            markdown.append(f"- landed {name}: {landed_sizes[name]} B")
            if report["status"] == "measured":
                markdown.append(f"- base {name}: {report['base_sizes'][name]} B")
                markdown.append(f"- saved {name}: {report['saved_bytes'][name]} B")
        markdown.append("")
        (out / "browser-bundle.md").write_text("\n".join(markdown))
        print("\n".join(markdown))
        return 0
    finally:
        run(["git", "worktree", "remove", "--force", str(worktree)], repo, check=False)


if __name__ == "__main__":
    raise SystemExit(main())
