// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: `pdfplumber` positioned content → `RosterSource`.
//! The only crate that depends on the extraction dependency.
//!
//! Column bands are derived from the detail-table header row and rows are
//! clustered by vertical position; reading order is never treated as table
//! structure (guardrail), and no column coordinate is hard-coded.

use std::cmp::Ordering;

use duty2cal_core::application::{RawRosterTable, RosterSource, SourceError};
use pdfplumber::{Pdf, Word, WordOptions};

/// Vertical tolerance in points for clustering words into one row.
const ROW_TOLERANCE: f64 = 4.0;
/// Maximum horizontal gap in points for merging header words into one label.
const LABEL_GAP: f64 = 6.0;

/// Reads a supplied document with the pinned extraction crate.
#[derive(Debug, Default)]
pub struct PdfRosterSource;

impl RosterSource for PdfRosterSource {
    fn read(&self, bytes: &[u8]) -> Result<RawRosterTable, SourceError> {
        let pdf = Pdf::open_bytes(bytes, None)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
        let options = WordOptions::default();

        let mut table = RawRosterTable::default();
        let mut areas: Option<Vec<Area>> = None;

        for page in pdf.pages() {
            let page = page.map_err(|error| SourceError::Unreadable(error.to_string()))?;
            let words = page.extract_words(&options);

            for row in cluster_rows(&words, ROW_TOLERANCE) {
                if let Some(header) = header_areas(&row) {
                    if table.columns.is_empty() {
                        table.columns = header.iter().map(|area| area.label.clone()).collect();
                    }
                    areas = Some(header);
                    continue;
                }
                // Material above the first detail-table header is overview or
                // statistics, and is not an activity source.
                let Some(areas) = areas.as_ref() else {
                    continue;
                };
                table.rows.push(assign_cells(&row, areas));
            }
        }

        if areas.is_none() {
            return Err(SourceError::UnsupportedFamily);
        }
        Ok(table)
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
/// and UTC-time labels; adjacent words are merged so wrapped headings such as
/// `Report (LT)` stay one column.
fn header_areas(row: &[&Word]) -> Option<Vec<Area>> {
    let labels: Vec<&str> = row.iter().map(|word| word.text.as_str()).collect();
    let recognised =
        labels.contains(&"Date") && labels.contains(&"(LT)") && labels.contains(&"(UTC)");
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
