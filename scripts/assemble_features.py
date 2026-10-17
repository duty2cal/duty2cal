#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Assemble the permanent feature corpus with the active changes' deltas.

`scripts/assemble-features.sh` owns the temporary output directory and prints
it; this helper performs the Gherkin-aware merge into `--out`:

* every permanent feature under `givn/specs/**` is copied after removing the
  scenario blocks whose exact titles a delta tags `@givn.modified` or
  `@givn.removed` for that same capability;
* every delta feature under `givn/changes/*/specs/**` is copied without its
  `@givn.removed` placeholder blocks;
* an unmatched, duplicate or ambiguously claimed replacement title is a loud
  failure naming the title and the file.

Only tags and scenario blocks are interpreted. Scenario shape is `givn lint`'s
job. The permanent files are never rewritten.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

SCENARIO_HEADER = re.compile(r"^\s*(?:Scenario Outline|Scenario):(.*)$")
TAG_LINE = re.compile(r"^\s*@\S")


@dataclass(frozen=True)
class Block:
    """One scenario, including the tag lines immediately above it."""

    title: str
    tags: frozenset[str]
    start: int
    end: int


def scenario_blocks(lines: list[str]) -> list[Block]:
    """Split a feature's lines into scenario blocks.

    A block begins at the contiguous tag lines directly above a Scenario or
    Scenario Outline header and ends at the next block's first line, or EOF.
    Doc-string fences are tracked so text inside them is never a header.
    """
    headers: list[int] = []
    fence: str | None = None
    for number, line in enumerate(lines):
        stripped = line.lstrip()
        if fence is not None:
            if stripped.startswith(fence):
                fence = None
            continue
        if stripped.startswith('"""') or stripped.startswith("```"):
            fence = '"""' if stripped.startswith('"""') else "```"
            # A fence opened and closed on the same line carries no block.
            if stripped.count(fence) >= 2:
                fence = None
            continue
        if SCENARIO_HEADER.match(line):
            headers.append(number)

    blocks: list[Block] = []
    for index, header in enumerate(headers):
        start = header
        above = header - 1
        while above >= 0 and TAG_LINE.match(lines[above]):
            start = above
            above -= 1
        end = len(lines)
        if index + 1 < len(headers):
            next_header = headers[index + 1]
            next_start = next_header
            above = next_header - 1
            while above >= 0 and TAG_LINE.match(lines[above]):
                next_start = above
                above -= 1
            end = next_start
        match = SCENARIO_HEADER.match(lines[header])
        assert match is not None
        tags: set[str] = set()
        for tag_line in lines[start:header]:
            if TAG_LINE.match(tag_line):
                tags.update(tag_line.split())
        blocks.append(
            Block(
                title=match.group(1).strip(),
                tags=frozenset(tags),
                start=start,
                end=end,
            )
        )
    return blocks


def feature_prefix(lines: list[str], blocks: list[Block]) -> list[str]:
    """The feature-level tag/Feature/description lines before the first block."""
    return lines[: blocks[0].start] if blocks else lines


def filtered_text(lines: list[str], blocks: list[Block], retained: list[Block]) -> str:
    """The prefix plus the retained blocks, byte for byte."""
    parts = feature_prefix(lines, blocks)
    for block in retained:
        parts.extend(lines[block.start : block.end])
    return "".join(parts)


def blocks_by_title(blocks: list[Block]) -> dict[str, list[Block]]:
    found: dict[str, list[Block]] = {}
    for block in blocks:
        found.setdefault(block.title, []).append(block)
    return found


def fail(message: str) -> None:
    print(f"assemble_features: {message}", file=sys.stderr)
    raise SystemExit(1)


def features_under(path: Path) -> list[Path]:
    if not path.is_dir():
        return []
    return sorted(candidate for candidate in path.rglob("*.feature") if candidate.is_file())


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="repository root")
    parser.add_argument("--out", required=True, help="directory to write the assembled features into")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)

    permanent_root = root / "givn" / "specs"
    changes_root = root / "givn" / "changes"

    permanent: dict[str, Path] = {
        path.relative_to(permanent_root).as_posix(): path
        for path in features_under(permanent_root)
    }

    delta_files: list[tuple[Path, str]] = []
    replacements: dict[str, dict[str, str]] = {}
    if changes_root.is_dir():
        for change in sorted(changes_root.iterdir()):
            specs = change / "specs"
            if not change.is_dir() or not specs.is_dir():
                continue
            for path in features_under(specs):
                key = path.relative_to(specs).as_posix()
                delta_files.append((path, key))
                lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
                for block in scenario_blocks(lines):
                    replacing = block.tags & {"@givn.modified", "@givn.removed"}
                    if not replacing:
                        continue
                    if key not in permanent:
                        fail(
                            f"{path}: {block.title!r} is tagged "
                            f"{' '.join(sorted(replacing))} but no permanent "
                            f"feature exists at givn/specs/{key}"
                        )
                    claims = replacements.setdefault(key, {})
                    if block.title in claims:
                        fail(
                            f"{path}: {block.title!r} is already claimed by "
                            f"{claims[block.title]} for givn/specs/{key}"
                        )
                    claims[block.title] = str(path)

    # Emit every permanent feature with the superseded blocks removed.
    sources: list[tuple[str, str]] = []
    for key, path in sorted(permanent.items()):
        lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
        blocks = scenario_blocks(lines)
        by_title = blocks_by_title(blocks)
        remove = replacements.get(key, {})
        for title, claimant in remove.items():
            found = by_title.get(title, [])
            if not found:
                fail(f"{claimant}: {title!r} is not found in givn/specs/{key}")
            if len(found) > 1:
                fail(f"givn/specs/{key}: {title!r} occurs {len(found)} times; cannot remove it unambiguously")
        retained = [block for block in blocks if block.title not in remove]
        sources.append((str(path), filtered_text(lines, blocks, retained)))

    # Emit every delta without its retired placeholder scenarios.
    for path, _key in sorted(delta_files):
        lines = path.read_text(encoding="utf-8").splitlines(keepends=True)
        blocks = scenario_blocks(lines)
        retained = [block for block in blocks if "@givn.removed" not in block.tags]
        sources.append((str(path), filtered_text(lines, blocks, retained)))

    for number, (source, text) in enumerate(sources, start=1):
        destination = out / f"f{number:04d}.feature"
        destination.write_text(text, encoding="utf-8")
        print(f"assemble_features: {source} -> {destination.name}", file=sys.stderr)


if __name__ == "__main__":
    main()
