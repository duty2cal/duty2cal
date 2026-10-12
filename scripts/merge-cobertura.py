#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Merge two Cobertura XML reports into one (hit counts summed per line).

The merged report is built from scratch: mutating one of the inputs would keep
its other package elements and duplicate the classes.
"""
import sys
import xml.etree.ElementTree as ET


def covers(path):
    """filename -> {line_number: hits}"""
    out = {}
    for cls in ET.parse(path).getroot().iter("class"):
        key = cls.get("filename") or cls.get("name") or ""
        lines = out.setdefault(key, {})
        # Only the class's own line block: `iter` would also descend into the
        # method blocks and count the same line twice.
        block = cls.find("lines")
        if block is None:
            continue
        for line in block.findall("line"):
            number = int(line.get("number", "0"))
            hits = int(line.get("hits", "0"))
            lines[number] = lines.get(number, 0) + hits
    return out


def main(argv):
    if len(argv) != 4:
        print("usage: merge-cobertura.py <a.xml> <b.xml> <out.xml>", file=sys.stderr)
        return 2
    first, second, out_path = argv[1], argv[2], argv[3]

    merged = covers(first)
    for filename, lines in covers(second).items():
        target = merged.setdefault(filename, {})
        for number, hits in lines.items():
            target[number] = target.get(number, 0) + hits

    coverage = ET.Element(
        "coverage",
        {"branch-rate": "0", "complexity": "0", "line-rate": "0", "version": "2.0.3"},
    )
    sources = ET.SubElement(coverage, "sources")
    ET.SubElement(sources, "source").text = "merged"
    packages = ET.SubElement(coverage, "packages")
    package = ET.SubElement(packages, "package", {"name": "merged", "line-rate": "0"})
    classes = ET.SubElement(package, "classes")

    covered = valid = 0
    for filename in sorted(merged):
        cls = ET.SubElement(classes, "class", {"name": filename, "filename": filename})
        lines_el = ET.SubElement(cls, "lines")
        file_covered = 0
        for number in sorted(merged[filename]):
            hits = merged[filename][number]
            ET.SubElement(lines_el, "line", {"number": str(number), "hits": str(hits)})
            if hits > 0:
                file_covered += 1
        count = len(merged[filename])
        cls.set("line-rate", f"{file_covered / count:.6f}" if count else "0")
        cls.set("branch-rate", "0")
        covered += file_covered
        valid += count

    rate = f"{covered / valid:.6f}" if valid else "0"
    for element in (coverage, package):
        element.set("line-rate", rate)
        element.set("lines-covered", str(covered))
        element.set("lines-valid", str(valid))

    ET.ElementTree(coverage).write(out_path, encoding="utf-8", xml_declaration=True)
    print(f"merged {len(merged)} files, {valid} lines ({covered} covered) -> {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
