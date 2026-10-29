// SPDX-License-Identifier: MPL-2.0
//! Differential evidence: the owned extraction against the test oracle.
//!
//! For each generated fixture the owned words match the oracle's word text
//! exactly and its `x0` and page-space `top` within 0.5 point; the shared table
//! construction then yields the same `RawRosterTable` and `SourceDocument`.
//! The real-corpus test is opt-in and reads `private/*.pdf` in place.

use std::path::Path;

use pdfplumber::{Pdf, Word, WordOptions};

use crate::reader::{self, ExtractedDocument, PageWords, PositionedWord};
use crate::table;
use duty2cal_pdf_fixtures as fixtures;

/// Point tolerance for the word box anchors.
const TOLERANCE: f64 = 0.5;

#[test]
fn classic_xref_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
    ));
}

#[test]
fn xref_stream_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::XrefStream,
    ));
}

#[test]
fn object_streams_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ObjectStreams,
    ));
}

#[test]
fn predicted_xref_stream_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::PredictedXrefStream,
    ));
}

#[test]
fn incremental_update_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::IncrementalUpdate,
    ));
}

#[test]
fn indirect_length_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
        fixtures::LayoutOptions {
            indirect_length: true,
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn inherited_resources_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
        fixtures::LayoutOptions {
            inherited_resources: true,
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn corrupt_checksum_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_corrupt_checksum_plan());
}

#[test]
fn leading_junk_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_leading_junk_plan());
}

#[test]
fn hybrid_xref_stream_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::HybridXrefStream,
    ));
}

#[test]
fn compact_xref_stream_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::CompactXrefStream,
    ));
}

#[test]
fn non_png_predictor_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
        fixtures::LayoutOptions {
            predictor: Some(2),
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn png_sub_filter_xref_stream_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::PredictedXrefStream,
        fixtures::LayoutOptions {
            png_filter: 1,
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn png_paeth_filter_xref_stream_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::PredictedXrefStream,
        fixtures::LayoutOptions {
            png_filter: 4,
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn raw_deflate_retry_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
        fixtures::LayoutOptions {
            raw_deflate_prefix: Some([0, 0]),
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn warned_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan(
        &fixtures::warned_plan(),
        fixtures::Encoding::ClassicXref,
    ));
}

/// The local corpus is release evidence, never CI. It reads every
/// `private/*.pdf` in place and prints one line per plan.
#[test]
#[ignore]
fn real_corpus_matches_the_oracle() {
    let Some(root) = std::env::var_os("DUTY2CAL_LOCAL_CORPUS") else {
        println!("SKIP: DUTY2CAL_LOCAL_CORPUS is not set");
        return;
    };
    let mut files: Vec<_> = std::fs::read_dir(Path::new(&root))
        .expect("read the local corpus directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "pdf"))
        .collect();
    files.sort();
    let mut failures = 0;
    for path in files {
        let bytes = std::fs::read(&path).expect("read a corpus plan");
        match check_against_oracle(&bytes) {
            Ok(()) => println!("PASS {}", path.display()),
            Err(error) => {
                failures += 1;
                println!("FAIL {}: {error}", path.display());
            }
        }
    }
    assert_eq!(failures, 0, "{failures} plan(s) differ from the oracle");
}

/// Compare one fixture and fail with the first mismatch.
fn compare_with_oracle(bytes: &[u8]) {
    if let Err(error) = check_against_oracle(bytes) {
        panic!("{error}");
    }
}

/// Compare owned words, table and document with the oracle.
fn check_against_oracle(bytes: &[u8]) -> Result<(), String> {
    let owned =
        reader::extract(bytes).map_err(|error| format!("owned extraction failed: {error}"))?;
    let oracle = oracle_document(bytes)?;

    if owned.pages.len() != oracle.pages.len() {
        return Err(format!(
            "page count differs: owned {} vs oracle {}",
            owned.pages.len(),
            oracle.pages.len()
        ));
    }
    for (owned_page, oracle_page) in owned.pages.iter().zip(&oracle.pages) {
        compare_words(owned_page, oracle_page)?;
    }

    let owned_document =
        table::build(owned.clone()).map_err(|error| format!("owned table: {error}"))?;
    let oracle_document = table::build(oracle).map_err(|error| format!("oracle table: {error}"))?;
    if owned_document.table != oracle_document.table {
        return Err(format!(
            "the table construction differs:\nowned {:#?}\noracle {:#?}",
            owned_document.table, oracle_document.table
        ));
    }
    if owned_document != oracle_document {
        return Err("the resulting SourceDocument differs".to_string());
    }
    Ok(())
}

/// Compare one page's words against the oracle.
fn compare_words(owned: &PageWords, oracle: &PageWords) -> Result<(), String> {
    let owned_words = sorted(&owned.words);
    let oracle_words = sorted(&oracle.words);
    if owned_words.len() != oracle_words.len() {
        return Err(format!(
            "page {} word count differs: owned {} vs oracle {}",
            owned.page,
            owned_words.len(),
            oracle_words.len()
        ) + &format!(
            "\nowned:\n{}\noracle:\n{}",
            owned_words
                .iter()
                .map(|w| format!("  {:?} {} {} {}", w.text, w.x0, w.x1, w.top))
                .collect::<Vec<_>>()
                .join("\n"),
            oracle_words
                .iter()
                .map(|w| format!("  {:?} {} {} {}", w.text, w.x0, w.x1, w.top))
                .collect::<Vec<_>>()
                .join("\n"),
        ));
    }
    for (index, (left, right)) in owned_words.iter().zip(&oracle_words).enumerate() {
        if left.text != right.text {
            return Err(format!(
                "page {} word {index} text differs: owned {:?} vs oracle {:?}",
                owned.page, left.text, right.text
            ));
        }
        if (left.x0 - right.x0).abs() > TOLERANCE || (left.top - right.top).abs() > TOLERANCE {
            return Err(format!(
                "page {} word {index} {:?} position differs: owned ({}, {}) vs oracle ({}, {})",
                owned.page, left.text, left.x0, left.top, right.x0, right.top
            ));
        }
    }
    Ok(())
}

/// Words sorted by `(top, x0, text)` with a total float order.
fn sorted(words: &[PositionedWord]) -> Vec<PositionedWord> {
    let mut words = words.to_vec();
    words.sort_by(|a, b| {
        cmp(a.top, b.top)
            .then_with(|| cmp(a.x0, b.x0))
            .then_with(|| a.text.cmp(&b.text))
    });
    words
}

/// The oracle's document payload, in the owned shape.
fn oracle_document(bytes: &[u8]) -> Result<ExtractedDocument, String> {
    let pdf = Pdf::open_bytes(bytes, None)
        .map_err(|error| format!("the oracle cannot read the fixture: {error}"))?;
    let options = WordOptions::default();
    let mut pages = Vec::new();
    for (index, page) in pdf.pages().into_iter().enumerate() {
        let page =
            page.map_err(|error| format!("the oracle cannot read page {}: {error}", index + 1))?;
        let words = page
            .extract_words(&options)
            .into_iter()
            .map(|word| positioned(&word))
            .collect();
        pages.push(PageWords {
            page: index + 1,
            words,
        });
    }
    Ok(ExtractedDocument { pages })
}

/// Convert one oracle word.
fn positioned(word: &Word) -> PositionedWord {
    PositionedWord {
        text: word.text.clone(),
        x0: word.bbox.x0,
        x1: word.bbox.x1,
        top: word.bbox.top,
    }
}

/// Total float order.
fn cmp(a: f64, b: f64) -> std::cmp::Ordering {
    a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
}

#[test]
fn multipage_plan_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_multipage_plan());
}

#[test]
fn png_none_filter_xref_stream_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::PredictedXrefStream,
        fixtures::LayoutOptions {
            png_filter: 0,
            ..fixtures::LayoutOptions::default()
        },
    ));
}

#[test]
fn png_average_filter_xref_stream_matches_the_oracle() {
    compare_with_oracle(&fixtures::build_plan_with_options(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::PredictedXrefStream,
        fixtures::LayoutOptions {
            png_filter: 3,
            ..fixtures::LayoutOptions::default()
        },
    ));
}
