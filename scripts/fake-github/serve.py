#!/usr/bin/env python3
# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
"""A deterministic local twin of the GitHub API (design.md "Test seam").

The real adoption command and the real ``tofu`` binary run unchanged while the
provider's ``base_url`` points at this server, so every request stays on
loopback.

Usage:
    serve.py --fixtures DIR [--port 0] [--journal FILE] [--writes]

The server prints the bound port on stdout and then serves the fixture tree:
a REST request for ``/repos/o/r`` reads ``DIR/repos/o/r.json``; a GraphQL query
POST reads ``DIR/graphql/<operation>.json``. A path with no fixture returns a
distinctive 599 with the ``X-Fake-Unknown-Path`` header and is journaled as an
unknown-path entry, so a missing fixture fails loudly instead of looking like a
real API response. A GraphQL mutation and every non-GET, non-GraphQL-query
request are refused with 405 and journaled as a refused write.

``--writes`` opts into the test-only write sandbox (design.md "Write sandbox"):
a repository ``PATCH``, and the ``PUT``/``DELETE`` pairs for the vulnerability
alerts and automated security fixes endpoints, update an in-memory overlay that
reads consult before the fixture file. Every other write is still refused with
405 and journaled.

A fixture body of ``{"__fake_response__": {"status": 204}}`` is a response
descriptor: the server replies with that status and no body.

``DIR/faults.json`` injects a status for a matching path, optionally only after
a number of applied writes and optionally once; ``DIR/mutate.json`` replaces a
response body after an explicit request count or when a trigger file appears.
The journal is JSON Lines with the method, path, status, the unknown-path and
refused-write flags, the GraphQL operation name, and the applied-write or
manual-edit recording (sorted body keys and the security block for a repository
``PATCH``).
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

    def __init__(self, fixtures: str, journal: str | None, writes: bool = False) -> None:
        self.fixtures = os.path.abspath(fixtures)
        self.journal = os.path.abspath(journal) if journal else None
        self.lock = threading.Lock()
        self.writes = writes
        self.counts: dict[str, int] = {}
        # The write sandbox's in-memory overlay: a merged repository body, the
        # alert flag and the security-updates flag per repository path.
        self.repos: dict[str, dict] = {}
        self.alerts: dict[str, bool] = {}
        self.security_updates: dict[str, bool] = {}
        self.applied_writes = 0
        self.fired_faults: set[int] = set()
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

    def overlay(self, path: str):
        """The in-memory write overlay's response for ``path``, if any."""
        repository = repository_path(path)
        if repository is None:
            return None
        if path == repository:
            return self.repos.get(repository)
        suffix = path[len(repository) :]
        if suffix == "/vulnerability-alerts" and repository in self.alerts:
            if self.alerts[repository]:
                return {"__fake_response__": {"status": 204}}
            return {"__fake_response__": {"status": 404}}
        if suffix == "/automated-security-fixes" and repository in self.security_updates:
            return {
                "enabled": self.security_updates[repository],
                "paused": False,
            }
        return None

    def apply_repository_patch(self, path: str, body: dict) -> dict:
        """Merge a repository ``PATCH`` body into the overlay and return it."""
        with self.lock:
            current = self.repos.get(path)
            if current is None:
                current = self.fixture(path) or {}
            merged = dict(current)
            for key, value in body.items():
                if key == "security_and_analysis" and isinstance(value, dict):
                    nested = dict(merged.get(key) or {})
                    for setting, setting_value in value.items():
                        if isinstance(setting_value, dict):
                            previous = dict(nested.get(setting) or {})
                            previous.update(setting_value)
                            nested[setting] = previous
                        else:
                            nested[setting] = setting_value
                    merged[key] = nested
                else:
                    merged[key] = value
            self.repos[path] = merged
            security = merged.get("security_and_analysis") or {}
            updates = security.get("dependabot_security_updates") or {}
            if updates.get("status") in ("enabled", "disabled"):
                self.security_updates[path] = updates["status"] == "enabled"
            self.applied_writes += 1
            return merged

    def set_overlay(self, path: str, kind: str, enabled: bool) -> None:
        """Record a PUT/DELETE write for the two settings endpoints."""
        with self.lock:
            if kind == "vulnerability-alerts":
                self.alerts[path] = enabled
            else:
                self.security_updates[path] = enabled
                repository = self.repos.get(path)
                if repository is not None:
                    security = dict(repository.get("security_and_analysis") or {})
                    security["dependabot_security_updates"] = {
                        "status": "enabled" if enabled else "disabled"
                    }
                    repository["security_and_analysis"] = security
            self.applied_writes += 1


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


def repository_path(path: str) -> str | None:
    """The repository key of ``/repos/{owner}/{repo}[/...]``, or ``None``."""
    match = re.match(r"^(/repos/[^/]+/[^/]+)(?:/.*)?$", path)
    return match.group(1) if match else None


def response_descriptor(body):
    """The status of a ``__fake_response__`` descriptor, or ``None``."""
    if not isinstance(body, dict):
        return None
    descriptor = body.get("__fake_response__")
    if not isinstance(descriptor, dict):
        return None
    return int(descriptor.get("status", 200))


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
                operation=None, mutated=False, applied_write=False,
                manual_edit=False, body=None) -> None:
        entry = {
            "method": self.command,
            "path": request_path(self.path),
            "status": status,
            "unknown_path": unknown_path,
            "refused_write": refused_write,
            "graphql_operation": operation,
            "mutated": mutated,
        }
        if applied_write or manual_edit:
            entry["applied_write"] = applied_write
            if manual_edit:
                entry["manual_edit"] = True
            if isinstance(body, dict):
                entry["body_keys"] = sorted(body)
                security = body.get("security_and_analysis")
                if isinstance(security, dict):
                    entry["security_and_analysis"] = security
        self.state.record(entry)

    def unknown(self) -> None:
        self.respond(
            599,
            {"message": f"no fixture for {self.command} {request_path(self.path)}"},
            unknown_path=True,
        )
        self.journal(599, unknown_path=True)

    # -- fault and mutation handling -------------------------------------

    def fault_for(self, path: str):
        with self.state.lock:
            for index, entry in enumerate(self.state.faults):
                pattern = entry.get("path_regex", entry.get("path"))
                method = entry.get("method")
                if method and method.upper() != self.command:
                    continue
                if pattern is None:
                    continue
                after = entry.get("after_applied_writes")
                if isinstance(after, int) and self.state.applied_writes < after:
                    continue
                if entry.get("once") and index in self.state.fired_faults:
                    continue
                try:
                    matched = re.search(pattern, path) is not None
                except re.error:
                    matched = path == pattern
                if matched:
                    if entry.get("once"):
                        self.state.fired_faults.add(index)
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

        body = self.state.overlay(path)
        if body is not None:
            status = response_descriptor(body)
            if status is not None:
                self.respond(status, None)
                self.journal(status)
                return
            self.respond(200, body)
            self.journal(200)
            return

        mutated = self.mutation_for(path)
        if mutated is not None:
            self.respond(200, mutated)
            self.journal(200, mutated=True)
            return

        body = self.state.fixture(path)
        if body is None:
            self.unknown()
            return
        status = response_descriptor(body)
        if status is not None:
            self.respond(status, None)
            self.journal(status)
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
        status = response_descriptor(body)
        if status is not None:
            self.respond(status, None)
            self.journal(status, operation=operation)
            return
        self.respond(200, body)
        self.journal(200, operation=operation)

    def read_body(self) -> dict | None:
        length = int(self.headers.get("Content-Length", "0") or "0")
        raw = self.rfile.read(length) if length else b""
        if not raw:
            return {}
        try:
            body = json.loads(raw.decode("utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            return None
        return body if isinstance(body, dict) else None

    def settings_write(self, kind: str, enabled: bool) -> None:
        path = unquote(request_path(self.path))
        repository = repository_path(path)
        if (
            not self.state.writes
            or repository is None
            or path != repository + f"/{kind}"
        ):
            self.refused_method()
            return
        self.state.set_overlay(repository, kind, enabled)
        self.respond(204, None)
        self.journal(204, applied_write=True)

    def do_PUT(self) -> None:  # noqa: N802
        path = unquote(request_path(self.path))
        fault = self.fault_for(path)
        if fault is not None:
            status = int(fault.get("status", 500))
            self.respond(status, fault.get("body", {"message": "injected fault"}))
            self.journal(status)
            return
        self.read_body()
        if path.endswith("/vulnerability-alerts"):
            self.settings_write("vulnerability-alerts", True)
            return
        if path.endswith("/automated-security-fixes"):
            self.settings_write("automated-security-fixes", True)
            return
        self.refused_method()

    def do_PATCH(self) -> None:  # noqa: N802
        path = unquote(request_path(self.path))
        fault = self.fault_for(path)
        if fault is not None:
            status = int(fault.get("status", 500))
            self.respond(status, fault.get("body", {"message": "injected fault"}))
            self.journal(status)
            return
        body = self.read_body()
        if (
            not self.state.writes
            or repository_path(path) != path
            or body is None
        ):
            self.refused_method()
            return
        merged = self.state.apply_repository_patch(repository_path(path), body)
        manual = self.headers.get("X-Fake-Manual-Edit", "").lower() == "true"
        self.respond(200, merged)
        self.journal(
            200,
            applied_write=not manual,
            manual_edit=manual,
            body=body,
        )

    def do_DELETE(self) -> None:  # noqa: N802
        path = unquote(request_path(self.path))
        fault = self.fault_for(path)
        if fault is not None:
            status = int(fault.get("status", 500))
            self.respond(status, fault.get("body", {"message": "injected fault"}))
            self.journal(status)
            return
        if path.endswith("/vulnerability-alerts"):
            self.settings_write("vulnerability-alerts", False)
            return
        if path.endswith("/automated-security-fixes"):
            self.settings_write("automated-security-fixes", False)
            return
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
    parser.add_argument("--writes", action="store_true")
    args = parser.parse_args()

    if not os.path.isdir(args.fixtures):
        print(f"no fixtures directory: {args.fixtures}", file=sys.stderr)
        return 2

    fake = FakeState(args.fixtures, args.journal, writes=args.writes)
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
