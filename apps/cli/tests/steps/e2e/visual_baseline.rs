// SPDX-License-Identifier: MPL-2.0
//! The rendered visual baseline shared by every browser scenario: labels,
//! document language, keyboard focus, duplicated text, reduced motion and the
//! internal identifiers that must never reach a reader. The document language
//! is a parameter because the converter page is English and the legal notices
//! are German.
//!
//! Observation only: the values come from the driver's `read_state` with
//! `baseline: true`.

use serde_json::Value;

/// Assert the rendered visual baseline at one of the scenario's states.
pub fn assert_visual_baseline(state: &Value, displayed: &str, lang: &str) {
    let baseline = state["baseline"]
        .as_object()
        .unwrap_or_else(|| panic!("the {displayed} state has no baseline observations: {state}"));
    assert_eq!(
        baseline["lang"].as_str(),
        Some(lang),
        "the {displayed} page does not declare {lang}: {state}"
    );
    let title = baseline["title"].as_str().unwrap_or_default().trim();
    assert!(
        !title.is_empty() && title != "Document" && title != "Untitled",
        "the {displayed} page has no real title: {state}"
    );
    assert_eq!(
        baseline["h1Count"].as_u64(),
        Some(1),
        "the {displayed} page does not have exactly one h1: {state}"
    );
    let h1 = baseline["h1"].as_str().unwrap_or_default().trim();
    assert!(
        !h1.is_empty(),
        "the {displayed} page has no visible h1: {state}"
    );
    assert_eq!(
        baseline["h1Visible"].as_bool(),
        Some(true),
        "the {displayed} h1 is not visible: {state}"
    );
    let controls = baseline["controls"]
        .as_array()
        .unwrap_or_else(|| panic!("the {displayed} state reports no controls: {state}"));
    assert!(
        !controls.is_empty(),
        "the {displayed} state has no visible control: {state}"
    );
    for control in controls {
        let label = control["label"].as_str().unwrap_or_default().trim();
        assert!(
            !label.is_empty(),
            "a {} control has no visible label: {control}",
            control["tag"]
        );
        assert_eq!(
            control["focusVisible"].as_bool(),
            Some(true),
            "the {} control {label:?} has no visible keyboard focus: {control}",
            control["tag"]
        );
    }
    let duplicates = baseline["duplicates"]
        .as_array()
        .expect("the baseline reports duplicated text blocks");
    assert!(
        duplicates.is_empty(),
        "the {displayed} state repeats visible text: {duplicates:?}"
    );
    assert_eq!(
        baseline["reducedMotion"].as_str(),
        Some("0ms"),
        "the {displayed} page does not honor reduced motion: {state}"
    );

    let text = state["visibleText"]
        .as_str()
        .expect("read_state returns visible text");
    let tokens: Vec<&str> = text
        .split(|character: char| !character.is_alphanumeric() && character != '_')
        .filter(|token| !token.is_empty())
        .collect();
    for denied in ["ok", "failure", "status", "WASM"] {
        assert!(
            !tokens.contains(&denied),
            "the {displayed} page shows the internal name {denied:?}: {text}"
        );
    }
    for denied in [
        "blob:",
        "parse-error",
        "showReady",
        "acceptFile",
        "createObjectURL",
        "duty2cal_wasm",
        "wasm-inline",
    ] {
        assert!(
            !text.contains(denied),
            "the {displayed} page shows the internal identifier {denied:?}: {text}"
        );
    }
}
