// SPDX-License-Identifier: MPL-2.0
//! Filesystem adapter (native only): publication of calendars and numbered
//! diagnostics beside the source. Only the CLI needs paths; WASM/UniFFI pass
//! bytes.
//!
//! The write itself creates the file (`create_new`), so an existing calendar or
//! report is never replaced, not even by a racing run: a name that turns out to
//! be taken between the check and the create fails the publication, which is
//! reported rather than hidden behind a later name.

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// How many enumerated candidates are tried before giving up.
const MAX_ENUMERATION: u32 = 9_999;

/// Write `body` beside `source` under the first name that does not exist.
///
/// The calendar names are `<stem>.ics`, `<stem>-1.ics`, …; the diagnostic names
/// are `<stem>.parse-error.txt`, `<stem>.parse-error-1.txt`, …. Returns the
/// written path, or `None` when every enumerated name is taken.
pub fn publish(source: &Path, kind: Kind, body: &str) -> io::Result<Option<PathBuf>> {
    let Some(stem) = source.file_stem().and_then(|stem| stem.to_str()) else {
        return Ok(None);
    };
    let dir = source.parent().unwrap_or_else(|| Path::new("."));

    for candidate in candidates(dir, stem, kind) {
        // An occupied name is enumerated past; a name that only looks free is
        // attempted, and the attempt's failure is reported rather than hidden
        // by silently taking a later one.
        if candidate.exists() {
            continue;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)?;
        file.write_all(body.as_bytes())?;
        return Ok(Some(candidate));
    }
    Ok(None)
}

/// What a publication writes, which fixes its names.
#[derive(Debug, Clone, Copy)]
pub enum Kind {
    /// The converted calendar: `<stem>.ics`, `<stem>-1.ics`, ….
    Calendar,
    /// The failure report: `<stem>.parse-error.txt`, `<stem>.parse-error-1.txt`, ….
    Diagnostic,
}

impl Kind {
    /// The part between the source's stem and the extension, and the extension.
    fn spelling(self) -> (&'static str, &'static str) {
        match self {
            Self::Calendar => ("", "ics"),
            Self::Diagnostic => (".parse-error", "txt"),
        }
    }
}

/// Every candidate name for one kind, plain first, then enumerated.
fn candidates<'a>(dir: &'a Path, stem: &'a str, kind: Kind) -> impl Iterator<Item = PathBuf> + 'a {
    let (middle, extension) = kind.spelling();
    (0..=MAX_ENUMERATION).map(move |number| {
        let suffix = if number == 0 {
            String::new()
        } else {
            format!("-{number}")
        };
        dir.join(format!("{stem}{middle}{suffix}.{extension}"))
    })
}
