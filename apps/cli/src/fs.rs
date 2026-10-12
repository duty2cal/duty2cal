// SPDX-License-Identifier: MPL-2.0
//! Filesystem adapter (native only): unused-path allocation for calendars and
//! numbered diagnostics. Only the CLI needs paths; WASM/UniFFI pass bytes.

use std::path::{Path, PathBuf};

/// How many enumerated candidates are tried before giving up.
const MAX_ENUMERATION: u32 = 9_999;

/// An unused calendar path beside the source: `<stem>.ics`, then
/// `<stem>-1.ics`, ... Never returns an existing path (design: no clobber).
pub fn unused_calendar_path(source: &Path) -> Option<PathBuf> {
    let stem = source.file_stem()?.to_str()?;
    let dir = source.parent().unwrap_or_else(|| Path::new("."));

    let ordinary = dir.join(format!("{stem}.ics"));
    if !ordinary.exists() {
        return Some(ordinary);
    }
    for n in 1..=MAX_ENUMERATION {
        let candidate = dir.join(format!("{stem}-{n}.ics"));
        if !candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// An unused numbered diagnostic path beside the source:
/// `<stem>.parse-error.txt`, then `<stem>.parse-error-1.txt`, ... Never
/// returns an existing path, so an older report is never overwritten.
pub fn unused_diagnostic_path(source: &Path) -> Option<PathBuf> {
    let stem = source.file_stem()?.to_str()?;
    let dir = source.parent().unwrap_or_else(|| Path::new("."));

    let ordinary = dir.join(format!("{stem}.parse-error.txt"));
    if !ordinary.exists() {
        return Some(ordinary);
    }
    for n in 1..=MAX_ENUMERATION {
        let candidate = dir.join(format!("{stem}.parse-error-{n}.txt"));
        if !candidate.exists() {
            return Some(candidate);
        }
    }
    None
}
