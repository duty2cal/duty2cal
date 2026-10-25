#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
"""Serve a hosting target's tree on loopback for the site scenarios.

The server reads each requested path from the target repository's branch tip,
so a publication's push is visible to the publication's own serving check the
moment it lands: it serves the bytes the target's tree holds, which is what a
pages host serves from a branch. `--earlier` additionally pins a directory
under `/earlier/`, so a scenario can prove an address that keeps serving the
page built from an earlier revision.

Usage: site_server.py --repo BARE_GIT_DIR [--branch main] [--earlier DIR]
Prints the loopback port on its first stdout line.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import urllib.parse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

CONTENT_TYPES = {
    ".css": "text/css; charset=utf-8",
    ".html": "text/html; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".json": "application/json; charset=utf-8",
    ".png": "image/png",
    ".txt": "text/plain; charset=utf-8",
    ".wasm": "application/wasm",
}


def read_from_tree(repo: str, branch: str, relative: str) -> bytes | None:
    """Return the blob at `branch:relative`, or None when it does not exist."""
    result = subprocess.run(
        ["git", f"--git-dir={repo}", "show", f"{branch}:{relative}"],
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        return None
    return result.stdout


def read_from_directory(root: Path, relative: str) -> bytes | None:
    """Return the file at `root/relative`, refusing any path outside root."""
    candidate = (root / relative).resolve()
    if candidate != root and root not in candidate.parents:
        return None
    if not candidate.is_file():
        return None
    return candidate.read_bytes()


class Handler(BaseHTTPRequestHandler):
    repo = ""
    branch = "main"
    earlier: Path | None = None

    def log_message(self, *args: object) -> None:  # keep the scenario quiet
        pass

    def do_HEAD(self) -> None:  # noqa: N802 - http.server's spelling
        self.respond(body_only=True)

    def do_GET(self) -> None:  # noqa: N802 - http.server's spelling
        self.respond(body_only=False)

    def respond(self, body_only: bool) -> None:
        path = urllib.parse.urlparse(self.path).path
        path = urllib.parse.unquote(path)
        if path.endswith("/") or path == "":
            path += "index.html"
        relative = path.lstrip("/")

        if relative.startswith("earlier/"):
            data = (
                read_from_directory(self.earlier, relative[len("earlier/"):])
                if self.earlier is not None
                else None
            )
        else:
            data = read_from_tree(self.repo, self.branch, relative)

        if data is None:
            self.send_response(404)
            self.send_header("content-type", "text/plain; charset=utf-8")
            self.end_headers()
            if not body_only:
                self.wfile.write(b"not found")
            return

        content_type = CONTENT_TYPES.get(Path(relative).suffix, "application/octet-stream")
        self.send_response(200)
        self.send_header("content-type", content_type)
        self.send_header("content-length", str(len(data)))
        self.send_header("cache-control", "no-store")
        self.end_headers()
        if not body_only:
            self.wfile.write(data)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", required=True)
    parser.add_argument("--branch", default="main")
    parser.add_argument("--earlier", default=None)
    args = parser.parse_args()

    Handler.repo = args.repo
    Handler.branch = args.branch
    Handler.earlier = Path(args.earlier).resolve() if args.earlier else None

    server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    print(server.server_address[1], flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main())
