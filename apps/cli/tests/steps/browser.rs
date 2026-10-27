// SPDX-License-Identifier: MPL-2.0
//! Shared client for `web/e2e/driver.mjs` (design.md "Step definition
//! locations").
//!
//! The driver speaks one JSON command per line and answers with one JSON
//! result per line. This client owns the child process and fails closed: an
//! action outside the protocol, a driver that stops answering, a malformed
//! result or a `{ "ok": false }` answer all panic, so a step can never read an
//! unperformed action as success.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Map, Value};

/// The actions the driver protocol serves. An action outside this list is a
/// step bug, not a driver failure.
const ACTIONS: [&str; 13] = [
    "start",
    "navigate",
    "click",
    "read_text",
    "set_file",
    "drop_file",
    "read_state",
    "save_offer",
    "dismiss_save",
    "reload",
    "screenshot",
    "axe",
    "stop",
];

/// A live Playwright driver process for one scenario.
#[derive(Debug)]
pub struct BrowserDriver {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

#[allow(dead_code)] // consumed by the browser scenarios as they leave RED
impl BrowserDriver {
    /// Start one driver process for the current scenario.
    pub fn start() -> Self {
        Self::spawn(None)
    }

    /// Start one driver process whose loopback server serves `root`, named to
    /// the driver through `DUTY2CAL_SITE_ROOT`. The environment is set on the
    /// driver child only, so a concurrently running scenario's driver keeps
    /// serving its own tree.
    pub fn start_serving(root: &Path) -> Self {
        Self::spawn(Some(root))
    }

    fn spawn(root: Option<&Path>) -> Self {
        let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../web/e2e/driver.mjs");
        let mut command = Command::new("node");
        command
            .arg(&script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        if let Some(root) = root {
            command.env("DUTY2CAL_SITE_ROOT", root);
        }
        let mut child = command.spawn().unwrap_or_else(|error| {
            panic!("start the browser driver {}: {error}", script.display())
        });
        let stdin = child.stdin.take().expect("the driver owns a stdin pipe");
        let stdout = child.stdout.take().expect("the driver owns a stdout pipe");
        Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
        }
    }

    /// Send one command and return its result object, or panic on anything but
    /// an explicit success.
    pub fn send(&mut self, action: &str, params: Map<String, Value>) -> Value {
        assert!(
            ACTIONS.contains(&action),
            "unknown browser driver action {action:?}"
        );

        let mut command = params;
        command.insert("action".to_string(), Value::String(action.to_string()));
        writeln!(self.stdin, "{}", Value::Object(command))
            .unwrap_or_else(|error| panic!("write browser driver action {action:?}: {error}"));
        self.stdin
            .flush()
            .unwrap_or_else(|error| panic!("flush browser driver action {action:?}: {error}"));

        let mut line = String::new();
        let read = self
            .stdout
            .read_line(&mut line)
            .unwrap_or_else(|error| panic!("read browser driver result for {action:?}: {error}"));
        assert!(
            read > 0,
            "the browser driver exited before answering {action:?}"
        );

        let result: Value = serde_json::from_str(line.trim()).unwrap_or_else(|error| {
            panic!("malformed browser driver result for {action:?}: {error}: {line:?}")
        });
        assert!(
            result.get("ok") == Some(&Value::Bool(true)),
            "the browser driver failed {action:?}: {}",
            result
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("no error was reported")
        );
        result
    }

    /// Stop the driver and reap it (also done by `Drop`).
    pub fn stop(mut self) {
        let _ = self.send("stop", Map::new());
        let _ = self.child.wait();
    }
}

impl Drop for BrowserDriver {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
