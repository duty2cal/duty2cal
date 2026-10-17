#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
"""A deterministic local twin of the GitHub API (design.md "Test seam").

The real adoption command and the real ``tofu`` binary run unchanged while the
provider's ``base_url`` points at this server, so every request stays on
loopback.

Usage:
    serve.py --fixtures DIR [--port 0] [--journal FILE]

The server prints the bound port on stdout and then serves the fixture tree:
a REST request for ``/repos/o/r`` reads ``DIR/repos/o/r.json``; a GraphQL query
POST reads ``DIR/graphql/<operation>.json``. A path with no fixture returns a
distinctive 599 with the ``X-Fake-Unknown-Path`` header and is journaled as an
unknown-path entry, so a missing fixture fails loudly instead of looking like a
real API response. A GraphQL mutation and every non-GET, non-GraphQL-query
request are refused with 405 and journaled as a refused write.

``DIR/faults.json`` injects a status for a matching path; ``DIR/mutate.json``
replaces a response body after an explicit request count or when a trigger file
appears. The journal is JSON Lines with the method, path, status, the
unknown-path and refused-write flags and the GraphQL operation name.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import unquote, urlsplit


class FakeState:
    """Shared request counters, journal and configuration for one server."""

    def __init__(self, fixtures: str, journal: str | None) -> None:
        self.fixtures = os.path.abspath(fixtures)
        self.journal = os.path.abspath(journal) if journal else None
        self.lock = threading.Lock()
        self.counts: dict[str, int] = {}
        self.faults = self._load_json("faults.json") or []
        self.mutations = self._load_json("mutate.json") or {}

        if self.journal:
            parent = os.path.dirname(self.journal)
            if parent:
                os.makedirs(parent, exist_ok=True)
            open(self.journal, "a", encoding="utf-8").close()

    def _load_json(self, name: str):
        path = os.path.join(self.fixtures, name)
        if not os.path.isfile(path):
            return None
        with open(path, encoding="utf-8") as handle:
            return json.load(handle)

    def record(self, entry: dict) -> None:
        with self.lock:
            if not self.journal:
                return
            with open(self.journal, "a", encoding="utf-8") as handle:
                handle.write(json.dumps(entry, sort_keys=True) + "\n")
                handle.flush()

    def count(self, key: str) -> int:
        with self.lock:
            self.counts[key] = self.counts.get(key, 0) + 1
            return self.counts[key]

    def fixture(self, relative: str):
        """The JSON body at ``relative`` (without ``.json``), or ``None``."""
        candidate = os.path.normpath(relative).lstrip(os.sep)
        if candidate.startswith("..") or os.path.isabs(candidate):
            return None
        for path in (
            os.path.join(self.fixtures, candidate + ".json"),
            os.path.join(self.fixtures, candidate, "index.json"),
        ):
            if os.path.isfile(path):
                with open(path, encoding="utf-8") as handle:
                    return json.load(handle)
        return None


def graphql_operation(payload: dict) -> str | None:
    """The operation name of a GraphQL request, when it can be determined."""
    name = payload.get("operationName")
    if isinstance(name, str) and name:
        return name
    query = payload.get("query")
    if not isinstance(query, str):
        return None
    match = re.search(
        r"\b(?:query|mutation|subscription)\s+([A-Za-z_][A-Za-z0-9_]*)", query
    )
    if match:
        return match.group(1)
    if query.lstrip().startswith("{"):
        return "query"
    return None


def graphql_is_mutation(payload: dict) -> bool:
    query = payload.get("query")
    if not isinstance(query, str):
        return False
    return re.match(r"\s*mutation\b", query) is not None


def request_path(raw_path: str) -> str:
    """The fixture key for a request path.

    The provider treats a configured ``base_url`` as a GitHub Enterprise
    endpoint and prefixes every request with ``/api/v3``. The fixture tree is
    keyed by the API path the design names (``repos/o/r``), so that prefix is
    removed here; the journal records the same normalized key.
    """
    path = unquote(urlsplit(raw_path).path)
    if path == "/api/v3":
        return "/"
    if path.startswith("/api/v3/"):
        return path[len("/api/v3") :]
    return path


class Handler(BaseHTTPRequestHandler):
    """Serve one fixture tree; the state arrives through the server object."""

    server_version = "fake-github/1"
    protocol_version = "HTTP/1.1"

    @property
    def state(self) -> FakeState:
        return self.server.fake  # type: ignore[attr-defined]

    def log_message(self, fmt: str, *args) -> None:  # noqa: A003
        # The runner owns the transcript; the server never writes to it.
        return

    # -- response helpers -------------------------------------------------

    def respond(self, status: int, body, unknown_path: bool = False) -> None:
        payload = b""
        if body is not None:
            payload = json.dumps(body).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        if unknown_path:
            self.send_header("X-Fake-Unknown-Path", "true")
        self.end_headers()
        if payload:
            self.wfile.write(payload)

    def journal(self, status: int, *, unknown_path=False, refused_write=False,
                operation=None) -> None:
        self.state.record(
            {
                "method": self.command,
                "path": request_path(self.path),
                "status": status,
                "unknown_path": unknown_path,
                "refused_write": refused_write,
                "graphql_operation": operation,
            }
        )

    def unknown(self) -> None:
        self.respond(
            599,
            {"message": f"no fixture for {self.command} {request_path(self.path)}"},
            unknown_path=True,
        )
        self.journal(599, unknown_path=True)

    # -- fault and mutation handling -------------------------------------

    def fault_for(self, path: str):
        for entry in self.state.faults:
            pattern = entry.get("path_regex", entry.get("path"))
            method = entry.get("method")
            if method and method.upper() != self.command:
                continue
            if pattern is None:
                continue
            try:
                matched = re.search(pattern, path) is not None
            except re.error:
                matched = path == pattern
            if matched:
                return entry
        return None

    def mutation_for(self, path: str):
        entry = self.state.mutations.get(path)
        if entry is None:
            return None
        trigger = entry.get("trigger_file")
        if trigger and os.path.isfile(os.path.join(self.state.fixtures, trigger)):
            return entry.get("body")
        after = entry.get("after_requests")
        if isinstance(after, int) and self.state.count(path) > after:
            return entry.get("body")
        return None

    # -- request handling -------------------------------------------------

    def do_GET(self) -> None:  # noqa: N802
        path = unquote(request_path(self.path))
        fault = self.fault_for(path)
        if fault is not None:
            status = int(fault.get("status", 500))
            self.respond(status, fault.get("body", {"message": "injected fault"}))
            self.journal(status)
            return

        mutated = self.mutation_for(path)
        if mutated is not None:
            self.respond(200, mutated)
            self.journal(200)
            return

        body = self.state.fixture(path)
        if body is None:
            self.unknown()
            return
        self.respond(200, body)
        self.journal(200)

    def do_POST(self) -> None:  # noqa: N802
        path = unquote(request_path(self.path))
        length = int(self.headers.get("Content-Length", "0") or "0")
        raw = self.rfile.read(length) if length else b""
        try:
            payload = json.loads(raw.decode("utf-8")) if raw else {}
        except (UnicodeDecodeError, json.JSONDecodeError):
            payload = {}

        if path != "/graphql":
            self.respond(405, {"message": f"refused write: {self.command} {path}"})
            self.journal(405, refused_write=True)
            return

        operation = graphql_operation(payload)
        if graphql_is_mutation(payload):
            self.respond(405, {"message": f"refused GraphQL mutation: {operation}"})
            self.journal(405, refused_write=True, operation=operation)
            return

        fault = self.fault_for(path)
        if fault is not None:
            status = int(fault.get("status", 500))
            self.respond(status, fault.get("body", {"message": "injected fault"}))
            self.journal(status, operation=operation)
            return

        body = None
        if operation:
            body = self.state.fixture(os.path.join("graphql", operation))
        if body is None:
            body = self.state.fixture(os.path.join("graphql", "query"))
        if body is None:
            self.unknown()
            return
        self.respond(200, body)
        self.journal(200, operation=operation)

    def do_PUT(self) -> None:  # noqa: N802
        self.refused_method()

    def do_PATCH(self) -> None:  # noqa: N802
        self.refused_method()

    def do_DELETE(self) -> None:  # noqa: N802
        self.refused_method()

    def do_HEAD(self) -> None:  # noqa: N802
        self.refused_method()

    def refused_method(self) -> None:
        path = request_path(self.path)
        self.respond(405, {"message": f"refused write: {self.command} {path}"})
        self.journal(405, refused_write=True)


def main() -> int:
    parser = argparse.ArgumentParser(description="fake GitHub API for the suite")
    parser.add_argument("--fixtures", required=True)
    parser.add_argument("--port", type=int, default=0)
    parser.add_argument("--journal")
    args = parser.parse_args()

    if not os.path.isdir(args.fixtures):
        print(f"no fixtures directory: {args.fixtures}", file=sys.stderr)
        return 2

    fake = FakeState(args.fixtures, args.journal)
    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    server.daemon_threads = True
    server.fake = fake  # type: ignore[attr-defined]

    print(server.server_address[1], flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
