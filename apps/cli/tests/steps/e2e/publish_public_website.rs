// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions for the capability `publish-public-website`.
//!
//! The step bodies drive the real publication command into a local bare git
//! target and then the published page through the shared Playwright driver. A
//! step body is never empty: every observation asserted here was rendered by
//! Chromium from the tree the publication wrote into the target.

use std::fs;
use std::path::PathBuf;

use cucumber::{given, then};
use serde_json::{Map, Value};

use crate::steps::browser::BrowserDriver;
use crate::steps::support::{self, SITE_MEMBERS};
use crate::DutyWorld;

/// The statement names the page carries: six above the converter panel, two
/// below it, and the footer's Built revision.
const STATEMENT_NAMES: [&str; 9] = [
    "what-it-does",
    "browser-route",
    "archive-route",
    "privacy",
    "non-affiliation",
    "as-is",
    "source",
    "license",
    "built-revision",
];

/// The @e2e scenario's committed visual evidence directory.
fn evidence_dir() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/publish-public-website");
    let archived = root.join("givn/archive/publish-public-website");
    // The archive gate moves the change before it runs the suite, so write to
    // the archived copy in that phase; the active change uses its own
    // directory.
    let base = if active.join("proposal.md").is_file() {
        active
    } else {
        archived
    };
    base.join(
        "evidence/visual/\
         release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page",
    )
}

/// The published revision the scenario recorded, and the text the fixture's
/// own history yields for it.
fn published_revision(w: &mut DutyWorld) -> (String, String) {
    let revision = support::site_get(w, "revision")
        .expect("the scenario recorded no revision to publish");
    let describe = support::site_describe(w, &revision);
    (revision, describe)
}

/// The browser driver for the published target's checked-out tree, started on
/// first use. `DUTY2CAL_SITE_ROOT` names that tree to the driver, so the page
/// under test is the one the hosting target holds.
fn browser(w: &mut DutyWorld) -> &mut BrowserDriver {
    if w.browser.is_none() {
        let root = support::site_target_checkout(w);
        let mut driver = BrowserDriver::start_serving(&root);
        let started = driver.send("start", Map::new());
        let url = started["url"]
            .as_str()
            .unwrap_or_else(|| panic!("the driver started no page: {started}"));
        assert!(
            url.starts_with("http://127.0.0.1:"),
            "the driver did not serve the hosting target on loopback: {started}"
        );
        w.browser = Some(driver);
    }
    w.browser
        .as_mut()
        .expect("the browser driver is running after start")
}

/// Read the rendered page, optionally with the visual-baseline and site
/// surfaces observations.
fn read_state(w: &mut DutyWorld, baseline: bool, site: bool) -> Value {
    let mut params = Map::new();
    if baseline {
        params.insert("baseline".to_string(), Value::Bool(true));
    }
    if site {
        params.insert("site".to_string(), Value::Bool(true));
    }
    browser(w).send("read_state", params)
}

/// The `site` observation object of a `read_state` result.
fn site_facts(state: &Value) -> &Value {
    state
        .get("site")
        .unwrap_or_else(|| panic!("read_state returned no site surfaces: {state}"))
}

/// The names of the statements inside one panel, in document order.
fn statement_names(panel: &Value) -> Vec<String> {
    panel["statements"]
        .as_array()
        .unwrap_or_else(|| panic!("the panel reports no statements: {panel}"))
        .iter()
        .map(|statement| {
            statement["name"]
                .as_str()
                .unwrap_or_else(|| panic!("a statement has no name: {statement}"))
                .to_string()
        })
        .collect()
}

/// The statement carrying `name`.
fn statement<'a>(panel: &'a Value, name: &str) -> &'a Value {
    panel["statements"]
        .as_array()
        .unwrap_or_else(|| panic!("the panel reports no statements: {panel}"))
        .iter()
        .find(|statement| statement["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("the panel carries no {name} statement: {panel}"))
}

/// Assert the page header: the 48×48 project mark labeled "duty2cal", left of
/// the heading and subheadline, and the same image as the tab icon.
fn assert_header(site: &Value, width: f64, height: f64, alt: &str) {
    let header = &site["header"];
    assert_eq!(
        header["iconAlt"].as_str(),
        Some(alt),
        "the header mark's alternative text differs: {header}"
    );
    let icon = &header["iconBox"];
    let icon_width = icon["width"]
        .as_f64()
        .unwrap_or_else(|| panic!("the header mark has no width: {header}"));
    let icon_height = icon["height"]
        .as_f64()
        .unwrap_or_else(|| panic!("the header mark has no height: {header}"));
    assert!(
        (icon_width - width).abs() < 0.5 && (icon_height - height).abs() < 0.5,
        "the header mark is not rendered {width} by {height}: {header}"
    );
    assert_eq!(
        header["iconLeftOfHeading"].as_bool(),
        Some(true),
        "the header mark is not left of the page heading: {header}"
    );
    assert_eq!(
        header["iconLeftOfSubheadline"].as_bool(),
        Some(true),
        "the header mark is not left of the subheadline: {header}"
    );
    assert_eq!(
        header["iconOverlapsHeadingRow"].as_bool(),
        Some(true),
        "the header mark is not aligned with the heading row: {header}"
    );

    let icon_src = header["iconSrc"]
        .as_str()
        .unwrap_or_else(|| panic!("the header mark has no source: {header}"));
    let tab = &site["tabIcon"];
    assert_eq!(
        tab["rel"].as_str(),
        Some("icon"),
        "the page declares no tab icon: {tab}"
    );
    assert_eq!(
        tab["href"].as_str(),
        Some(icon_src),
        "the tab icon is not the same image as the header mark: {site}"
    );
    assert!(
        icon_src.ends_with("/icon.png"),
        "the header mark is not the published icon.png: {icon_src}"
    );
}

/// Assert the "Before you choose a plan" panel above the picker.
fn assert_terms_panel(site: &Value) {
    let terms = &site["terms"];
    assert_eq!(
        terms["heading"].as_str(),
        Some("Before you choose a plan"),
        "the terms panel does not carry its heading: {site}"
    );
    assert_eq!(
        statement_names(terms),
        vec![
            "what-it-does",
            "browser-route",
            "archive-route",
            "privacy",
            "non-affiliation",
            "as-is"
        ],
        "the terms panel's statements differ: {terms}"
    );
    for statement in terms["statements"]
        .as_array()
        .expect("the terms panel reports statements")
    {
        let text = statement["text"].as_str().unwrap_or_default().trim();
        assert!(
            !text.is_empty(),
            "the {} statement carries no text: {statement}",
            statement["name"]
        );
    }

    let archive = statement(terms, "archive-route");
    let text = archive["text"].as_str().unwrap_or_default();
    assert!(
        text.contains("Downloads for various operating systems are available on request"),
        "the archive route does not offer downloads for various operating systems: {archive}"
    );
    assert!(
        text.contains("release page carries the published builds"),
        "the archive route does not name the release page's builds: {archive}"
    );
    let release = archive["links"]
        .as_array()
        .expect("the archive route reports its links")
        .iter()
        .find(|link| link["href"].as_str() == Some("https://github.com/duty2cal/duty2cal/releases"))
        .unwrap_or_else(|| panic!("the archive route does not link the release page: {archive}"));
    assert!(
        release["text"]
            .as_str()
            .unwrap_or_default()
            .contains("release page"),
        "the release link does not name the release page: {release}"
    );

    assert_eq!(
        terms["abovePicker"].as_bool(),
        Some(true),
        "the terms panel is not above the converter panel: {terms}"
    );
}

/// Assert the "Source and licence" panel and the footer prose.
fn assert_licence_and_footer(site: &Value) {
    let licence = &site["licence"];
    assert_eq!(
        licence["heading"].as_str(),
        Some("Source and licence"),
        "the source and licence panel does not carry its heading: {site}"
    );
    assert_eq!(
        statement_names(licence),
        vec!["source", "license"],
        "the source and licence panel's statements differ: {licence}"
    );
    let source = statement(licence, "source");
    assert!(
        source["text"]
            .as_str()
            .unwrap_or_default()
            .contains("github.com/duty2cal/duty2cal"),
        "the source statement names no repository: {source}"
    );
    let license = statement(licence, "license");
    assert!(
        license["text"]
            .as_str()
            .unwrap_or_default()
            .contains("MPL-2.0"),
        "the licence statement names no licence: {license}"
    );
    assert_eq!(
        licence["belowPicker"].as_bool(),
        Some(true),
        "the source and licence panel is not below the converter panels: {licence}"
    );
    assert_eq!(
        licence["aboveFooter"].as_bool(),
        Some(true),
        "the source and licence panel is not above the footer: {licence}"
    );

    let footer = &site["footer"];
    let footer_names: Vec<&str> = footer["statements"]
        .as_array()
        .expect("the footer reports its statement attributes")
        .iter()
        .map(|name| name.as_str().expect("a footer statement name is text"))
        .collect();
    assert_eq!(
        footer_names,
        vec!["built-revision"],
        "the footer repeats a statement attribute: {footer}"
    );
    let text = footer["text"].as_str().unwrap_or_default();
    for prose in [
        "Not affiliated with any duty-plan provider or airline",
        "Provided as is, without warranty",
        "No account, email or upload is needed",
        "sends no request after it loads",
    ] {
        assert!(
            text.contains(prose),
            "the footer prose does not repeat {prose:?}: {footer}"
        );
    }

    let document = site["documentStatements"]
        .as_array()
        .expect("the page reports its statement attributes");
    assert_eq!(
        document.len(),
        STATEMENT_NAMES.len(),
        "the page carries another statement count: {site}"
    );
    for name in STATEMENT_NAMES {
        let count = document
            .iter()
            .filter(|entry| entry.as_str() == Some(name))
            .count();
        assert_eq!(
            count, 1,
            "the page carries {count} {name} statement attributes: {document:?}"
        );
    }
}

/// Assert the footer's Built revision text and commit link.
fn assert_footer_revision(site: &Value, describe: &str, commit: &str) {
    let footer = &site["footer"];
    assert_eq!(
        footer["revisionText"].as_str(),
        Some(describe),
        "the footer does not name the Built revision the fixture's history yields: {footer}"
    );
    let expected = format!("https://github.com/duty2cal/duty2cal/commit/{commit}");
    assert_eq!(
        footer["revisionHref"].as_str(),
        Some(expected.as_str()),
        "the footer does not link the built commit: {footer}"
    );
}

/// Assert the rendered visual baseline at one of the scenario's states.
fn assert_visual_baseline(state: &Value, displayed: &str) {
    let baseline = state["baseline"]
        .as_object()
        .unwrap_or_else(|| panic!("the {displayed} state has no baseline observations: {state}"));
    assert_eq!(
        baseline["lang"].as_str(),
        Some("en"),
        "the {displayed} page does not declare English: {state}"
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
    assert!(
        !baseline["h1"].as_str().unwrap_or_default().trim().is_empty(),
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

    let tokens = &site_facts(state)["tokens"];
    assert_eq!(
        tokens["motionDuration"].as_str(),
        Some("0ms"),
        "the {displayed} page does not collapse its motion token: {tokens}"
    );
    assert_eq!(
        tokens["radius"].as_str(),
        Some("8px"),
        "the {displayed} page does not carry the design system's radius: {tokens}"
    );
    assert_eq!(
        tokens["space"].as_str(),
        Some("1rem"),
        "the {displayed} page does not carry the design system's spacing: {tokens}"
    );
}

/// Run axe on the current rendered state and fail on any violation.
fn assert_axe_clean(w: &mut DutyWorld, displayed: &str) {
    let result = browser(w).send("axe", Map::new());
    let violations = result["violations"]
        .as_array()
        .expect("axe returns a violations array");
    assert!(
        violations.is_empty(),
        "the {displayed} state has accessibility violations: {violations:?}"
    );
}

/// Capture the committed screenshots for one displayed state at both
/// viewports.
fn capture_screenshots(w: &mut DutyWorld, empty: bool) {
    let dir = evidence_dir();
    fs::create_dir_all(&dir).expect("create the visual evidence directory");
    for (width, height, viewport) in [(360_u32, 780_u32, "mobile"), (1920, 1080, "desktop")] {
        let path = dir.join(if empty {
            format!("{viewport}--empty.png")
        } else {
            format!("{viewport}.png")
        });
        let mut params = Map::new();
        params.insert(
            "path".to_string(),
            Value::String(path.display().to_string()),
        );
        params.insert("width".to_string(), Value::from(width));
        params.insert("height".to_string(), Value::from(height));
        params.insert("fullPage".to_string(), Value::Bool(true));
        let result = browser(w).send("screenshot", params);
        let saved_to = path.display().to_string();
        assert_eq!(
            result["savedTo"].as_str(),
            Some(saved_to.as_str()),
            "the driver did not capture {saved_to}: {result}"
        );
        assert!(path.is_file(), "the screenshot {saved_to} was not written");
    }
}

// ---------------------------------------------------------------------------
// Scenario steps.
// ---------------------------------------------------------------------------

#[given("a hosting target whose default branch serves an earlier page")]
async fn a_hosting_target_with_an_earlier_page(w: &mut DutyWorld) {
    let url = support::site_set_up_target_from(w, "v0.5.0");
    assert!(
        url.starts_with("http://127.0.0.1:"),
        "the fixture's serving address is not loopback: {url}"
    );
    assert_eq!(
        support::site_get(w, "base-describe").as_deref(),
        Some("v0.5.0"),
        "the earlier page was not built from the release tag"
    );
    let mut earlier = support::site_target_members(w);
    earlier.sort();
    let mut expected: Vec<String> = SITE_MEMBERS.iter().map(|member| member.to_string()).collect();
    expected.sort();
    assert_eq!(
        earlier, expected,
        "the earlier page does not hold the Site output"
    );
}

#[given(regex = r#"^a source revision three revisions after the release tag "([^"]+)"$"#)]
async fn a_source_revision_after_the_tag(w: &mut DutyWorld, tag: String) {
    let repo = support::site_repository_path(w);
    let distance = std::process::Command::new("git")
        .arg("-C")
        .arg(&repo)
        .args(["rev-list", "--count", &format!("{tag}..HEAD")])
        .output()
        .expect("count the fixture's commits after the release tag");
    assert!(
        distance.status.success(),
        "the fixture cannot count its history: {}",
        String::from_utf8_lossy(&distance.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&distance.stdout).trim(),
        "3",
        "the fixture's source revision is not three revisions after {tag}"
    );
    let revision = support::site_resolve(w, "HEAD");
    let describe = support::site_describe(w, &revision);
    assert!(
        describe.starts_with(&format!("{tag}-3-g")),
        "the fixture's own history yields {describe:?}, not three revisions after {tag}"
    );
    support::site_set(w, "revision", &revision);
}

#[then("the hosting target holds the Site output built from that revision")]
async fn the_target_holds_the_output(w: &mut DutyWorld) {
    let (revision, describe) = published_revision(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the publication failed:\n{}",
        w.command_transcript
    );
    assert!(
        w.stdout
            .contains(&format!("publish-site: published {describe}")),
        "the publication did not report publishing this Built revision:\n{}",
        w.command_transcript
    );

    let mut target = support::site_target_members(w);
    target.sort();
    let mut output = support::site_output_members(w);
    output.sort();
    assert_eq!(
        target, output,
        "the hosting target holds a different tree than the Site output"
    );
    let page = support::site_target_file(w, "index.html");
    assert!(
        page.contains(&format!("data-site-statement=\"built-revision\">{describe}<")),
        "the hosting target's page does not carry the Built revision {describe:?}"
    );
    assert!(
        page.contains(&format!("/commit/{}", support::site_resolve(w, &revision))),
        "the hosting target's page does not link the built commit"
    );
}

#[then(
    regex = r#"^the hosting target holds exactly the page, the capability gate, the script, the stylesheet, the icon, "([^"]+)", "([^"]+)" and "([^"]+)"$"#
)]
async fn the_target_holds_exactly_the_members(w: &mut DutyWorld, first: String, second: String, third: String) {
    let mut members = support::site_target_members(w);
    members.sort();
    let mut expected: Vec<String> = SITE_MEMBERS.iter().map(|member| member.to_string()).collect();
    expected.sort();
    assert_eq!(
        members, expected,
        "the hosting target holds another member list than the Site output"
    );
    for named in [first, second, third] {
        assert!(
            members.contains(&named),
            "the hosting target holds no {named}: {members:?}"
        );
    }
    for member in [
        "index.html",
        "capability.js",
        "app.js",
        "styles.css",
        "icon.png",
    ] {
        assert!(
            members.iter().any(|entry| entry == member),
            "the hosting target holds no {member}: {members:?}"
        );
    }
}

#[then("the earlier page has been replaced as a whole")]
async fn the_earlier_page_was_replaced(w: &mut DutyWorld) {
    let history = support::site_target_history(w);
    assert_eq!(
        history.len(),
        2,
        "the hosting target's history is not the earlier page and its replacement: {history:?}"
    );
    let tip = support::site_target_tip(w);
    assert_eq!(tip, history[0], "the target's tip is not its newest commit");
    assert_eq!(
        support::site_target_parents(w, &tip),
        vec![history[1].clone()],
        "the replacement is not the earlier page's single fast-forward commit"
    );

    let earlier_revision =
        support::site_get(w, "base-revision").expect("the scenario recorded no earlier revision");
    let earlier_describe = support::site_describe(w, &earlier_revision);
    let earlier_page = support::site_target_git(w, &["show", &format!("{}:index.html", history[1])]);
    assert!(
        earlier_page.contains(&format!("data-site-statement=\"built-revision\">{earlier_describe}<")),
        "the earlier page does not carry its own Built revision"
    );

    let (_, describe) = published_revision(w);
    assert_ne!(
        earlier_describe, describe,
        "the earlier page and the served page name the same Built revision"
    );
    let page = support::site_target_file(w, "index.html");
    assert!(
        !page.contains(&format!("data-site-statement=\"built-revision\">{earlier_describe}<")),
        "the earlier page's Built revision survives in the hosting target"
    );
}

#[then(
    "the served address and the footer name the Built revision that revision's own history yields, and the footer links that commit"
)]
async fn the_served_address_and_footer_name_the_revision(w: &mut DutyWorld) {
    let (revision, describe) = published_revision(w);
    let commit = support::site_resolve(w, &revision);

    let served = support::site_served_page(w);
    assert_eq!(
        support::served_revision(&served).as_deref(),
        Some(describe.as_str()),
        "the served address does not name the Built revision {describe:?}"
    );
    assert!(
        served.contains(&format!("/commit/{commit}")),
        "the served page does not link the built commit {commit}"
    );

    let state = read_state(w, false, true);
    assert_footer_revision(site_facts(&state), &describe, &commit);
}

#[then(
    regex = r#"^the page header shows the project icon at ([\d.]+) by ([\d.]+) with the alternative text "([^"]+)", left of the page heading and its subheadline, and the page's tab carries the same icon$"#
)]
async fn the_page_header_shows_the_icon(
    w: &mut DutyWorld,
    width: String,
    height: String,
    alt: String,
) {
    let width: f64 = width
        .parse()
        .unwrap_or_else(|error| panic!("the icon width {width:?} is not a number: {error}"));
    let height: f64 = height
        .parse()
        .unwrap_or_else(|error| panic!("the icon height {height:?} is not a number: {error}"));
    let state = read_state(w, false, true);
    assert_header(site_facts(&state), width, height, &alt);
}

#[then(
    "a panel above the converter panel states what the tool does, the browser route, the archive route, the privacy rule, the non-affiliation statement and the as-is statement before the page asks for a file, the archive route saying that downloads for various operating systems are available on request and that the release page carries the published builds"
)]
async fn the_terms_panel_states_the_terms(w: &mut DutyWorld) {
    let state = read_state(w, false, true);
    assert_terms_panel(site_facts(&state));
}

#[then(
    "a panel below the converter panels states the source and the licence, and the footer repeats the non-affiliation, as-is and privacy terms in prose without repeating any statement the panels make"
)]
async fn the_source_panel_and_footer_state_the_terms(w: &mut DutyWorld) {
    let state = read_state(w, false, true);
    assert_licence_and_footer(site_facts(&state));
}

#[then("the served page has sent no request to another origin since it loaded")]
async fn no_request_to_another_origin(w: &mut DutyWorld) {
    let state = read_state(w, false, false);
    let initial = state["initialAddress"]["url"]
        .as_str()
        .expect("read_state returns the initial address");
    let origin: String = initial.split('/').take(3).collect::<Vec<_>>().join("/");
    let log = state["requestLog"]
        .as_array()
        .expect("read_state returns the request log");
    assert!(
        !log.is_empty(),
        "the request log proves nothing because the page requested nothing: {state}"
    );
    for entry in log {
        let url = entry["url"].as_str().unwrap_or_default();
        assert!(
            url.starts_with(&origin),
            "the page requested another origin {url:?} (page origin {origin}): {state}"
        );
    }
}

#[then("the served page shows its empty state before a plan file is offered")]
async fn the_page_shows_its_empty_state(w: &mut DutyWorld) {
    let state = read_state(w, false, false);
    assert_eq!(
        state["state"].as_str(),
        Some("empty"),
        "the page did not show its empty state: {state}"
    );
    assert!(
        state["status"]
            .as_str()
            .unwrap_or_default()
            .contains("No plan selected."),
        "the empty status is not visible: {state}"
    );
    assert!(
        state["offers"]
            .as_array()
            .expect("read_state returns offers")
            .is_empty(),
        "the empty state offers a download: {state}"
    );
}

#[then("the served page shows its ready state once a supported plan file is offered")]
async fn the_page_shows_its_ready_state(w: &mut DutyWorld) {
    let directory = w.working_dir().join("plan");
    fs::create_dir_all(&directory).expect("create the plan fixture directory");
    let source = directory.join("roster.pdf");
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (support::DATE_X, "2099-06-07"),
            (support::ACTIVITY_X, "LU742"),
            (support::FROM_X, "Tychon"),
            (support::TO_X, "Mareum"),
            (support::START_X, "09:25"),
            (support::END_X, "15:45"),
        ],
    ));
    fs::write(&source, support::build_pdf(&items)).expect("write the supported plan");

    let mut params = Map::new();
    params.insert(
        "path".to_string(),
        Value::String(source.display().to_string()),
    );
    browser(w).send("set_file", params);

    let state = read_state(w, false, false);
    assert_eq!(
        state["state"].as_str(),
        Some("ready"),
        "the page did not show its ready state after the supported plan: {state}"
    );
    assert!(
        state["status"]
            .as_str()
            .unwrap_or_default()
            .contains("Your calendar is ready to save."),
        "the ready status is not visible: {state}"
    );
    let offers = state["offers"].as_array().expect("read_state returns offers");
    assert!(
        offers.iter().any(|offer| {
            offer["kind"].as_str() == Some("calendar")
                && offer["name"].as_str().is_some_and(|name| name.ends_with(".ics"))
        }),
        "the ready state offers no calendar download: {state}"
    );
}

#[then(
    "the served page meets the visual baseline in the empty state and in the ready state it traverses"
)]
async fn the_page_meets_the_visual_baseline(w: &mut DutyWorld) {
    let (revision, describe) = published_revision(w);
    let commit = support::site_resolve(w, &revision);

    // The ready state is the one the scenario just rendered.
    let ready = read_state(w, true, true);
    assert_eq!(
        ready["state"].as_str(),
        Some("ready"),
        "the visual baseline ran outside the ready state: {ready}"
    );
    assert_visual_baseline(&ready, "ready");
    let site = site_facts(&ready);
    assert_header(site, 48.0, 48.0, "duty2cal");
    assert_terms_panel(site);
    assert_licence_and_footer(site);
    assert_footer_revision(site, &describe, &commit);
    assert_axe_clean(w, "ready");
    capture_screenshots(w, false);

    // Return to the empty state the scenario displayed first and assert it too.
    browser(w).send("reload", Map::new());
    let empty = read_state(w, true, true);
    assert_eq!(
        empty["state"].as_str(),
        Some("empty"),
        "the page did not return to the empty state for its capture: {empty}"
    );
    assert_visual_baseline(&empty, "empty");
    let site = site_facts(&empty);
    assert_header(site, 48.0, 48.0, "duty2cal");
    assert_terms_panel(site);
    assert_licence_and_footer(site);
    assert_footer_revision(site, &describe, &commit);
    assert_axe_clean(w, "empty");
    capture_screenshots(w, true);
}
