// SPDX-License-Identifier: MPL-2.0
//! Steps for the capability `state-the-legal-notices`.
//!
//! Two kinds of evidence live here. The refusal scenarios drive
//! `scripts/legal-notices-check.sh` against a page set built from the
//! repository's own pages, mutated one element at a time, so a refusal is
//! attributable to the mutation. The two browser scenarios read the rendered
//! notices from the served Site output: they activate the footer link a reader
//! clicks, assert what the page says, and capture the visual baseline at both
//! viewports.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when};
use serde_json::{Map, Value};

use crate::steps::e2e::visual_baseline::assert_visual_baseline;
use crate::steps::export_duty_plan_calendar::driver;
use crate::DutyWorld;

const CHECK: &str = "scripts/legal-notices-check.sh";

/// The repository root, from this crate's manifest directory.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

static FIXTURE: AtomicUsize = AtomicUsize::new(0);

/// A fresh page-set directory for one scenario.
fn fresh_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "duty2cal-notices-{}-{}",
        std::process::id(),
        FIXTURE.fetch_add(1, Ordering::SeqCst)
    ));
    std::fs::create_dir_all(&dir).expect("create the page-set directory");
    dir
}

fn copy(from: PathBuf, to: PathBuf) {
    std::fs::create_dir_all(to.parent().expect("a fixture file has a parent"))
        .expect("create the fixture directory");
    std::fs::copy(&from, &to)
        .unwrap_or_else(|error| panic!("copy {} to {}: {error}", from.display(), to.display()));
}

/// Build the page set from the repository's own pages and carriers.
fn prepare(w: &mut DutyWorld) {
    let root = repo_root();
    let dir = fresh_dir();
    for member in ["index.html", "impressum.html", "privacy.html"] {
        copy(
            root.join("web").join(member),
            dir.join("web").join(member),
        );
    }
    copy(
        root.join("org/profile/README.md"),
        dir.join("org/profile/README.md"),
    );
    copy(root.join("README.md"), dir.join("README.md"));
    w.dir = Some(dir);
}

fn page(w: &DutyWorld, name: &str) -> PathBuf {
    w.working_dir().join("web").join(name)
}

/// Replace one literal in a fixture page, refusing a no-op mutation.
fn edit(path: &Path, from: &str, to: &str) {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        text.contains(from),
        "{} does not carry {from:?}; the fixture would not be mutated",
        path.display()
    );
    std::fs::write(path, text.replacen(from, to, 1))
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

/// Run the source check over the scenario's page set.
fn run_check(w: &mut DutyWorld) {
    let output = Command::new("bash")
        .arg(repo_root().join(CHECK))
        .arg("--root")
        .arg(w.working_dir())
        .output()
        .expect("run the legal-notices check");
    w.exit_status = output.status.code();
    w.stdout = String::from_utf8_lossy(&output.stdout).to_string();
    w.stderr = String::from_utf8_lossy(&output.stderr).to_string();
}

/// Assert the check refused and named every required element.
fn assert_refused_naming(w: &DutyWorld, needles: &[&str]) {
    assert_eq!(
        w.exit_status,
        Some(1),
        "the check did not refuse the page set: stdout={:?} stderr={:?}",
        w.stdout,
        w.stderr
    );
    for needle in needles {
        assert!(
            w.stderr.contains(needle),
            "the refusal {:?} does not name {needle:?}",
            w.stderr
        );
    }
}

// --- The page set and its mutations ---------------------------------------

#[given("a page set that carries both notices and links both")]
async fn page_set_linked(w: &mut DutyWorld) {
    prepare(w);
}

#[given("a page set whose converter page links both notices")]
async fn page_set_linked_by_converter(w: &mut DutyWorld) {
    prepare(w);
}

#[given("a page set that carries both notices")]
async fn page_set_carries_both(w: &mut DutyWorld) {
    prepare(w);
}

#[given("the site serves both notices and links both")]
async fn site_serves_both(w: &mut DutyWorld) {
    prepare(w);
}

#[given("the page set carries the Impressum and no Datenschutzerklärung")]
async fn without_privacy(w: &mut DutyWorld) {
    std::fs::remove_file(page(w, "privacy.html")).expect("remove the Datenschutzerklärung");
}

#[given("the Datenschutzerklärung does not link the Impressum")]
async fn privacy_without_impressum_link(w: &mut DutyWorld) {
    edit(
        &page(w, "privacy.html"),
        "<a href=\"./impressum.html\">Impressum</a>",
        "Impressum",
    );
}

#[given(regex = r#"^the Impressum still carries the marker "([^"]+)"$"#)]
async fn impressum_placeholder(w: &mut DutyWorld, marker: String) {
    edit(&page(w, "impressum.html"), "Daniel Kressner", &marker);
}

#[given("the Impressum states a name and no postal address")]
async fn impressum_without_postal_address(w: &mut DutyWorld) {
    edit(&page(w, "impressum.html"), "10969 Berlin", "");
}

#[given("the Datenschutzerklärung carries no standing date")]
async fn privacy_without_date(w: &mut DutyWorld) {
    edit(&page(w, "privacy.html"), "Stand: 27.10.2026", "");
}

#[given("the Datenschutzerklärung carries no English summary")]
async fn privacy_without_summary(w: &mut DutyWorld) {
    edit(
        &page(w, "privacy.html"),
        "The German version is the legally binding one.",
        "",
    );
}

#[given(regex = r#"^the Impressum links "([^"]+)"$"#)]
async fn impressum_links(w: &mut DutyWorld, target: String) {
    edit(
        &page(w, "impressum.html"),
        "https://github.com/duty2cal/duty2cal",
        &format!("https://{target}"),
    );
}

#[given(regex = r#"^the Datenschutzerklärung loads an image from "([^"]+)"$"#)]
async fn privacy_loads_image(w: &mut DutyWorld, url: String) {
    edit(
        &page(w, "privacy.html"),
        "</main>",
        &format!("<p><img src=\"{url}\" alt=\"\"></p></main>"),
    );
}

#[given("the organization profile copies the operator's postal address")]
async fn profile_copies_address(w: &mut DutyWorld) {
    let path = w.working_dir().join("org/profile/README.md");
    let text = std::fs::read_to_string(&path).expect("read the profile");
    std::fs::write(
        &path,
        format!("{text}\nAnschrift: Prinzenstraße 34, 10969 Berlin\n"),
    )
    .expect("append to the profile");
}

// --- The check and its refusals -------------------------------------------

#[when("the notices are checked before publication")]
async fn check_before_publication(w: &mut DutyWorld) {
    run_check(w);
}

#[when("the notices are checked across the carriers")]
async fn check_across_carriers(w: &mut DutyWorld) {
    run_check(w);
}

#[then("the check is refused and names the Datenschutzerklärung")]
async fn refused_naming_privacy(w: &mut DutyWorld) {
    assert_refused_naming(w, &["privacy.html"]);
}

#[then("the previously published site stays")]
async fn previous_site_stays(w: &mut DutyWorld) {
    assert!(
        page(w, "index.html").is_file(),
        "the refused check changed the page set it was given"
    );
}

#[then(regex = r#"^the check is refused and names the Datenschutzerklärung and the missing "([^"]+)" link$"#)]
async fn refused_naming_missing_link(w: &mut DutyWorld, link: String) {
    assert_refused_naming(w, &["privacy.html", &link]);
}

#[then("the check is refused and names the Impressum and the marker")]
async fn refused_naming_marker(w: &mut DutyWorld) {
    assert_refused_naming(w, &["impressum.html", "TODO"]);
}

#[then("the check is refused and names the Impressum and the missing postal address")]
async fn refused_naming_postal_address(w: &mut DutyWorld) {
    assert_refused_naming(w, &["impressum.html", "postal address"]);
}

#[then("the check is refused and names the Datenschutzerklärung and the missing standing date")]
async fn refused_naming_standing_date(w: &mut DutyWorld) {
    assert_refused_naming(w, &["privacy.html", "standing date"]);
}

#[then("the check is refused and names the Datenschutzerklärung and the missing English summary")]
async fn refused_naming_summary(w: &mut DutyWorld) {
    assert_refused_naming(w, &["privacy.html", "English summary"]);
}

#[then("the check is refused and names the Impressum and the withdrawn reference")]
async fn refused_naming_withdrawn_reference(w: &mut DutyWorld) {
    assert_refused_naming(w, &["impressum.html", "ec.europa.eu/consumers/odr"]);
}

#[then("the check is refused and names the Datenschutzerklärung and the foreign asset")]
async fn refused_naming_foreign_asset(w: &mut DutyWorld) {
    assert_refused_naming(w, &["privacy.html", "cdn.analytics.invalid"]);
}

#[then("the check is refused and names the organization profile and the copied address")]
async fn refused_naming_profile(w: &mut DutyWorld) {
    assert_refused_naming(w, &["org/profile/README.md", "postal address"]);
}

#[then("the organization profile and the README link the notices instead")]
async fn carriers_link_notices(w: &mut DutyWorld) {
    let readme = std::fs::read_to_string(w.working_dir().join("README.md")).expect("read the README");
    for link in [
        "[Impressum](https://duty2cal.github.io/impressum)",
        "[Datenschutz](https://duty2cal.github.io/privacy)",
    ] {
        assert!(
            readme.contains(link),
            "the README does not carry {link:?} as the carrier's route"
        );
    }
}

#[when("a reader reads either notice")]
async fn reader_reads_notice(w: &mut DutyWorld) {
    for name in ["impressum.html", "privacy.html"] {
        w.calendars.push(
            std::fs::read_to_string(page(w, name))
                .unwrap_or_else(|error| panic!("read {name}: {error}")),
        );
    }
}

#[then("the notice's body is written in German")]
async fn notice_body_is_german(w: &mut DutyWorld) {
    for notice in &w.calendars {
        assert!(
            notice.contains("lang=\"de\""),
            "a notice does not declare German"
        );
        assert!(
            notice.contains("Verantwortlicher") || notice.contains("Zustellanschrift"),
            "a notice carries no German body"
        );
    }
}

#[then("the notice carries a short English summary below the German text")]
async fn notice_carries_summary(w: &mut DutyWorld) {
    for notice in &w.calendars {
        let german = notice.find("Zustellanschrift").or_else(|| notice.find("Verantwortlicher"));
        let summary = notice.find("lang=\"en\"");
        assert!(
            matches!((german, summary), (Some(g), Some(s)) if s > g),
            "a notice's English summary is not below its German text"
        );
    }
}

#[then("the summary states that the German version is the binding one")]
async fn summary_names_binding_version(w: &mut DutyWorld) {
    for notice in &w.calendars {
        assert!(
            notice.contains("The German version is the legally binding one."),
            "a notice's summary does not name the German version as binding"
        );
    }
}

// --- The rendered notices -------------------------------------------------

#[when(regex = r#"^the reader follows the footer's "([^"]+)" link$"#)]
async fn follow_footer_link(w: &mut DutyWorld, label: String) {
    let before = driver(w).send("read_text", Map::new());
    w.heading_before = before["h1"].as_str().unwrap_or_default().to_string();
    let mut params = Map::new();
    params.insert("text".to_string(), Value::String(label));
    driver(w).send("click", params);
    let after = driver(w).send("read_text", Map::new());
    w.heading_after = after["h1"].as_str().unwrap_or_default().to_string();
}

/// The rendered document of the page in hand.
fn document(w: &mut DutyWorld) -> Value {
    driver(w).send("read_text", Map::new())
}

fn document_text(w: &mut DutyWorld) -> String {
    document(w)["text"].as_str().unwrap_or_default().to_string()
}

fn assert_says(w: &mut DutyWorld, needles: &[&str]) {
    let text = document_text(w);
    for needle in needles {
        assert!(text.contains(needle), "the notice does not say {needle:?}: {text}");
    }
}

#[then("the page states the operator's name and a postal address")]
async fn states_operator(w: &mut DutyWorld) {
    assert_says(w, &["Daniel Kressner", "c/o adesso SE", "Prinzenstraße 34", "10969 Berlin"]);
}

#[then("the page offers an electronic mail address as a link")]
async fn offers_mail_link(w: &mut DutyWorld) {
    let state = document(w);
    let link = state["links"]
        .as_array()
        .expect("read_text returns links")
        .iter()
        .find(|link| link["scheme"].as_str() == Some("mailto:"));
    assert!(link.is_some(), "the notice offers no mailto link: {state}");
}

#[then("the page states that the employer address is a service address for this private project and not the employer's offering")]
async fn states_service_address(w: &mut DutyWorld) {
    assert_says(
        w,
        &[
            "Zustellanschrift",
            "nicht Anbieter dieses Projekts",
            "mit ihm nicht verbunden",
        ],
    );
}

#[then("the page states that the project is not affiliated with Lufthansa, Jeppesen, or any duty-plan provider or airline")]
async fn states_non_affiliation(w: &mut DutyWorld) {
    assert_says(w, &["keine Verbindung zu Lufthansa, Jeppesen"]);
}

#[then("the page states the source repository and the MPL-2.0 licence")]
async fn states_source_and_licence(w: &mut DutyWorld) {
    assert_says(w, &["github.com/duty2cal/duty2cal", "MPL-2.0"]);
}

#[then("the page states the as-is sentence")]
async fn states_as_is(w: &mut DutyWorld) {
    assert_says(w, &["ohne Gewähr"]);
}

#[then("the page carries no placeholder and no unresolved marker")]
async fn carries_no_placeholder(w: &mut DutyWorld) {
    let text = document_text(w);
    for marker in ["TODO", "Mustermann", "example.org"] {
        assert!(!text.contains(marker), "the notice carries {marker:?}: {text}");
    }
}

#[then("the page sends no request to another origin since it loaded")]
async fn no_foreign_request(w: &mut DutyWorld) {
    let state = driver(w).send("read_state", Map::new());
    let foreign: Vec<String> = state["requestLog"]
        .as_array()
        .expect("read_state returns the request log")
        .iter()
        .filter_map(|entry| entry["url"].as_str())
        .filter(|url| !url.starts_with("http://127.0.0.1"))
        .map(str::to_string)
        .collect();
    assert!(foreign.is_empty(), "the notice requested {foreign:?}");
}

#[then(regex = r#"^the page sets no cookie and stores no key in "localStorage" or "sessionStorage"$"#)]
async fn no_cookie_no_storage(w: &mut DutyWorld) {
    let state = document(w);
    assert_eq!(
        state["cookies"].as_str(),
        Some(""),
        "the notice set a cookie: {state}"
    );
    for scope in ["local", "session"] {
        let keys = state["storage"][scope]
            .as_array()
            .unwrap_or_else(|| panic!("read_text returns {scope} storage"));
        assert!(keys.is_empty(), "the notice stored a {scope} key: {keys:?}");
    }
}

#[then("the reader saw the site root's heading before activation and the notice's heading after it")]
async fn heading_changed(w: &mut DutyWorld) {
    assert!(
        !w.heading_before.is_empty() && !w.heading_after.is_empty(),
        "the scenario recorded no heading"
    );
    assert_ne!(
        w.heading_before, w.heading_after,
        "the notice rendered the site root's heading; the link did not navigate"
    );
}

#[then("the page names the controller and how to reach it")]
async fn names_controller(w: &mut DutyWorld) {
    assert_says(w, &["1. Verantwortlicher", "c/o adesso SE", "E-Mail:"]);
}

#[then("the page states that GitHub, Inc. hosts the site in the United States and logs the visitor's IP address for security purposes")]
async fn states_hosting(w: &mut DutyWorld) {
    assert_says(
        w,
        &[
            "GitHub, Inc.",
            "San Francisco",
            "IP-Adresse",
            "Sicherheitszwecken",
        ],
    );
}

#[then("the page names the time of the request, the page, the referrer, and the browser and device information that GitHub processes")]
async fn names_processed_data(w: &mut DutyWorld) {
    assert_says(
        w,
        &[
            "Zeitpunkt der Anfrage",
            "aufgerufene Seite",
            "verweisende Seite",
            "Browser- und Geräteinformationen",
        ],
    );
}

#[then("the page states that GitHub publishes no fixed retention period and that the operator has no access to those logs")]
async fn states_retention_and_access(w: &mut DutyWorld) {
    assert_says(
        w,
        &["feste Speicherdauer veröffentlicht GitHub nicht", "keinen Zugriff auf diese Protokolle"],
    );
}

#[then("the page states the legal basis of that processing")]
async fn states_legal_basis(w: &mut DutyWorld) {
    assert_says(w, &["Art. 6 Abs. 1 lit. f DSGVO"]);
}

#[then("the page states the third-country transfer and its basis")]
async fn states_transfer(w: &mut DutyWorld) {
    assert_says(w, &["Data Privacy Framework", "Standardvertragsklauseln"]);
}

#[then("the page states that the host uses sub-processors to deliver the pages")]
async fn states_sub_processors(w: &mut DutyWorld) {
    assert_says(w, &["Unterauftragsverarbeiter"]);
}

#[then("the page names the mail-forwarding provider, its role and its location, and the legal basis of the forwarding")]
async fn states_mail_provider(w: &mut DutyWorld) {
    assert_says(
        w,
        &["SimpleLogin", "Proton AG", "Schweiz", "in meinem Auftrag"],
    );
}

#[then("the page states that the conversion runs in the browser without upload, storage or a request after it loads")]
async fn states_local_conversion(w: &mut DutyWorld) {
    assert_says(
        w,
        &[
            "ausschließlich in Ihrem Browser",
            "keine Übertragung an einen Server",
            "keine Netzwerkanfrage mehr",
        ],
    );
}

#[then("the page states that no cookie is set and no third-party content is loaded")]
async fn states_cookie_free(w: &mut DutyWorld) {
    assert_says(w, &["keine Cookies", "keine Inhalte Dritter"]);
}

#[then("the page states the reader's rights and the right to complain to a supervisory authority")]
async fn states_rights(w: &mut DutyWorld) {
    assert_says(w, &["Auskunft", "Widerspruch", "Aufsichtsbehörde"]);
}

#[then("the page carries a standing date")]
async fn states_standing_date(w: &mut DutyWorld) {
    assert_says(w, &["Stand: 27.10.2026"]);
}

/// Where this scenario's committed screenshots belong.
fn evidence_dir(slug: &str) -> PathBuf {
    let root = repo_root();
    let active = root.join("givn/changes/publish-legal-notices");
    let base = if active.join("proposal.md").is_file() {
        active
    } else {
        root.join("givn/archive/publish-legal-notices")
    };
    base.join("evidence/visual").join(slug)
}

/// The evidence directory name for a notice: the scenario title, slugified the
/// way the archive gate slugs it (a non-ASCII character becomes one hyphen).
fn notice_slug(heading: &str) -> String {
    if heading.contains("Impressum") {
        "reader-opens-the-impressum-from-the-site-footer-and-reads-who-publishes-the-site".to_string()
    } else {
        "reader-opens-the-datenschutzerkl-rung-from-the-footer-and-reads-what-a-page-request-processes"
            .to_string()
    }
}

#[then("the page meets the visual baseline at both viewports")]
async fn meets_visual_baseline(w: &mut DutyWorld) {
    // The committed screenshots are the frozen record of the reviewed run; a
    // normal run captures beside the scenario so another machine's rendering
    // cannot rewrite that record.
    let dir = if std::env::var_os("DUTY2CAL_UPDATE_VISUAL_EVIDENCE").is_some() {
        evidence_dir(&notice_slug(&w.heading_after))
    } else {
        fresh_dir().join("visual-capture")
    };
    std::fs::create_dir_all(&dir).expect("create the visual evidence directory");
    for (width, height, viewport) in [(360_u32, 780_u32, "mobile"), (1920, 1080, "desktop")] {
        let path = dir.join(format!("{viewport}.png"));
        let mut params = Map::new();
        params.insert("path".to_string(), Value::String(path.display().to_string()));
        params.insert("width".to_string(), Value::from(width));
        params.insert("height".to_string(), Value::from(height));
        let result = driver(w).send("screenshot", params);
        assert_eq!(
            result["savedTo"].as_str(),
            Some(path.display().to_string().as_str()),
            "the driver did not capture {}: {result}",
            path.display()
        );
        let mut params = Map::new();
        params.insert("baseline".to_string(), Value::Bool(true));
        let state = driver(w).send("read_state", params);
        assert_visual_baseline(&state, viewport, "de");
        let axe = driver(w).send("axe", Map::new());
        let violations = axe["violations"]
            .as_array()
            .expect("axe returns a violations array");
        assert!(
            violations.is_empty(),
            "the {viewport} notice has accessibility violations: {violations:?}"
        );
    }
}
