// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: `pdfplumber` positioned content → `RosterSource`.
//! The only crate that depends on the extraction dependency.
//!
//! Column bands are derived from the detail-table header and rows are
//! clustered by vertical position; reading order is never treated as table
//! structure (guardrail), and no column coordinate is hard-coded. The family's
//! header wraps over several rows, so a band runs from one label to the next
//! and the label words that continue a column (`(LT)`, `ID`, `Difference`)
//! join the band that contains them.

use std::cmp::Ordering;

use duty2cal_core::application::{RawRosterTable, RosterSource, SourceDocument, SourceError};
use duty2cal_core::domain::timing;
use pdfplumber::{Pdf, TextOptions, Word, WordOptions};
use time::{Date, Month, OffsetDateTime, Weekday};

/// Vertical tolerance in points for clustering words into one row.
const ROW_TOLERANCE: f64 = 4.0;
/// Vertical gap in points larger than the row pitch that ends the detail table.
const TABLE_GAP: f64 = 20.0;
/// Header words that continue the column to their left instead of starting one.
const CONTINUATION_TOKENS: [&str; 4] = ["(lt)", "(utc)", "id", "difference"];
/// The labels that recognise the detail-table header of the supported family.
const ANCHOR_LABELS: [&str; 3] = ["date", "(lt)", "(utc)"];
/// The labels rows print in: the first row that prints one ends the header.
const BODY_LABELS: [&str; 5] = ["date", "pos", "activity", "from", "to"];
/// The English month abbreviations the family prints in its heading.
const MONTH_NAMES: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];
/// The English weekday abbreviations the family prints beside the day number.
const WEEKDAY_NAMES: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];
/// The weekday each name above denotes, in the same order.
const WEEKDAYS: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

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
        let mut roster_month: Option<(i32, Month)> = None;
        let mut family_recognised = false;

        for page in pdf.pages() {
            let page = page.map_err(|error| SourceError::Unreadable(error.to_string()))?;
            let words = page.extract_words(&options);
            let text = page.extract_text(&TextOptions::default());
            if revision.is_none() {
                revision = created_footer(&text);
            }
            if roster_month.is_none() {
                roster_month = heading_month(&text);
            }

            // A continuation page repeats the detail-table header, so nothing is
            // a detail row before that header on its own page.
            let mut areas: Option<Vec<Area>> = None;
            let mut header: Vec<&Word> = Vec::new();
            let mut last_top: Option<f64> = None;
            let mut columns_frozen = !table.columns.is_empty();

            for row in cluster_rows(&words, ROW_TOLERANCE) {
                let top = row.first().map(|word| word.bbox.top).unwrap_or_default();

                if header.is_empty() && starts_header(&row) {
                    header = row.clone();
                    let bands = build_areas(&header);
                    areas = Some(bands.clone());
                    if !columns_frozen {
                        table.columns = labels(&bands);
                    }
                    family_recognised |= recognises_family(&header);
                    last_top = Some(top);
                    continue;
                }

                if let Some(previous) = last_top {
                    if top - previous > TABLE_GAP {
                        // A gap larger than the row pitch ends the table:
                        // whatever follows on the page is footer material.
                        areas = None;
                        header.clear();
                        continue;
                    }
                }

                if !header.is_empty() {
                    let bands = build_areas(&header);
                    if !prints_body_word(&row, &bands) {
                        // Still label material: the header wraps over several
                        // rows and its later rows carry the trailing columns.
                        header.extend(row.iter().copied());
                        family_recognised |= recognises_family(&header);
                        let bands = build_areas(&header);
                        areas = Some(bands.clone());
                        if !columns_frozen {
                            table.columns = labels(&bands);
                        }
                        last_top = Some(top);
                        continue;
                    }
                    areas = Some(bands.clone());
                    if !columns_frozen {
                        table.columns = labels(&bands);
                        columns_frozen = true;
                    }
                    header.clear();
                }

                let Some(current) = areas.as_ref() else {
                    continue;
                };
                let mut cells = assign_cells(&row, current);
                resolve_date(&mut cells, current, roster_month);
                table.rows.push(cells);
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

/// Whether this row opens the detail-table header.
///
/// The header always prints the local-date label; the remaining anchor labels
/// may land on a later print row of the same wrapped header.
fn starts_header(row: &[&Word]) -> bool {
    joined_text(row).contains(ANCHOR_LABELS[0])
}

/// Whether the header rows read so far recognise the supported family.
///
/// The family is recognised by the local-date, local-report and UTC-time
/// labels, which a wrapped header may print on different rows.
fn recognises_family(header: &[&Word]) -> bool {
    let joined = joined_text(header);
    ANCHOR_LABELS.iter().all(|label| joined.contains(label))
}

/// The lowercased text of a word list, in reading order.
fn joined_text(words: &[&Word]) -> String {
    words
        .iter()
        .map(|word| word.text.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Derive the column bands from every word of the header rows.
///
/// A band starts at a label word and reaches to the next label word, so a
/// header that wraps over several rows still yields one band per column; a
/// word that continues a column joins the band that contains it.
fn build_areas(words: &[&Word]) -> Vec<Area> {
    let mut starts: Vec<(f64, &Word)> = words
        .iter()
        .filter(|word| !is_continuation(&word.text))
        .map(|word| (word.bbox.x0, *word))
        .collect();
    starts.sort_by(|a, b| cmp(a.0, b.0));

    let mut columns: Vec<Area> = Vec::new();
    for (x0, word) in starts {
        match columns.last_mut() {
            Some(area) if (x0 - area.x0).abs() <= 1.0 => {}
            Some(area) => {
                area.x1 = x0;
                columns.push(Area {
                    label: word.text.clone(),
                    x0,
                    x1: f64::INFINITY,
                });
            }
            None => columns.push(Area {
                label: word.text.clone(),
                x0,
                x1: f64::INFINITY,
            }),
        }
    }

    for word in words.iter().filter(|word| is_continuation(&word.text)) {
        let centre = (word.bbox.x0 + word.bbox.x1) / 2.0;
        if let Some(area) = columns
            .iter_mut()
            .find(|area| centre >= area.x0 && centre < area.x1)
        {
            area.label.push(' ');
            area.label.push_str(&word.text);
        }
    }
    columns
}

/// The labels of a band list, in band order.
fn labels(areas: &[Area]) -> Vec<String> {
    areas.iter().map(|area| area.label.clone()).collect()
}

/// Whether this header word continues the column to its left.
fn is_continuation(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    CONTINUATION_TOKENS.contains(&lower.as_str())
}

/// Whether this row prints a word inside one of the labels rows print in.
fn prints_body_word(row: &[&Word], areas: &[Area]) -> bool {
    row.iter().any(|word| {
        let centre = (word.bbox.x0 + word.bbox.x1) / 2.0;
        areas.iter().any(|area| {
            centre >= area.x0 && centre < area.x1 && is_body_label(&area.label)
        })
    })
}

/// Whether a band label is one of the labels rows print in.
fn is_body_label(label: &str) -> bool {
    BODY_LABELS
        .iter()
        .any(|wanted| label_matches(label, wanted))
}

/// Whether a derived label denotes the wanted column.
///
/// Word extraction merges adjacent label words without a space, so a label
/// matches when it equals the wanted name or continues it at a real boundary.
fn label_matches(label: &str, wanted: &str) -> bool {
    let lower = label.to_ascii_lowercase();
    lower == wanted
        || lower.starts_with(&format!("{wanted} "))
        || lower.starts_with(&format!("{wanted}("))
}

/// Replace the family's `DD Ddd` date cell with its calendar date.
///
/// The family prints the day and weekday in the row and the month and year once
/// in the document heading; the weekday is checked against the resolved date, so
/// a misaligned cell is never accepted as activity timing.
fn resolve_date(cells: &mut [String], areas: &[Area], month: Option<(i32, Month)>) {
    let Some((year, month)) = month else {
        return;
    };
    let Some(index) = areas.iter().position(|area| is_body_label_of(area, "date")) else {
        return;
    };
    let mut parts = cells[index].split_whitespace();
    let (Some(day), Some(weekday), None) = (parts.next(), parts.next(), parts.next()) else {
        return;
    };
    let Ok(day) = day.parse::<u8>() else {
        return;
    };
    let Some(wanted) = WEEKDAY_NAMES
        .iter()
        .position(|name| name.eq_ignore_ascii_case(weekday))
        .map(|index| WEEKDAYS[index])
    else {
        return;
    };
    let Ok(date) = Date::from_calendar_date(year, month, day) else {
        return;
    };
    if date.weekday() != wanted {
        return;
    }
    cells[index] = format!("{:04}-{:02}-{:02}", date.year(), u8::from(date.month()), date.day());
}

/// Whether a band is the named body label.
fn is_body_label_of(area: &Area, label: &str) -> bool {
    label_matches(&area.label, &label.to_ascii_lowercase())
}

/// The document heading's month and year, printed as `Month: June 2026`.
fn heading_month(text: &str) -> Option<(i32, Month)> {
    let rest = text.split("Month:").nth(1)?;
    let mut parts = rest.split_whitespace();
    // The month and the year are separate prints, but a long month name can
    // touch the year, and the extraction then joins them.
    let stamp = parts.next()?;
    let letters: String = stamp.chars().take_while(char::is_ascii_alphabetic).collect();
    let digits = stamp.get(letters.len()..).unwrap_or_default();
    let year = if digits.is_empty() {
        parts.next()?.parse::<i32>().ok()?
    } else {
        digits.parse::<i32>().ok()?
    };
    let name = letters.to_ascii_lowercase();
    let index = MONTH_NAMES
        .iter()
        .position(|month| name.starts_with(month))?;
    Some((year, Month::try_from(index as u8 + 1).ok()?))
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

/// The source's own revision stamp, written as `Created DDMonYYYY HH:MM (UTC)`.
///
/// This is source metadata, not activity timing: it never supplies an activity
/// date (the change's timestamp-provenance decision). The ISO spelling is also
/// accepted, for sources that print it that way.
fn created_footer(text: &str) -> Option<OffsetDateTime> {
    let rest = text.split("Created").nth(1)?;
    let mut parts = rest.split_whitespace();
    let stamp = parts.next()?;
    let date = match timing::parse_iso_date(stamp) {
        Some(date) => date,
        None => {
            let (year, month, day) = compact_date(stamp)?;
            Date::from_calendar_date(year, month, day).ok()?
        }
    };
    let clock = timing::parse_clock(parts.next()?)?;
    let zone = parts.next()?.trim_matches(['(', ')']);
    if !zone.eq_ignore_ascii_case("UTC") {
        return None;
    }
    Some(date.with_time(clock).assume_utc())
}

/// A date printed as `DDMonYYYY`, the family's footer spelling.
fn compact_date(token: &str) -> Option<(i32, Month, u8)> {
    let digits: String = token.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty() || digits.len() > 2 {
        return None;
    }
    let rest = token.get(digits.len()..)?;
    let (name, year) = rest.split_at_checked(3)?;
    let index = MONTH_NAMES
        .iter()
        .position(|month| name.eq_ignore_ascii_case(month))?;
    Some((
        year.parse::<i32>().ok()?,
        Month::try_from(index as u8 + 1).ok()?,
        digits.parse::<u8>().ok()?,
    ))
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
