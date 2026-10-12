// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: `pdfplumber` positioned content → `RosterSource`.
//! The only crate that depends on the extraction dependency.
//!
//! Column bands are derived from the detail-table header row and rows are
//! clustered by vertical position; reading order is never treated as table
//! structure (guardrail), and no column coordinate is hard-coded.

use std::cmp::Ordering;

use duty2cal_core::application::{RawRosterTable, RosterSource, SourceDocument, SourceError};
use duty2cal_core::domain::timing;
use pdfplumber::{Pdf, TextOptions, Word, WordOptions};
use time::OffsetDateTime;

/// Vertical tolerance in points for clustering words into one row.
const ROW_TOLERANCE: f64 = 4.0;
/// Maximum horizontal gap in points for merging header words into one label.
const LABEL_GAP: f64 = 12.0;
/// Vertical gap in points larger than the row pitch that ends the detail table.
const TABLE_GAP: f64 = 20.0;

/// Reads a supplied document with the pinned extraction crate.
#[derive(Debug, Default)]
pub struct PdfRosterSource;

impl RosterSource for PdfRosterSource {
    fn read(&self, bytes: &[u8]) -> Result<SourceDocument, SourceError> {
        let pdf = Pdf::open_bytes(bytes, None)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
        let options = WordOptions::default();

        let mut table = RawRosterTable::default();
        let mut revision: Option<OffsetDateTime> = None;
        let mut family_recognised = false;

        for page in pdf.pages() {
            let page = page.map_err(|error| SourceError::Unreadable(error.to_string()))?;
            let words = page.extract_words(&options);
            if revision.is_none() {
                revision = created_footer(&page.extract_text(&TextOptions::default()));
            }

            // A continuation page repeats the detail-table header, so nothing is
            // a detail row before that header on its own page.
            let mut areas: Option<Vec<Area>> = None;
            let mut last_top: Option<f64> = None;

            for row in cluster_rows(&words, ROW_TOLERANCE) {
                let top = row.first().map(|word| word.bbox.top).unwrap_or_default();

                if let Some(header) = header_areas(&row) {
                    if table.columns.is_empty() {
                        table.columns = header.iter().map(|area| area.label.clone()).collect();
                    }
                    areas = Some(header);
                    family_recognised = true;
                    last_top = Some(top);
                    continue;
                }

                if areas.is_some() {
                    if let Some(previous) = last_top {
                        if top - previous > TABLE_GAP {
                            // A gap larger than the row pitch ends the table:
                            // whatever follows on the page is footer material.
                            areas = None;
                            continue;
                        }
                    }
                }

                let Some(current) = areas.as_ref() else {
                    continue;
                };
                table.rows.push(assign_cells(&row, current));
                last_top = Some(top);
            }
        }

        if !family_recognised {
            return Err(SourceError::UnsupportedFamily);
        }
        Ok(SourceDocument { table, revision })
    }
}

/// A detail-table column band: its header label and horizontal extent.
#[derive(Debug, Clone)]
struct Area {
    label: String,
    x0: f64,
    x1: f64,
}

/// Group words into rows by vertical position (top to bottom).
fn cluster_rows(words: &[Word], tolerance: f64) -> Vec<Vec<&Word>> {
    let mut sorted: Vec<&Word> = words.iter().collect();
    sorted.sort_by(|a, b| cmp(a.bbox.top, b.bbox.top));

    let mut rows: Vec<Vec<&Word>> = Vec::new();
    for word in sorted {
        match rows.last_mut() {
            Some(row) if (word.bbox.top - row[0].bbox.top).abs() <= tolerance => row.push(word),
            _ => rows.push(vec![word]),
        }
    }
    rows
}

/// If this row is the detail-table header, return its column bands.
///
/// The supported family's header is recognised by the local-date, local-report
/// and UTC-time labels; word extraction merges adjacent label words, so
/// recognition works on the row's joined text.
fn header_areas(row: &[&Word]) -> Option<Vec<Area>> {
    let joined = row
        .iter()
        .map(|word| word.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let recognised = joined.contains("Date") && joined.contains("(LT)") && joined.contains("(UTC)");
    if !recognised {
        return None;
    }

    let mut sorted = row.to_vec();
    sorted.sort_by(|a, b| cmp(a.bbox.x0, b.bbox.x0));

    let mut areas: Vec<Area> = Vec::new();
    for word in sorted {
        match areas.last_mut() {
            Some(area) if word.bbox.x0 - area.x1 <= LABEL_GAP => {
                area.label.push(' ');
                area.label.push_str(&word.text);
                area.x1 = word.bbox.x1;
            }
            _ => areas.push(Area {
                label: word.text.clone(),
                x0: word.bbox.x0,
                x1: word.bbox.x1,
            }),
        }
    }
    Some(areas)
}

/// Assign each word of a data row to the nearest column band.
fn assign_cells(row: &[&Word], areas: &[Area]) -> Vec<String> {
    let mut cells = vec![String::new(); areas.len()];
    let mut sorted = row.to_vec();
    sorted.sort_by(|a, b| cmp(a.bbox.x0, b.bbox.x0));

    for word in sorted {
        let centre = (word.bbox.x0 + word.bbox.x1) / 2.0;
        let nearest = areas
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| cmp(distance(centre, a), distance(centre, b)))
            .map(|(index, _)| index);
        if let Some(index) = nearest {
            if !cells[index].is_empty() {
                cells[index].push(' ');
            }
            cells[index].push_str(&word.text);
        }
    }
    cells
}

/// The source's own revision stamp, written as `Created YYYY-MM-DD HH:MM UTC`.
///
/// This is source metadata, not activity timing: it never supplies an activity
/// date (the change's timestamp-provenance decision).
fn created_footer(text: &str) -> Option<OffsetDateTime> {
    let rest = text.split("Created").nth(1)?;
    let mut parts = rest.split_whitespace();
    let date = timing::parse_iso_date(parts.next()?)?;
    let clock = timing::parse_clock(parts.next()?)?;
    if !parts.next()?.eq_ignore_ascii_case("UTC") {
        return None;
    }
    Some(date.with_time(clock).assume_utc())
}

/// Distance from a point to a band, zero when inside it.
fn distance(x: f64, area: &Area) -> f64 {
    if x < area.x0 {
        area.x0 - x
    } else if x > area.x1 {
        x - area.x1
    } else {
        0.0
    }
}

/// Total order over coordinates, treating a NaN as equal rather than panicking.
fn cmp(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}
