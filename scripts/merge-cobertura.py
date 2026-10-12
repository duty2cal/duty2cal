#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Merge two Cobertura XML reports into one (hit counts summed per line)."""
import sys
import xml.etree.ElementTree as ET


def load(path):
    return ET.parse(path).getroot()


def covers(root):
    """filename -> {line_number: hits}"""
    out = {}
    for cls in root.iter("class"):
        key = cls.get("filename") or cls.get("name") or ""
        lines = out.setdefault(key, {})
        # Only the class's own line block: `iter` would also descend into the
        # method blocks and count the same line twice.
        block = cls.find("lines")
        if block is None:
            continue
        for line in block.findall("line"):
            num = int(line.get("number", "0"))
            hits = int(line.get("hits", "0"))
            lines[num] = lines.get(num, 0) + hits
    return out


def main(argv):
    if len(argv) != 4:
        print("usage: merge-cobertura.py <a.xml> <b.xml> <out.xml>", file=sys.stderr)
        return 2
    a, b, out_path = argv[1], argv[2], argv[3]
    root = load(a)
    merged = covers(root)
    for filename, lines in covers(load(b)).items():
        target = merged.setdefault(filename, {})
        for num, hits in lines.items():
            target[num] = target.get(num, 0) + hits

    packages = root.find("packages")
    if packages is None:
        packages = ET.SubElement(root, "packages")
    package = packages.find("package")
    if package is None:
        package = ET.SubElement(packages, "package", {"name": "merged"})
    classes = package.find("classes")
    if classes is None:
        classes = ET.SubElement(package, "classes")
    for cls in list(classes):
        classes.remove(cls)

    covered = valid = 0
    for filename in sorted(merged):
        cls = ET.SubElement(classes, "class", {"name": filename, "filename": filename})
        lines_el = ET.SubElement(cls, "lines")
        file_covered = 0
        for num in sorted(merged[filename]):
            hits = merged[filename][num]
            ET.SubElement(lines_el, "line", {"number": str(num), "hits": str(hits)})
            if hits > 0:
                file_covered += 1
        count = len(merged[filename])
        rate = (file_covered / count) if count else 0.0
        cls.set("line-rate", f"{rate:.6f}")
        cls.set("branch-rate", "0")
        covered += file_covered
        valid += count

    total_rate = (covered / valid) if valid else 0.0
    for el in (root, package):
        el.set("line-rate", f"{total_rate:.6f}")
        el.set("branch-rate", "0")
    root.set("lines-covered", str(covered))
    root.set("lines-valid", str(valid))
    package.set("lines-covered", str(covered))
    package.set("lines-valid", str(valid))

    ET.ElementTree(root).write(out_path, encoding="utf-8", xml_declaration=True)
    print(f"merged {valid} lines ({covered} covered) -> {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
