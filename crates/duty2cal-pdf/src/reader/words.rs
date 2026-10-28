// SPDX-License-Identifier: MPL-2.0
//! Character boxes to positioned words, following the oracle's default word
//! rules: `x_tolerance` 3.0, `y_tolerance` 3.0, blank characters dropped and
//! Latin ligatures expanded.

use super::PositionedWord;

/// Maximum horizontal gap between characters of one word, in points.
const X_TOLERANCE: f64 = 3.0;
/// Maximum vertical distance between characters of one word, in points.
const Y_TOLERANCE: f64 = 3.0;

/// One rendered character with its page-space box.
#[derive(Debug, Clone)]
pub(crate) struct Char {
    /// The character's Unicode text.
    pub text: String,
    /// Left edge in page space.
    pub x0: f64,
    /// Right edge in page space.
    pub x1: f64,
    /// Distance from the page's top edge.
    pub top: f64,
}

/// Group characters into words the way the oracle does.
pub(crate) fn extract_words(mut chars: Vec<Char>) -> Vec<PositionedWord> {
    cluster_sort(&mut chars);

    let mut words = Vec::new();
    let mut current: Vec<Char> = Vec::new();
    for ch in chars {
        if ch.text.chars().all(char::is_whitespace) {
            flush(&mut current, &mut words);
            continue;
        }
        let split = match current.last() {
            Some(last) => {
                let x_gap = (last.x0.max(ch.x0) - last.x1.min(ch.x1)).max(0.0);
                let y_diff = (ch.top - last.top).abs();
                x_gap > X_TOLERANCE || y_diff > Y_TOLERANCE
            }
            None => false,
        };
        if split {
            flush(&mut current, &mut words);
        }
        current.push(ch);
    }
    flush(&mut current, &mut words);
    words
}

/// Finish the current word, if any.
fn flush(current: &mut Vec<Char>, words: &mut Vec<PositionedWord>) {
    if current.is_empty() {
        return;
    }
    let mut text = String::new();
    let mut x0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    let mut top = f64::INFINITY;
    for ch in current.drain(..) {
        text.push_str(&ch.text);
        x0 = x0.min(ch.x0);
        x1 = x1.max(ch.x1);
        top = top.min(ch.top);
    }
    words.push(PositionedWord {
        text: expand_ligatures(&text),
        x0,
        x1,
        top,
    });
}

/// Sort characters into line clusters, then left to right inside each line.
///
/// The oracle clusters on the top coordinate by chaining the distinct tops and
/// then stably sorts each cluster by `x0`; the source order inside a line is
/// preserved until that sort.
fn cluster_sort(chars: &mut [Char]) {
    if chars.is_empty() {
        return;
    }
    let original: Vec<usize> = (0..chars.len()).collect();
    let mut tops: Vec<f64> = chars.iter().map(|ch| ch.top).collect();
    tops.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mut band_of_top: Vec<usize> = Vec::with_capacity(tops.len());
    let mut band = 0;
    for (index, top) in tops.iter().enumerate() {
        if index > 0 && *top > tops[index - 1] + Y_TOLERANCE {
            band += 1;
        }
        band_of_top.push(band);
    }

    let bands: Vec<usize> = original
        .iter()
        .map(|&index| {
            let top = chars[index].top;
            let position = tops
                .iter()
                .position(|candidate| *candidate == top)
                .unwrap_or(0);
            band_of_top[position]
        })
        .collect();

    let mut order = original;
    order.sort_by_key(|&index| bands[index]);
    let mut sorted: Vec<Char> = order
        .into_iter()
        .map(|index| chars[index].clone())
        .collect();

    let mut starts = vec![0];
    for index in 1..sorted.len() {
        let previous = position_band(&tops, &band_of_top, sorted[index - 1].top);
        let current = position_band(&tops, &band_of_top, sorted[index].top);
        if previous != current {
            starts.push(index);
        }
    }
    starts.push(sorted.len());
    for window in starts.windows(2) {
        sorted[window[0]..window[1]]
            .sort_by(|a, b| a.x0.partial_cmp(&b.x0).unwrap_or(std::cmp::Ordering::Equal));
    }
    chars.clone_from_slice(&sorted);
}

/// The band of one top value.
fn position_band(tops: &[f64], bands: &[usize], top: f64) -> usize {
    let position = tops
        .iter()
        .position(|candidate| *candidate == top)
        .unwrap_or(0);
    bands[position]
}

/// Expand the Latin ligatures U+FB00–U+FB06.
fn expand_ligatures(text: &str) -> String {
    let mut expanded = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\u{FB00}' => expanded.push_str("ff"),
            '\u{FB01}' => expanded.push_str("fi"),
            '\u{FB02}' => expanded.push_str("fl"),
            '\u{FB03}' => expanded.push_str("ffi"),
            '\u{FB04}' => expanded.push_str("ffl"),
            '\u{FB05}' => expanded.push_str("\u{017F}t"),
            '\u{FB06}' => expanded.push_str("st"),
            other => expanded.push(other),
        }
    }
    expanded
}
