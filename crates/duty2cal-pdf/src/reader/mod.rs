// SPDX-License-Identifier: MPL-2.0
//! Owned extraction: supplied bytes to positioned words per page.
//!
//! The reading boundary is the positioned word. The table construction consumes
//! this payload and is the one implementation used by the executable and the
//! browser.

mod cmap;
mod content;
mod document;
mod font;
mod structure;
mod words;

pub use document::extract;

/// A word with its text and page-space position, in points.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionedWord {
    /// The word's Unicode text, ligatures expanded.
    pub text: String,
    /// Left edge in page space.
    pub x0: f64,
    /// Right edge in page space.
    pub x1: f64,
    /// Distance from the page's top edge.
    pub top: f64,
}

/// One page's positioned words, in extraction order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageWords {
    /// 1-based page number in page-tree order.
    pub page: usize,
    /// The page's words.
    pub words: Vec<PositionedWord>,
}

/// A supplied document's readable pages.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExtractedDocument {
    /// Pages in page-tree order.
    pub pages: Vec<PageWords>,
}
