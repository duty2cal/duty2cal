# DutyPlan2Ical — Rust CLI Converter

## Mission

Build a production-ready Rust CLI application called `dutyplan2ical` that converts
a Jeppesen crew duty-plan PDF into a RFC 5545-compliant `.ics` calendar file.
The user drops a PDF onto the `.exe` (Windows drag-and-drop = `argv[1]`). The tool
writes `<same-name>.ics` next to the PDF and exits. No GUI, no installer.

---

## Non-Negotiable Constraints

- **Language**: Rust (stable toolchain, edition 2021)
- **Target binary**: `x86_64-pc-windows-gnu` (cross-compiled on Linux/CI)
- **Single `.exe`**: no DLL, no runtime installer required on the target machine
- **All dependencies**: OSS with permissive licenses only (MIT or Apache-2.0).
  Run `cargo deny check licenses` in CI; fail on any non-permissive license.
- **PDF extraction**: `pdfplumber` crate (pure Rust, lopdf backend). Pin to exact
  version in `Cargo.toml`. No native C/C++ dependency allowed (rules out
  `pdfium-render` for this project).
- **ICS output**: hand-written RFC 5545 — no ical library dependency.
  Lines terminated with CRLF. Lines longer than 75 octets folded per RFC 5545 §3.1.
- **No real PII** anywhere in the repository. The test fixture is fully synthetic.
  The real duty-plan PDF must never be committed (enforced via `.gitignore` and CI guard).
- **Public GitHub repository** — everything committed must be releasable under
  MIT or Apache-2.0.

---

## Architecture

Follow Clean Architecture. Dependency arrows point inward toward the domain.
```
src/
  main.rs                         # Presentation: parse argv, call use case, handle errors
  domain/
    mod.rs                        # RosterEntry, EntryKind, RosterModel (no framework imports)
  application/
    mod.rs                        # convert() use case; RosterParser + CalendarWriter traits
  infrastructure/
    pdfplumber_parser.rs          # impl RosterParser  — pdfplumber crate
    ics_writer.rs                 # impl CalendarWriter — hand-written RFC 5545
tests/
  extraction_test.rs              # L1: raw extraction sanity
  domain_test.rs                  # L2: domain invariants
  golden_test.rs                  # L3: end-to-end against golden oracle
  fixtures/
    reference_roster.pdf          # synthetic Typst-generated PDF (see §Fixture below)
  golden/
    2099-01.ics                   # accepted golden oracle (see §Oracle below)
scripts/
  pii-guard.sh                    # pattern-based PII check, no literals
.cargo/
  config.toml                     # cross-compile linker config
.github/
  workflows/
    build.yml                     # CI: test + cross-build + release artefact
```

### Domain Model (`src/domain/mod.rs`)

```rust
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Clone, PartialEq)]
pub enum EntryKind {
    Flight,
    Free,
    Off,
    Standby,
    Em,
    Layover,
}

#[derive(Debug, Clone)]
pub struct RosterEntry {
    pub kind: EntryKind,
    pub date_lt: NaiveDate,
    pub trip_id: Option&lt;String&gt;,
    pub report_lt: Option&lt;String&gt;,       // &quot;HH:MM&quot;, stored as string
    pub activity_code: Option&lt;String&gt;,   // &quot;ZZ100&quot;, &quot;FREE&quot;, &quot;EM&quot;, etc.
    pub from: Option&lt;String&gt;,            // IATA or synthetic 3-letter code
    pub to: Option&lt;String&gt;,
    pub start_utc: Option&lt;DateTime&lt;Utc&gt;&gt;,
    pub end_utc: Option&lt;DateTime&lt;Utc&gt;&gt;,
    pub aircraft: Option&lt;String&gt;,
    pub layover: Option&lt;String&gt;,         // &quot;51:30&quot;
    pub timezone_diff: Option&lt;String&gt;,   // &quot;+01:00&quot;
    pub blocktime: Option&lt;String&gt;,       // &quot;03:00&quot;
    pub tags: Option&lt;String&gt;,
}

pub struct RosterModel {
    pub crew_name: String,
    pub year: i32,
    pub month: u32,
    pub entries: Vec&lt;RosterEntry&gt;,
}
```

### Application Ports (`src/application/mod.rs`)

```rust
pub trait RosterParser {
    fn parse(&amp;self, pdf_path: &amp;std::path::Path) -&gt; anyhow::Result&lt;crate::domain::RosterModel&gt;;
}

pub trait CalendarWriter {
    fn write(&amp;self, model: &amp;crate::domain::RosterModel) -&gt; String;
}

pub fn convert(
    parser: &amp;dyn RosterParser,
    writer: &amp;dyn CalendarWriter,
    pdf: &amp;std::path::Path,
) -&gt; anyhow::Result&lt;String&gt; {
    let model = parser.parse(pdf)?;
    Ok(writer.write(&amp;model))
}
```

---

## PDF Parsing Algorithm

The Jeppesen duty-plan PDF contains a header section (crew name, stats) and a
**detail table** with these columns (dynamic — locate from the header row):

```
Date (LT) | Trip ID | Report (LT) | Pos | Activity | From | To |
Start (UTC) | End (UTC) | A/C | Layover | Timezone Difference | Blocktime
```

### Step-by-step

1. Open PDF with `pdfplumber`. Per page, extract all words with bounding boxes.
2. **Locate column bands dynamically**: find the header row by searching for the
   word `"Date"` followed by `"(LT)"` in Y-proximity. Record the X-center of each
   column header word as the column band centre. Never hardcode X values.
3. **Cluster words into rows**: group words by Y coordinate (tolerance = half line
   height). Within each row, assign each word to the nearest column band.
4. **Identify date blocks**: a row whose `Date (LT)` cell matches
   `\d{2}\s+(Mon|Tue|Wed|Thu|Fri|Sat|Sun)` opens a new date block.
5. **Classify entry kind**:
   - Activity ∈ {`FREE`, `OFF`} with `Start==00:00` → `EntryKind::Free` / `Off`, all-day
   - Activity == `SB` or `SBX` with `Start==00:00` → `EntryKind::Standby`, all-day
   - Activity == `EM` with real times → `EntryKind::Em`, timed
   - Activity == `Layover` or `From`/`To` empty and no flight number → `EntryKind::Layover`, all-day
   - Otherwise → `EntryKind::Flight`, timed
6. **Midnight crossing resolution** (critical):
   A flight leg may be split across two date blocks:
   - Row A (date D): has `From` + `Start (UTC)` but no `To` or `End (UTC)`
   - Row B (date D+1 or later): same flight number, has `To` + `End (UTC)` but no `From` or `Start (UTC)`
   Merge A and B into one `RosterEntry`:
   `start_utc` from row A's date + Start time,
   `end_utc` from row B's date + End time.
   Validate: `end_utc > start_utc`. If same-day `end < start`, add 1 day to end.
7. **UTC assembly**: `Start (UTC)` and `End (UTC)` columns already contain UTC times.
   Combine with the date of their respective date block (after midnight-merge).
   `Report (LT)` and `Timezone Difference` go into `DESCRIPTION` only — never used
   for time calculation.
8. **Error handling**: missing header row, unknown activity code, unmatched half-leg →
   return `Err(anyhow!(...))` with row context. Never silently drop entries.
   On error, `main.rs` writes `<pdf-stem>.parse-error.txt` instead of a `.ics`.

---

## ICS Output Rules (`src/infrastructure/ics_writer.rs`)

- `PRODID:-//kressnerd//DutyPlan2Ical//EN`
- **Timed events**: `DTSTART` and `DTEND` as UTC (`20990106T090000Z`).
- **All-day events**: `DTSTART;VALUE=DATE:20990101` and `DTEND;VALUE=DATE:20990102`
  (`DTEND` = date + 1 day, non-inclusive per RFC 5545).
- `UID`: deterministic, format `YYYYMMDD-<SUMMARY-slug>@dutyplan2ical`
  where slug = activity code + from + to, dashes, uppercased, no spaces.
  Examples: `20990106-ZZ100-ABC-DEF@dutyplan2ical`, `20990101-FREE@dutyplan2ical`.
- `DTSTAMP`: set to `YYYYMMM01T000000Z` of the roster month (deterministic for tests).
- `SUMMARY`:
  - Flight: `ZZ100 ABC-DEF`
  - Layover: `Layover GHI`
  - Other: activity code as-is (`FREE`, `OFF`, `EM`, etc.)
- `LOCATION`: `ABC-DEF` for flights, station code for Layover/EM, omitted for FREE/OFF/SB.
- `DESCRIPTION` (omit if all fields absent):
  `Trip <id> | Report <hh:mm> LT | A/C <ac> | Block <hh:mm> | TZ <tz> | Layover <hh:mm> | Tags <tags>`
  Include only the fields that are present. Separate with ` | `.
- Line folding: any content line exceeding 75 octets must be folded with CRLF + single space.
- Output encoding: UTF-8.

---

## Test Fixture — Synthetic PDF

**Never commit any real duty-plan PDF.**

Generate the test fixture from `tests/fixtures/gen/roster.typ` using Typst
(OSS, Apache-2.0). The fixture covers all entry kinds and all edge cases:

```
tests/fixtures/gen/roster.typ   ← Typst source (committed)
tests/fixtures/reference_roster.pdf  ← compiled output (committed, no PII)
```

After compiling with `typst compile roster.typ reference_roster.pdf`, strip PDF
metadata:

```bash
exiftool -all= -overwrite_original tests/fixtures/reference_roster.pdf
```

The synthetic roster represents **January 2099**, crew **DOE JANE**, staff **STAFF-0**,
phone **PHONE-0**, company **ZZ**, aircraft qualification **320**.

**Synthetic table** (all times UTC, all codes invented):

| Date       | Trip   | Report | Pos | Activity | From | To  | Start | End   | A/C | Layover | Block |
|------------|--------|--------|-----|----------|------|-----|-------|-------|-----|---------|-------|
| 01 Sat     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 02 Sun     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 03 Mon     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 04 Tue     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 06 Thu     | 100001 | 08:10  | FB  | ZZ100    | ABC  | DEF | 09:00 | 12:00 | 320 |         | 03:00 |
|            | 100001 |        | FB  | ZZ100    | DEF  | GHI | 13:00 | 15:30 | 320 | 51:30   | 02:30 |
| 07 Fri     |        |        |     | Layover  | GHI  |     | 00:00 | 00:00 |     |         |       |
| 08 Sat     | 100001 | 18:10  | FB  | ZZ101    | GHI  | DEF | 19:00 | 21:00 | 320 |         | 02:00 |
|            | 100001 |        | FB  | ZZ101    | DEF  |     | 22:00 |       | 320 |         | 04:30 |
| 09 Sun     |        |        |     | ZZ101    |      | ABC |       | 02:30 | 320 |         |       |
| 12 Wed     | 100002 | 10:10  | FB  | ZZ200    | ABC  | JKL | 11:00 | 13:00 | 320 |         | 02:00 |
|            | 100002 |        | FB  | ZZ201    | JKL  | ABC | 14:00 | 16:00 | 320 |         | 02:00 |
| 15 Sat     | AK     |        |     | EM       | ABC  |     | 06:00 | 10:00 |     |         |       |
| 20 Thu     | 100003 | 17:40  | FB  | ZZ300    | ABC  |     | 19:00 |       | 359 |         | 12:00 |
| 21 Fri     |        |        |     | ZZ300    |      | MNO |       | 07:00 | 359 |         |       |
| 22 Sat     |        |        |     | Layover  | MNO  |     | 00:00 | 00:00 |     |         |       |
| 23 Sun     | 100003 | 19:50  | FB  | ZZ301    | MNO  |     | 21:00 |       | 359 |         | 12:00 |
| 24 Mon     |        |        |     | ZZ301    |      | ABC |       | 09:00 | 359 |         |       |
| 28 Fri     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 29 Sat     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 30 Sun     |        |        |     | FREE     |      |     | 00:00 | 00:00 |     |         |       |
| 31 Mon     |        |        |     | OFF      |      |     | 00:00 | 00:00 |     |         | 3_OFF |

Midnight crossings: ZZ101 DEF→ABC (08→09 Jan), ZZ300 ABC→MNO (20→21 Jan),
ZZ301 MNO→ABC (23→24 Jan).

---

## Golden Oracle `tests/golden/2099-01.ics`

Commit this file verbatim (CRLF line endings):

```ics
BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//kressnerd//DutyPlan2Ical//EN
CALSCALE:GREGORIAN
BEGIN:VEVENT
UID:20990101-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990101
DTEND;VALUE=DATE:20990102
END:VEVENT
BEGIN:VEVENT
UID:20990102-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990102
DTEND;VALUE=DATE:20990103
END:VEVENT
BEGIN:VEVENT
UID:20990103-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990103
DTEND;VALUE=DATE:20990104
END:VEVENT
BEGIN:VEVENT
UID:20990104-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990104
DTEND;VALUE=DATE:20990105
END:VEVENT
BEGIN:VEVENT
UID:20990106-ZZ100-ABC-DEF@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ100 ABC-DEF
LOCATION:ABC-DEF
DESCRIPTION:Trip 100001 | Report 08:10 LT | A/C 320 | Block 03:00 | TZ +01:00
DTSTART:20990106T090000Z
DTEND:20990106T120000Z
END:VEVENT
BEGIN:VEVENT
UID:20990106-ZZ100-DEF-GHI@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ100 DEF-GHI
LOCATION:DEF-GHI
DESCRIPTION:Trip 100001 | A/C 320 | Block 02:30 | TZ +01:00 | Layover 51:30
DTSTART:20990106T130000Z
DTEND:20990106T153000Z
END:VEVENT
BEGIN:VEVENT
UID:20990107-LAYOVER-GHI@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:Layover GHI
LOCATION:GHI
DTSTART;VALUE=DATE:20990107
DTEND;VALUE=DATE:20990108
END:VEVENT
BEGIN:VEVENT
UID:20990108-ZZ101-GHI-DEF@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ101 GHI-DEF
LOCATION:GHI-DEF
DESCRIPTION:Trip 100001 | Report 18:10 LT | A/C 320 | Block 02:00 | TZ +01:00
DTSTART:20990108T190000Z
DTEND:20990108T210000Z
END:VEVENT
BEGIN:VEVENT
UID:20990108-ZZ101-DEF-ABC@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ101 DEF-ABC
LOCATION:DEF-ABC
DESCRIPTION:Trip 100001 | A/C 320 | Block 04:30 | TZ +01:00
DTSTART:20990108T220000Z
DTEND:20990109T023000Z
END:VEVENT
BEGIN:VEVENT
UID:20990112-ZZ200-ABC-JKL@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ200 ABC-JKL
LOCATION:ABC-JKL
DESCRIPTION:Trip 100002 | Report 10:10 LT | A/C 320 | Block 02:00 | TZ +01:00
DTSTART:20990112T110000Z
DTEND:20990112T130000Z
END:VEVENT
BEGIN:VEVENT
UID:20990112-ZZ201-JKL-ABC@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ201 JKL-ABC
LOCATION:JKL-ABC
DESCRIPTION:Trip 100002 | A/C 320 | Block 02:00 | TZ +01:00
DTSTART:20990112T140000Z
DTEND:20990112T160000Z
END:VEVENT
BEGIN:VEVENT
UID:20990115-EM-ABC@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:EM
LOCATION:ABC
DESCRIPTION:Trip AK | Activity EM
DTSTART:20990115T060000Z
DTEND:20990115T100000Z
END:VEVENT
BEGIN:VEVENT
UID:20990120-ZZ300-ABC-MNO@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ300 ABC-MNO
LOCATION:ABC-MNO
DESCRIPTION:Trip 100003 | Report 17:40 LT | A/C 359 | Block 12:00 | TZ +08:00 | Tags 5_MFL
DTSTART:20990120T190000Z
DTEND:20990121T070000Z
END:VEVENT
BEGIN:VEVENT
UID:20990122-LAYOVER-MNO@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:Layover MNO
LOCATION:MNO
DTSTART;VALUE=DATE:20990122
DTEND;VALUE=DATE:20990123
END:VEVENT
BEGIN:VEVENT
UID:20990123-ZZ301-MNO-ABC@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:ZZ301 MNO-ABC
LOCATION:MNO-ABC
DESCRIPTION:Trip 100003 | Report 19:50 LT | A/C 359 | Block 12:00 | TZ +02:00 | Tags 5_MFL
DTSTART:20990123T210000Z
DTEND:20990124T090000Z
END:VEVENT
BEGIN:VEVENT
UID:20990128-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990128
DTEND;VALUE=DATE:20990129
END:VEVENT
BEGIN:VEVENT
UID:20990129-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990129
DTEND;VALUE=DATE:20990130
END:VEVENT
BEGIN:VEVENT
UID:20990130-FREE@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:FREE
DTSTART;VALUE=DATE:20990130
DTEND;VALUE=DATE:20990131
END:VEVENT
BEGIN:VEVENT
UID:20990131-OFF@dutyplan2ical
DTSTAMP:20990101T000000Z
SUMMARY:OFF
DESCRIPTION:Tags 3_OFF
DTSTART;VALUE=DATE:20990131
DTEND;VALUE=DATE:20990201
END:VEVENT
END:VCALENDAR
```

---

## Test Suite

### L1 — Extraction (`tests/extraction_test.rs`)

```rust
#[test]
fn pdf_yields_words_and_reports_table_count() {
    let doc = pdfplumber::Pdf::open(&quot;tests/fixtures/reference_roster.pdf&quot;).unwrap();
    let total_words: usize = doc.pages().map(|p| p.words().len()).sum();
    assert!(total_words &gt; 50, &quot;extraction returned no words — crate or PDF regression&quot;);
    // Empirical gate: log table count; hard-assert once find_tables behaviour confirmed
    let tables: usize = doc.pages()
        .map(|p| p.find_tables(&amp;pdfplumber::TableSettings::default()).len())
        .sum();
    eprintln!(&quot;tables detected: {tables}&quot;); // adjust to hard assert after first run
}
```

### L2 — Domain Invariants (`tests/domain_test.rs`)

```rust
#[test]
fn maps_19_entries_with_3_midnight_crossings() {
    let parser = infrastructure::PdfPlumberParser::new();
    let model = parser.parse(std::path::Path::new(
        &quot;tests/fixtures/reference_roster.pdf&quot;
    )).unwrap();

    assert_eq!(model.entries.len(), 19);

    for e in &amp;model.entries {
        if let (Some(s), Some(end)) = (e.start_utc, e.end_utc) {
            assert!(end &gt; s, &quot;DTEND &gt; DTSTART violated: {:?}&quot;, e.activity_code);
        }
    }
    // Blocktime must NOT be used to derive DTEND — it is description-only.
    // Invariant: blocktime ≠ (end - start) for midnight-crossing entries.

    let midnight = model.entries.iter()
        .filter(|e| matches!((&amp;e.start_utc, &amp;e.end_utc),
            (Some(s), Some(t)) if t.date_naive() &gt; s.date_naive()))
        .count();
    assert_eq!(midnight, 3, &quot;expected ZZ101/ZZ300/ZZ301 midnight crossings&quot;);
}
```

### L3 — Golden Oracle (`tests/golden_test.rs`)

```rust
fn normalize(ics: &amp;str) -&gt; String {
    ics.lines()
        .filter(|l| !l.starts_with(&quot;DTSTAMP&quot;))
        .collect::&lt;Vec&lt;_&gt;&gt;()
        .join(&quot;\r\n&quot;)
}

#[test]
fn reproduces_golden_oracle() {
    let parser = infrastructure::PdfPlumberParser::new();
    let writer = infrastructure::IcsWriter::new();
    let model = parser.parse(std::path::Path::new(
        &quot;tests/fixtures/reference_roster.pdf&quot;
    )).unwrap();
    let got = writer.write(&amp;model);
    let want = std::fs::read_to_string(&quot;tests/golden/2099-01.ics&quot;).unwrap();
    assert_eq!(normalize(&amp;got), normalize(&amp;want));
}
```

---

## `Cargo.toml`

```toml
[package]
name = &quot;dutyplan2ical&quot;
version = &quot;0.1.0&quot;
edition = &quot;2021&quot;

[dependencies]
pdfplumber = &quot;=0.1.0&quot;   # EXACT pin — v0.1, no SemVer guarantee
anyhow = &quot;1&quot;
chrono = { version = &quot;0.4&quot;, features = [&quot;std&quot;] }

[profile.release]
opt-level = &quot;z&quot;
lto = true
strip = true
```

---

## `.cargo/config.toml`

```toml
[target.x86_64-pc-windows-gnu]
linker = &quot;x86_64-w64-mingw32-gcc&quot;
```

---

## `main.rs`

```rust
fn main() -&gt; std::process::ExitCode {
    let Some(pdf) = std::env::args_os().nth(1) else {
        eprintln!(&quot;Usage: drop a PDF onto the exe, or pass path as argument&quot;);
        return std::process::ExitCode::from(2);
    };
    let pdf = std::path::PathBuf::from(&amp;pdf);
    let parser = infrastructure::PdfPlumberParser::new();
    let writer = infrastructure::IcsWriter::new();
    match application::convert(&amp;parser, &amp;writer, &amp;pdf) {
        Ok(ics) =&gt; {
            let out = pdf.with_extension(&quot;ics&quot;);
            std::fs::write(&amp;out, ics).expect(&quot;cannot write .ics&quot;);
            eprintln!(&quot;Written: {}&quot;, out.display());
            std::process::ExitCode::SUCCESS
        }
        Err(e) =&gt; {
            let err_path = pdf.with_extension(&quot;parse-error.txt&quot;);
            std::fs::write(&amp;err_path, format!(&quot;{e:#}&quot;)).ok();
            eprintln!(&quot;Error: {e:#}&quot;);
            std::process::ExitCode::FAILURE
        }
    }
}
```

---

## `.gitignore`

```
/private/
*_real*.pdf
*_original*.pdf
.pii-denylist
target/
```

---

## PII Guard (`scripts/pii-guard.sh`)

No real PII literals in this script. Pattern-based only:

```bash
#!/usr/bin/env bash
set -euo pipefail
# Detect PII-shaped patterns: phone numbers (+CC/digits) and staff-number form (6 digits + letter)
PATTERNS=&#x27;(\+[0-9]{1,3}/[0-9]{6,})|([0-9]{6}[A-Z])&#x27;
if grep -rIEn &quot;$PATTERNS&quot; --exclude-dir=.git --exclude=&#x27;*.sh&#x27; .; then
  echo &quot;PII pattern found in repository&quot;; exit 1
fi
echo &quot;PII guard: clean&quot;
```

---

## CI (`.github/workflows/build.yml`)

```yaml
name: build
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:

- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- run: cargo test

  pii-guard:
    runs-on: ubuntu-latest
    steps:

- uses: actions/checkout@v4
- run: bash scripts/pii-guard.sh

  license-check:
    runs-on: ubuntu-latest
    steps:

- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
- run: cargo install cargo-deny --locked
- run: cargo deny check licenses

  release-windows:
    needs: [test, pii-guard, license-check]
    runs-on: ubuntu-latest
    steps:

- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@stable
        with: { targets: x86_64-pc-windows-gnu }

- run: sudo apt-get install -y mingw-w64
- run: cargo build --release --target x86_64-pc-windows-gnu
- uses: actions/upload-artifact@v4
        with:
          name: dutyplan2ical-windows
          path: target/x86_64-pc-windows-gnu/release/dutyplan2ical.exe
```

---

## Acceptance Criteria

All of the following must be true before the task is complete:

1. `cargo test` passes with zero failures on Linux (native).
2. L1: `pdf_yields_words_and_reports_table_count` — no panic, word count > 50.
3. L2: `maps_19_entries_with_3_midnight_crossings` — exactly 19 entries, exactly 3 midnight-crossing legs, `DTEND > DTSTART` for all timed entries.
4. L3: `reproduces_golden_oracle` — DTSTAMP-normalised output matches `tests/golden/2099-01.ics` byte-for-byte (CRLF).
5. `cargo build --release --target x86_64-pc-windows-gnu` succeeds and produces a single `.exe` with no companion DLL.
6. `cargo deny check licenses` passes (all deps MIT or Apache-2.0).
7. `bash scripts/pii-guard.sh` exits 0 on the full repo.
8. No file matching real-person patterns (phone, 6-digit+letter staff code) exists anywhere in the committed tree.
9. Dropping any valid Jeppesen duty-plan PDF onto the `.exe` on Windows produces a `.ics` file next to the PDF; on parse failure it produces a `.parse-error.txt` instead of a partial `.ics`.
10. The generated `.ics` for the synthetic fixture passes validation at https://icalendar.org/validator.html (RFC 5545 compliant, no errors).

---

## Known Risks / Agent Guidance

- `pdfplumber` is v0.1 — the API may differ from the code snippets above. Check
  actual method signatures against `cargo doc` before implementing. If `find_tables`
  is unavailable or does not recognise the Jeppesen table layout, fall back to
  the word bounding-box column-clustering approach (Step 2–3 of the algorithm).
- The Typst-generated PDF may produce slightly different word-extraction coordinates
  than a real Jeppesen PDF. The test suite validates correctness on the synthetic
  fixture. A separate local (non-committed) validation against a real PDF is
  recommended before declaring the parser production-ready.
- Blocktime values in the synthetic fixture are intentionally set so that
  `blocktime ≠ (DTEND − DTSTART)` for midnight-crossing entries. Do not use
  blocktime to derive or validate DTEND — it is a description-only field.
- RFC 5545 §3.1: content lines must be folded at 75 **octets** (not characters).
  Multi-byte UTF-8 characters must not be split across fold boundaries.

