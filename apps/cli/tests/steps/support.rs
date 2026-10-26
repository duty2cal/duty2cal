// SPDX-License-Identifier: MPL-2.0
//! Test support: build a minimal, uncompressed PDF from positioned text items,
//! and place them at the column positions of the supported detail table.
//!
//! Fixtures are generated at test time, so no real plan or committed binary
//! fixture is needed (see the change's fixture decision).

/// Column x positions of the supported detail table.
pub const DATE_X: f64 = 40.0;
pub const TRIP_X: f64 = 100.0;
pub const REPORT_X: f64 = 150.0;
pub const POS_X: f64 = 210.0;
pub const ACTIVITY_X: f64 = 250.0;
pub const FROM_X: f64 = 320.0;
pub const TO_X: f64 = 360.0;
pub const START_X: f64 = 400.0;
pub const END_X: f64 = 480.0;
pub const AIRCRAFT_X: f64 = 540.0;
pub const BLOCKTIME_X: f64 = 600.0;
pub const TZ_X: f64 = 660.0;
pub const LAYOVER_X: f64 = 760.0;
pub const TAGS_X: f64 = 820.0;

/// Escape a string for a PDF literal string.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// A positioned piece of text in a generated fixture: x, y, and the text.
pub type TextItem = (f64, f64, String);

/// The roster month and year a fixture's dates belong to, and its heading.
///
/// A fixture writes its dates as `YYYY-MM-DD`; the builder prints them the way
/// the producer does, so the reader always faces the producer's spelling.
fn producer_layout(items: &[TextItem]) -> (Vec<TextItem>, Option<Vec<TextItem>>) {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut months: Vec<(i32, u32)> = Vec::new();
    let mut printed = Vec::with_capacity(items.len());
    // A revision footer prints its own stamp, which is not a roster date.
    let footer_rows: Vec<f64> = items
        .iter()
        .filter(|(_, _, text)| text.contains("Created"))
        .map(|(_, y, _)| *y)
        .collect();
    for (x, y, text) in items {
        let Some((year, month, day)) = iso_date(text) else {
            printed.push((*x, *y, text.clone()));
            continue;
        };
        if footer_rows
            .iter()
            .any(|footer_y| (footer_y - y).abs() < 1.0)
        {
            printed.push((*x, *y, text.clone()));
            continue;
        }
        if !months.contains(&(year, month)) {
            months.push((year, month));
        }
        printed.push((
            *x,
            *y,
            format!("{day:02} {}", weekday_abbreviation(year, month, day)),
        ));
    }
    assert!(
        months.len() <= 1,
        "a fixture must print one roster month, found {months:?}"
    );
    let heading = months.first().map(|(year, month)| {
        at(
            830.0,
            &[
                (DATE_X, "Month:"),
                (DATE_X + 40.0, MONTHS[(month - 1) as usize]),
                // Clear of the longest month name, so the two never touch.
                (DATE_X + 100.0, &year.to_string()),
            ],
        )
    });
    (printed, heading)
}

/// A `YYYY-MM-DD` cell as its calendar date.
fn iso_date(text: &str) -> Option<(i32, u32, u32)> {
    if text.len() != 10 || &text[4..5] != "-" || &text[7..8] != "-" {
        return None;
    }
    let year = text[..4].parse::<i32>().ok()?;
    let month = text[5..7].parse::<u32>().ok()?;
    let day = text[8..10].parse::<u32>().ok()?;
    (1..=12).contains(&month).then_some((year, month, day))
}

/// The weekday of a calendar date, as the producer abbreviates it.
pub fn weekday_abbreviation(year: i32, month: u32, day: u32) -> &'static str {
    const SHIFTS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut shifted = year;
    if month < 3 {
        shifted -= 1;
    }
    let index = (shifted + shifted / 4 - shifted / 100
        + shifted / 400
        + SHIFTS[(month - 1) as usize]
        + day as i32)
        .rem_euclid(7) as usize;
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][index]
}

/// Build a one-page PDF placing each `(x, y, text)` item with Helvetica 9pt.
///
/// Coordinates are in page space (origin bottom-left); the page is A4-sized.
/// The heading and the date spelling are the producer's, never the fixture's.
pub fn build_pdf(items: &[(f64, f64, String)]) -> Vec<u8> {
    build_pdf_with_padding(items, 0)
}

/// Build a PDF of exactly `size` bytes whose page content stays readable.
///
/// The padding is one comment line inside the content stream, so the trailing
/// `%%EOF` stays where the reader expects it. `/Length` grows with the
/// padding, so the builder corrects for the changed digit count.
pub fn build_pdf_at_size(items: &[(f64, f64, String)], size: usize) -> Vec<u8> {
    let base = build_pdf(items);
    assert!(
        base.len() <= size,
        "a {size}-byte PDF cannot hold the {} bytes of fixture content",
        base.len()
    );
    let mut padding = size - base.len();
    for _ in 0..8 {
        let pdf = build_pdf_with_padding(items, padding);
        if pdf.len() == size {
            return pdf;
        }
        if pdf.len() < size {
            padding += size - pdf.len();
        } else {
            padding = padding.saturating_sub(pdf.len() - size);
        }
    }
    panic!("could not build a PDF of exactly {size} bytes");
}

/// The PDF builder with `padding` bytes of content-stream comment.
fn build_pdf_with_padding(items: &[(f64, f64, String)], padding: usize) -> Vec<u8> {
    let (printed, heading) = producer_layout(items);
    let mut all = heading.unwrap_or_default();
    all.extend(printed);
    let items = &all;
    let mut content = String::from("BT\n/F1 9 Tf\n");
    for (x, y, text) in items {
        content.push_str(&format!("1 0 0 1 {x} {y} Tm ({}) Tj\n", escape(text)));
    }
    content.push_str("ET\n");
    match padding {
        0 => {}
        1 => content.push('\n'),
        _ => {
            content.push('%');
            content.push_str(&" ".repeat(padding - 2));
            content.push('\n');
        }
    }

    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 1200 842] \
         /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>"
            .to_string(),
        format!(
            "<< /Length {} >>\nstream\n{content}endstream",
            content.len()
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
    ];

    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (index, body) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.push_str(&format!("{} 0 obj\n{body}\nendobj\n", index + 1));
    }

    let xref_position = pdf.len();
    pdf.push_str(&format!("xref\n0 {}\n", objects.len() + 1));
    pdf.push_str("0000000000 65535 f \n");
    for offset in &offsets {
        pdf.push_str(&format!("{offset:010} 00000 n \n"));
    }
    pdf.push_str(&format!(
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_position}\n%%EOF\n",
        objects.len() + 1
    ));

    pdf.into_bytes()
}

/// Place `(x, text)` values on one row at height `y`.
pub fn at(y: f64, values: &[(f64, &str)]) -> Vec<(f64, f64, String)> {
    values
        .iter()
        .map(|(x, text)| (*x, y, (*text).to_string()))
        .collect()
}

/// The detail-table header row: local-date, local-report and UTC-time labels.
///
/// Adjacent label words are placed within the merge tolerance so that wrapped
/// headings such as `Date (LT)` and `Start (UTC)` form one column each.
pub fn detail_header_row(y: f64) -> Vec<(f64, f64, String)> {
    let mut items = Vec::new();
    items.extend(at(
        y,
        &[
            (DATE_X, "Date"),
            (DATE_X + 20.0, "(LT)"),
            (TRIP_X, "Trip"),
            (REPORT_X, "Report"),
            (REPORT_X + 30.0, "(LT)"),
            (POS_X, "Pos"),
            (ACTIVITY_X, "Activity"),
            (FROM_X, "From"),
            (TO_X, "To"),
            (START_X, "Start"),
            (START_X + 24.0, "(UTC)"),
            (END_X, "End"),
            (END_X + 16.0, "(UTC)"),
            (AIRCRAFT_X, "A/C"),
            (BLOCKTIME_X, "Blocktime"),
            (TZ_X, "Timezone"),
            (LAYOVER_X, "Layover"),
            (TAGS_X, "Tags"),
        ],
    ));
    // The header wraps over several print rows: this one carries the words that
    // continue a column, and the anchor labels of the family are spread across
    // both rows only when the producer's own header starts on a later row.
    items.extend(at(y - 9.0, &[(TZ_X, "Difference")]));
    items
}

// ---------------------------------------------------------------------------
// Site publication fixtures (design.md "Local runnability & digital twins").
//
// A per-process template is a real git clone of the project's committed
// history with the working-tree implementation overlaid and committed, a real
// `v0.5.0` tag and three documentation-only commits after it. Scenarios clone
// the template cheaply; a clone may retag, commit and be checked out without
// touching another scenario. `abc1234` labels the revision three commits after
// the tag (the branch `site-base`); a scenario that needs `def5678` commits it
// and records the label. `9999999` is deliberately not a revision.
//
// The hosting target is a real bare repository; a loopback server reads the
// target's branch tip on every request, so a publication's own serving check
// sees its push. The recording `gh` stand-in completes the twins the change's
// later tasks consume.
// ---------------------------------------------------------------------------

use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use crate::DutyWorld;

/// The project checkout under test: the implementation the steps drive.
pub fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A fixed identity and clock, so every fixture revision is reproducible.
fn fixture_env(command: &mut Command) -> &mut Command {
    command
        .env("GIT_AUTHOR_NAME", "duty2cal fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@duty2cal.invalid")
        .env("GIT_COMMITTER_NAME", "duty2cal fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@duty2cal.invalid")
        .env("GIT_AUTHOR_DATE", "2026-01-01T00:00:00+0000")
        .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00+0000")
}

fn run_command(command: &mut Command, what: &str) -> Output {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("{what}: {error}"));
    assert!(
        output.status.success(),
        "{what} failed with {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git_ok(dir: &Path, args: &[&str]) -> String {
    let mut command = Command::new("git");
    command.arg("-C").arg(dir).args(args);
    fixture_env(&mut command);
    let output = run_command(&mut command, &format!("git {}", args.join(" ")));
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_bare(target: &Path, args: &[&str]) -> String {
    let mut command = Command::new("git");
    command.arg(format!("--git-dir={}", target.display())).args(args);
    fixture_env(&mut command);
    let output = run_command(&mut command, &format!("git --git-dir target {}", args.join(" ")));
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination)
        .unwrap_or_else(|error| panic!("create {}: {error}", destination.display()));
    for entry in fs::read_dir(source).unwrap_or_else(|error| panic!("read {}: {error}", source.display())) {
        let entry = entry.expect("read a fixture directory entry");
        let target = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap_or_else(|error| {
                panic!("copy {} to {}: {error}", entry.path().display(), target.display())
            });
        }
    }
}

/// The per-process fixture template.
fn site_template() -> PathBuf {
    static TEMPLATE: OnceLock<PathBuf> = OnceLock::new();
    TEMPLATE.get_or_init(build_site_template).clone()
}

fn build_site_template() -> PathBuf {
    let template = std::env::temp_dir().join(format!(
        "duty2cal-site-template-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&template);
    let project = repository_root();

    let mut clone = Command::new("git");
    clone
        .arg("clone")
        .arg("-q")
        .arg("--no-hardlinks")
        .arg(&project)
        .arg(&template);
    fixture_env(&mut clone);
    run_command(&mut clone, "clone the fixture template");

    // The project's own `v0.5.0` would collide with the fixture's tag; the
    // other release tags stay, because the project's release-text scenarios
    // read them from the same history.
    if template.join(".git/refs/tags/v0.5.0").exists()
        || template.join(".git/packed-refs").exists()
    {
        let _ = Command::new("git")
            .arg("-C")
            .arg(&template)
            .args(["tag", "-d", "v0.5.0"])
            .output();
    }

    // The change's planning artifacts are not part of a publishable revision:
    // the tag's suite is the permanent corpus, so its gate can never select an
    // unfinished change's own scenarios.
    let changes = template.join("givn/changes");
    if changes.exists() {
        fs::remove_dir_all(&changes).expect("remove the change's planning artifacts");
    }
    // The fixture revision carries the capabilities this test host can run.
    // The two size capabilities build for x86_64-pc-windows-gnu, whose target
    // standard library and cross linker this host does not carry; their
    // scenarios are left out of the fixture's corpus, while their use cases
    // stay, because the change log's capability allow-list must still name
    // them. The publication's gate is unchanged: it runs whatever suite the
    // revision holds.
    for capability in [
        "givn/specs/measure-distributed-executable-size",
        "givn/specs/reduce-distributed-executable-size",
    ] {
        let dir = template.join(capability);
        if !dir.exists() {
            continue;
        }
        for entry in fs::read_dir(&dir).expect("read a fixture capability") {
            let entry = entry.expect("read a fixture capability entry");
            if entry.path().extension().is_some_and(|ext| ext == "feature") {
                fs::remove_file(entry.path()).expect("remove an unsupported fixture capability");
            }
        }
    }

    // The publication gate runs the revision's own suite, so the fixture keeps
    // one representative capability instead of the project's whole runtime
    // suite: the test needs the gate mechanism, not a second copy of every
    // product scenario. `export-duty-plan-calendar` is the capability whose
    // browser `@e2e` and conversion suite the gate invokes, so it stays; the
    // other capabilities' use-case directories stay too, because the change
    // log's capability allow-list must still name them.
    let specs = template.join("givn/specs");
    for capability in fs::read_dir(&specs).expect("read the fixture corpus") {
        let capability = capability.expect("read a fixture capability entry");
        if !capability.path().is_dir() {
            continue;
        }
        for entry in fs::read_dir(capability.path()).expect("read a fixture capability directory") {
            let entry = entry.expect("read a fixture capability file");
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "feature")
                && entry.file_name() != "export-duty-plan-calendar.feature"
            {
                fs::remove_file(&path).expect("trim a fixture capability feature");
            }
        }
    }

    // Overlay the working-tree implementation. Tracked modifications and
    // untracked source files are copied; a tracked deletion is removed. The
    // change's planning artifacts (`givn/changes/**`) are not part of a
    // publishable revision, so the tag's suite is the permanent corpus.
    let changed = git_ok(&project, &["diff", "--name-only", "-z", "HEAD"]);
    for relative in changed.split('\0').filter(|path| !path.is_empty()) {
        let source = project.join(relative);
        let destination = template.join(relative);
        if source.is_file() {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).expect("create an overlay directory");
            }
            fs::copy(&source, &destination).unwrap_or_else(|error| {
                panic!("overlay {relative}: {error}")
            });
        } else if destination.exists() {
            fs::remove_file(&destination).expect("remove a deleted fixture file");
        }
    }
    let untracked = git_ok(&project, &["ls-files", "--others", "--exclude-standard", "-z"]);
    for relative in untracked.split('\0').filter(|path| !path.is_empty()) {
        if relative.starts_with("givn/changes/")
            || relative.starts_with("dist/")
            || relative.starts_with("coverage/")
            || relative.starts_with("reports/")
        {
            continue;
        }
        let source = project.join(relative);
        if !source.is_file() {
            continue;
        }
        let destination = template.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).expect("create an overlay directory");
        }
        fs::copy(&source, &destination)
            .unwrap_or_else(|error| panic!("overlay {relative}: {error}"));
    }
    git_ok(&template, &["add", "-A"]);
    git_ok(&template, &["commit", "-q", "-m", "fixture: overlay the working tree"]);

    // A real tag and three commits after it, so the `tag-N-gSHA` form is
    // produced by git rather than synthesised by the test.
    git_ok(&template, &["tag", "v0.5.0"]);
    for number in 1..=3 {
        let relative = format!("docs/fixture-commit-{number}.md");
        fs::write(
            template.join(&relative),
            format!("Fixture documentation revision {number}.\n"),
        )
        .expect("write a fixture documentation revision");
        git_ok(&template, &["add", &relative]);
        git_ok(
            &template,
            &["commit", "-q", "-m", &format!("docs: fixture revision {number}")],
        );
    }
    // The label `abc1234` is this branch; it stays put when a scenario commits
    // a later revision on `main`.
    git_ok(&template, &["branch", "site-base"]);
    template
}

/// The Site output assembled from the fixture revision `abc1234`, built once
/// per process from the project's already-built browser bundle.
fn site_base_output() -> PathBuf {
    static OUTPUT: OnceLock<PathBuf> = OnceLock::new();
    OUTPUT
        .get_or_init(|| {
            let template = site_template();
            let revision = git_ok(&template, &["rev-parse", "refs/heads/site-base"]);
            let work = std::env::temp_dir().join(format!(
                "duty2cal-site-base-tree-{}",
                std::process::id()
            ));
            let output = std::env::temp_dir().join(format!(
                "duty2cal-site-base-output-{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&work);
            let _ = fs::remove_dir_all(&output);
            git_ok(
                &template,
                &["worktree", "add", "--detach", path_str(&work), revision.as_str()],
            );
            copy_tree(
                &repository_root().join("web/pkg"),
                &work.join("web/pkg"),
            );
            let mut command = Command::new(work.join("scripts/build-site.sh"));
            command
                .arg("--repo")
                .arg(&work)
                .arg("--revision")
                .arg(&revision)
                .arg("--out")
                .arg(&output);
            run_command(&mut command, "assemble the fixture's base Site output");
            git_ok(
                &template,
                &["worktree", "remove", "--force", path_str(&work)],
            );
            output
        })
        .clone()
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("the fixture path is valid UTF-8")
}

/// Fresh per-scenario directory, so concurrently executed scenarios never
/// share a fixture.
fn fresh_site_directory() -> PathBuf {
    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "duty2cal-site-{}-{sequence}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create the scenario's site directory");
    dir
}

/// The scenario's fixture directory, created on first use.
fn site_directory(w: &mut DutyWorld) -> PathBuf {
    if w.dir.is_none() {
        w.dir = Some(fresh_site_directory());
    }
    let dir = w.working_dir().join("site");
    fs::create_dir_all(&dir).expect("create the site fixture directory");
    dir
}

/// The scenario's clone of the fixture template.
fn site_repository(w: &mut DutyWorld) -> PathBuf {
    let dir = site_directory(w);
    let repo = dir.join("fixture");
    if !repo.join(".git").exists() {
        let template = site_template();
        let mut clone = Command::new("git");
        clone
            .arg("clone")
            .arg("-q")
            .arg("--no-hardlinks")
            .arg(&template)
            .arg(&repo);
        fixture_env(&mut clone);
        run_command(&mut clone, "clone the scenario fixture");
        // The caller's installed driver tree stands in for the clone's: the
        // lockfiles are identical because the clone holds the same revision.
        let node_modules = repository_root().join("web/e2e/node_modules");
        std::os::unix::fs::symlink(node_modules, repo.join("web/e2e/node_modules"))
            .expect("link the driver tree into the fixture");
    }
    repo
}

/// The bare hosting target.
fn site_target(w: &mut DutyWorld) -> PathBuf {
    site_directory(w).join("target.git")
}

/// The publication's output directory.
fn site_output(w: &mut DutyWorld) -> PathBuf {
    site_directory(w).join("out")
}

/// The output directory a step asserts against.
pub fn site_output_directory(w: &mut DutyWorld) -> PathBuf {
    site_output(w)
}

fn site_state_path(w: &mut DutyWorld, key: &str) -> PathBuf {
    site_directory(w).join(format!("state-{key}"))
}

/// Record one scenario fact the steps share (a revision label, the serving
/// address, and so on).
pub fn site_set(w: &mut DutyWorld, key: &str, value: &str) {
    let path = site_state_path(w, key);
    fs::write(&path, value).unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

/// Read one scenario fact.
pub fn site_get(w: &mut DutyWorld, key: &str) -> Option<String> {
    fs::read_to_string(site_state_path(w, key))
        .ok()
        .map(|value| value.trim().to_string())
}

/// Resolve a scenario's revision label to the fixture's real commit.
pub fn site_label(w: &mut DutyWorld, label: &str) -> String {
    if label == "abc1234" {
        // `site-base` is a remote-tracking branch in every clone, so it keeps
        // naming the third commit after the tag as `main` advances.
        git_ok(&site_repository(w), &["rev-parse", "refs/remotes/origin/site-base"])
    } else if label == "9999999" {
        label.to_string()
    } else {
        let path = site_directory(w).join(format!("label-{label}"));
        fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("the fixture holds no revision labelled {label}"))
            .trim()
            .to_string()
    }
}

/// Record the checked-out commit under a scenario's label.
pub fn site_record_label(w: &mut DutyWorld, label: &str) -> String {
    let revision = git_ok(&site_repository(w), &["rev-parse", "HEAD"]);
    let path = site_directory(w).join(format!("label-{label}"));
    fs::write(&path, &revision).expect("record a fixture revision label");
    revision
}

/// Check out a revision in the fixture repository.
pub fn site_checkout(w: &mut DutyWorld, revision: &str) {
    git_ok(&site_repository(w), &["checkout", "-q", "--detach", revision]);
}

/// Append text to a file in the fixture's working tree, leaving it tracked but
/// modified.
pub fn site_append(w: &mut DutyWorld, relative: &str, body: &str) {
    use std::io::Write as _;
    let path = site_repository(w).join(relative);
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap_or_else(|error| panic!("append {}: {error}", path.display()));
    file.write_all(body.as_bytes())
        .expect("append the fixture edit");
}

/// Write a file the fixture revision does not track.
pub fn site_write_untracked(w: &mut DutyWorld, relative: &str, body: &str) {
    let path = site_repository(w).join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create an untracked fixture directory");
    }
    fs::write(&path, body).expect("write the untracked fixture file");
}

/// Commit a file on the fixture's current branch and return the revision.
pub fn site_commit_file(
    w: &mut DutyWorld,
    relative: &str,
    body: &str,
    message: &str,
) -> String {
    let repo = site_repository(w);
    let path = repo.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create a fixture commit directory");
    }
    fs::write(&path, body).expect("write a fixture commit file");
    git_ok(&repo, &["add", relative]);
    git_ok(&repo, &["commit", "-q", "-m", message]);
    git_ok(&repo, &["rev-parse", "HEAD"])
}

/// Replace one text fragment in a file the fixture tracks, commit the result
/// on the current branch and return the revision.
pub fn site_commit_replace(
    w: &mut DutyWorld,
    relative: &str,
    needle: &str,
    replacement: &str,
    message: &str,
) -> String {
    let path = site_repository(w).join(relative);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        body.contains(needle),
        "{} does not carry the fragment the fixture replaces: {needle:?}",
        path.display()
    );
    site_commit_file(w, relative, &body.replacen(needle, replacement, 1), message)
}

/// Remove every line carrying `marker` from a file the fixture tracks, commit
/// the result and return the revision.
pub fn site_commit_drop_line(
    w: &mut DutyWorld,
    relative: &str,
    marker: &str,
    message: &str,
) -> String {
    let path = site_repository(w).join(relative);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let mut removed = false;
    let mut edited = String::new();
    for line in body.lines() {
        if line.contains(marker) {
            removed = true;
            continue;
        }
        edited.push_str(line);
        edited.push('\n');
    }
    assert!(
        removed,
        "{} carries no line with the marker the fixture drops: {marker:?}",
        path.display()
    );
    site_commit_file(w, relative, &edited, message)
}

/// Append text to a file the fixture tracks, commit the result and return the
/// revision.
pub fn site_commit_append(
    w: &mut DutyWorld,
    relative: &str,
    body: &str,
    message: &str,
) -> String {
    let path = site_repository(w).join(relative);
    let mut current = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    current.push_str(body);
    site_commit_file(w, relative, &current, message)
}

/// The paths a revision changes relative to a base.
pub fn site_changed_paths(w: &mut DutyWorld, base: &str, revision: &str) -> Vec<String> {
    git_ok(
        &site_repository(w),
        &["diff", "--name-only", &format!("{base}..{revision}")],
    )
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .map(str::to_string)
    .collect()
}

/// Move a fixture tag onto a revision.
pub fn site_tag(w: &mut DutyWorld, tag: &str, revision: &str) {
    git_ok(&site_repository(w), &["tag", "-f", tag, revision]);
}

/// Whether the fixture repository holds the revision.
pub fn site_resolves(w: &mut DutyWorld, revision: &str) -> bool {
    let repo = site_repository(w);
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(&repo)
        .args(["rev-parse", "--verify", &format!("{revision}^{{commit}}")]);
    fixture_env(&mut command);
    command
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Resolve a fixture revision or tag to its commit.
pub fn site_resolve(w: &mut DutyWorld, revision: &str) -> String {
    git_ok(
        &site_repository(w),
        &["rev-parse", &format!("{revision}^{{commit}}")],
    )
}

/// The Built revision text the fixture's own history yields.
pub fn site_describe(w: &mut DutyWorld, revision: &str) -> String {
    git_ok(
        &site_repository(w),
        &["describe", "--tags", "--always", "--match", "v*", revision],
    )
}

/// Populate the bare hosting target with the Site output built from `label`
/// and start a loopback server over the target's branch tip.
pub fn site_set_up_target(w: &mut DutyWorld, label: &str) -> String {
    assert_eq!(
        label, "abc1234",
        "the scenario fixtures build their hosting target from the base revision"
    );
    let revision = site_label(w, label);
    let populated = site_base_output();
    let target = site_target(w);
    fs::create_dir_all(site_directory(w)).expect("create the fixture directory");
    let mut init = Command::new("git");
    init.arg("init").arg("-q").arg("--bare").arg(&target);
    fixture_env(&mut init);
    run_command(&mut init, "create the fixture hosting target");

    let stage = site_directory(w).join("target-stage");
    let _ = fs::remove_dir_all(&stage);
    let mut init = Command::new("git");
    init.arg("init").arg("-q").arg(&stage);
    fixture_env(&mut init);
    run_command(&mut init, "create the fixture target staging tree");
    copy_tree(&populated, &stage);
    git_ok(&stage, &["add", "-A"]);
    git_ok(
        &stage,
        &["commit", "-q", "-m", "publish-site: the previously published page"],
    );
    git_ok(&stage, &["remote", "add", "origin", path_str(&target)]);
    git_ok(&stage, &["push", "-q", "origin", "HEAD:refs/heads/main"]);

    let tip = git_bare(&target, &["rev-parse", "refs/heads/main"]);
    let describe = site_describe(w, &revision);
    site_set(w, "target-tip", &tip);
    site_set(w, "base-revision", &revision);
    site_set(w, "base-describe", &describe);

    let port = site_server_port(w, &[]);

    let url = format!("http://127.0.0.1:{port}/");
    site_set(w, "verify-url", &url);
    url
}

/// Start the fixture's loopback static server over the hosting target's branch
/// tip, with extra arguments, and return its port. Any earlier server is
/// reaped first: a scenario has one serving address at a time.
fn site_server_port(w: &mut DutyWorld, extra: &[&str]) -> u16 {
    if let Some(mut child) = w.fake.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let target = site_target(w);
    let server = repository_root().join("apps/cli/tests/support/site_server.py");
    let mut child = Command::new("python3")
        .arg(&server)
        .arg("--repo")
        .arg(&target)
        .arg("--branch")
        .arg("main")
        .args(extra)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("start the loopback site server");
    let mut reader = BufReader::new(child.stdout.take().expect("the server owns a stdout pipe"));
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("read the loopback server's port");
    let port: u16 = line
        .trim()
        .parse()
        .unwrap_or_else(|error| panic!("the loopback server printed {line:?}: {error}"));
    w.fake = Some(child);
    port
}

/// The address the publication reads.
pub fn site_verify_url(w: &mut DutyWorld) -> String {
    site_get(w, "verify-url").expect("the scenario prepared no serving address")
}

/// Point the publication's serving check at a loopback address that keeps
/// serving the page built from the fixture's base revision, so the target's
/// replacement cannot change what that address returns. The earlier tree is
/// served under `/earlier/` by the same server that serves the target's branch
/// tip, which is the twin the design describes.
pub fn site_serve_earlier(w: &mut DutyWorld) -> String {
    let earlier = site_base_output();
    let port = site_server_port(w, &["--earlier", path_str(&earlier)]);

    let url = format!("http://127.0.0.1:{port}/earlier/");
    site_set(w, "verify-url", &url);
    url
}

/// The publication command for one revision and one output directory. The
/// scenario's `--since`, scratch path and incremental artifact directory are
/// part of the invocation, so a concurrent run uses the same publication as a
/// single run.
fn site_publication_command(w: &mut DutyWorld, revision: &str, out: &Path) -> Command {
    let repo = site_repository(w);
    let target = site_target(w);
    let url = site_verify_url(w);
    let script = repository_root().join("scripts/publish-site.sh");

    let mut command = Command::new(&script);
    command
        .arg("--repo")
        .arg(&repo)
        .arg("--revision")
        .arg(revision)
        .arg("--target")
        .arg(&target)
        .arg("--out")
        .arg(out)
        .arg("--verify-url")
        .arg(&url);
    if let Some(since) = site_get(w, "since") {
        command.arg("--since").arg(since);
    }
    if let Some(tmpdir) = site_get(w, "tmpdir") {
        command.env("TMPDIR", tmpdir);
    }
    // The fixture's own build and suite stay incremental. A directory of their
    // own keeps the fixture's artifacts from replacing the test binary this
    // scenario is running under (the two checkouts share a package id).
    command.env(
        "CARGO_TARGET_DIR",
        repository_root().join("target/site-publication"),
    );
    command
}

/// Run the publication once, recording its exit status and both streams.
pub fn site_publish(w: &mut DutyWorld) {
    let revision = site_get(w, "revision").expect("the scenario named no revision to publish");
    let since = site_get(w, "since");
    let out = site_output(w);
    let script = repository_root().join("scripts/publish-site.sh");

    let mut command = site_publication_command(w, &revision, &out);
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("run {}: {error}", script.display()));
    w.exit_status = output.status.code();
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    w.command_transcript = format!(
        "$ scripts/publish-site.sh --repo <fixture> --revision {revision}{} --target <target> --out <out> --verify-url <url>\n\
         exit: {:?}\nstdout:\n{}\nstderr:\n{}",
        since
            .as_deref()
            .map(|base| format!(" --since {base}"))
            .unwrap_or_default(),
        w.exit_status,
        w.stdout,
        w.stderr
    );
}

/// One concurrent publication's recorded file: its log or its exit status.
fn site_publication_path(w: &mut DutyWorld, index: usize, extension: &str) -> PathBuf {
    site_directory(w).join(format!("publication-{index}.{extension}"))
}

/// Run two publications of the scenario's revision concurrently: they begin
/// together, share the repository, the target and the default lock, and each
/// writes its own output directory. Records both exit statuses and both logs.
pub fn site_publish_together(w: &mut DutyWorld) {
    let revision = site_get(w, "revision").expect("the scenario named no revision to publish");
    let directory = site_directory(w);

    let mut children = Vec::new();
    for index in 0..2 {
        let out = directory.join(format!("out-{index}"));
        let log = site_publication_path(w, index, "log");
        let mut command = site_publication_command(w, &revision, &out);
        let file = fs::File::create(&log).expect("create the publication log");
        let errors = file.try_clone().expect("clone the publication log");
        command.stdout(Stdio::from(file)).stderr(Stdio::from(errors));
        let child = command
            .spawn()
            .unwrap_or_else(|error| panic!("start the site publication: {error}"));
        children.push(child);
    }

    let mut transcript = String::new();
    for (index, mut child) in children.into_iter().enumerate() {
        let status = child.wait().expect("wait for the site publication");
        let path = site_publication_path(w, index, "status");
        fs::write(&path, status.code().unwrap_or(-1).to_string())
            .expect("write the publication's exit status");
        transcript.push_str(&format!(
            "$ scripts/publish-site.sh (publication {index}) --repo <fixture> --revision {revision} --target <target> --out <out-{index}> --verify-url <url>\nexit: {:?}\n{}\n",
            status.code(),
            site_publication_log(w, index)
        ));
    }
    w.command_transcript = transcript;
}

/// One concurrent publication's recorded log, by index.
pub fn site_publication_log(w: &mut DutyWorld, index: usize) -> String {
    fs::read_to_string(site_publication_path(w, index, "log")).unwrap_or_default()
}

/// One concurrent publication's exit status, by index.
pub fn site_publication_status(w: &mut DutyWorld, index: usize) -> Option<i32> {
    fs::read_to_string(site_publication_path(w, index, "status"))
        .ok()
        .and_then(|text| text.trim().parse().ok())
}

/// The publication lock's default path in the fixture repository.
pub fn site_default_lock(w: &mut DutyWorld) -> PathBuf {
    let repo = site_repository(w);
    let git_dir = git_ok(&repo, &["rev-parse", "--absolute-git-dir"]);
    PathBuf::from(git_dir).join("publish-site.lock")
}

/// The publication's combined output, for a refusal's named failure.
pub fn site_publication_text(w: &DutyWorld) -> String {
    format!("{}\n{}", w.stdout, w.stderr)
}

/// One file from the hosting target's branch tip.
pub fn site_target_file(w: &mut DutyWorld, relative: &str) -> String {
    let target = site_target(w);
    git_bare(&target, &["show", &format!("main:{relative}")])
}

/// Every member of the hosting target's branch tip, sorted.
pub fn site_target_members(w: &mut DutyWorld) -> Vec<String> {
    site_target_git(w, &["ls-tree", "-r", "--name-only", "main"])
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

/// Commit a revision that changes the page under `label` and return its
/// commit. A comment inside the body is a different tree and a different Built
/// revision, while every rendered statement and the visual baseline stay as
/// they were, so the evidence gate passes.
pub fn site_page_revision(w: &mut DutyWorld, label: &str) -> String {
    let committed = site_commit_replace(
        w,
        "web/index.html",
        "</body>",
        "  <!-- fixture revision: republished page -->\n</body>",
        "docs: fixture page revision",
    );
    let recorded = site_record_label(w, label);
    assert_eq!(recorded, committed, "the fixture recorded another revision");
    committed
}

/// The fixture's bare hosting target path, as a publication names it.
pub fn site_target_url(w: &mut DutyWorld) -> String {
    site_target(w).to_string_lossy().into_owned()
}

/// The hosting target's branch tip.
pub fn site_target_tip(w: &mut DutyWorld) -> String {
    site_target_git(w, &["rev-parse", "refs/heads/main"])
}

/// The hosting target's branch history, newest first.
pub fn site_target_history(w: &mut DutyWorld) -> Vec<String> {
    site_target_git(w, &["rev-list", "main"])
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

/// Run one git command against the hosting target and return its output.
pub fn site_target_git(w: &mut DutyWorld, args: &[&str]) -> String {
    let target = site_target(w);
    git_bare(&target, args)
}

/// The parents of a revision in the hosting target.
pub fn site_target_parents(w: &mut DutyWorld, revision: &str) -> Vec<String> {
    site_target_git(w, &["rev-list", "--parents", "-n", "1", revision])
        .split_whitespace()
        .skip(1)
        .map(str::to_string)
        .collect()
}

/// Add a file to the hosting target by hand, as a person editing the branch
/// would, and return the resulting tip.
pub fn site_add_hand_file(w: &mut DutyWorld) -> String {
    let target = site_target(w);
    let stage = site_directory(w).join("hand-stage");
    let _ = fs::remove_dir_all(&stage);
    let mut clone = Command::new("git");
    clone
        .arg("clone")
        .arg("-q")
        .arg("-b")
        .arg("main")
        .arg(path_str(&target))
        .arg(&stage);
    fixture_env(&mut clone);
    run_command(&mut clone, "clone the hosting target for the hand edit");
    fs::write(
        stage.join("hand-edited.html"),
        "<!-- edited in the hosting target by hand -->\n",
    )
    .expect("write the hand-edited file");
    git_ok(&stage, &["add", "hand-edited.html"]);
    git_ok(
        &stage,
        &["commit", "-q", "-m", "hand edit: a page nobody published"],
    );
    git_ok(&stage, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
    git_bare(&target, &["rev-parse", "refs/heads/main"])
}

/// Every member of the publication's output directory, sorted.
pub fn site_output_members(w: &mut DutyWorld) -> Vec<String> {
    let out = site_output(w);
    let mut members = Vec::new();
    let mut pending = vec![out.clone()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).expect("read the Site output directory") {
            let path = entry.expect("read a Site output entry").path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let relative = path.strip_prefix(&out).expect("a member under the output");
                members.push(relative.to_string_lossy().into_owned());
            }
        }
    }
    members.sort();
    members
}

/// Read the page the serving address returns.
pub fn site_served_page(w: &mut DutyWorld) -> String {
    let url = site_verify_url(w);
    let mut command = Command::new("curl");
    command.args(["-fsS", "--max-time", "10", &url]);
    let output = run_command(&mut command, "read the serving address");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The Built revision text the served page carries, if any.
pub fn served_revision(page: &str) -> Option<String> {
    let marker = "data-site-statement=\"built-revision\">";
    let start = page.find(marker)? + marker.len();
    let rest = &page[start..];
    let end = rest.find('<')?;
    Some(rest[..end].to_string())
}

/// The publication's own evidence of the eight output members.
pub const SITE_MEMBERS: [&str; 8] = [
    "LICENSE.txt",
    "app.js",
    "capability.js",
    "icon.png",
    "index.html",
    "pkg/duty2cal_wasm.js",
    "pkg/wasm-inline.js",
    "styles.css",
];

/// Create a recording `gh` stand-in on `PATH` and return its directory. It
/// records every invocation and refuses an unknown subcommand, so a scenario
/// can prove both the order and the absence of a call.
pub fn gh_stand_in(w: &mut DutyWorld) -> PathBuf {
    let directory = site_directory(w).join("bin");
    fs::create_dir_all(&directory).expect("create the gh stand-in directory");
    let log = site_directory(w).join("gh.log");
    let path = directory.join("gh");
    // The generated script's own licence header is a string literal, not this
    // file's header; keep REUSE from parsing the literal's escape as part of
    // the expression.
    // REUSE-IgnoreStart
    let body = format!(
        "#!/usr/bin/env bash\n# SPDX-License-Identifier: MPL-2.0\n\
         printf '%s\\n' \"$*\" >> {log}\n\
         case \"$1\" in\n\
           release|api) exit 0 ;;\n\
           *) echo \"gh stand-in: unknown subcommand $1\" >&2; exit 1 ;;\n\
         esac\n",
        log = path_str(&log)
    );
    // REUSE-IgnoreEnd
    fs::write(&path, body).expect("write the recording gh stand-in");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
        .expect("make the gh stand-in executable");
    site_set(w, "gh-log", path_str(&log));
    directory
}

/// The lockfile-installed driver tree a publication links into its worktree.
pub fn installed_driver_tree() -> PathBuf {
    repository_root().join("web/e2e/node_modules")
}

/// Install a `pre-receive` hook that refuses every push into the fixture's
/// hosting target, so a scenario can prove a refused replacement. Returns the
/// hook, so the scenario can assert it is in place.
pub fn site_refuse_pushes(w: &mut DutyWorld) -> PathBuf {
    let target = site_target(w);
    let hooks = target.join("hooks");
    fs::create_dir_all(&hooks).expect("create the hosting target's hooks directory");
    let hook = hooks.join("pre-receive");
    // The generated hook's own licence header is a string literal, not this
    // file's header.
    // REUSE-IgnoreStart
    let body = "#!/usr/bin/env bash\n\
                # SPDX-License-Identifier: MPL-2.0\n\
                echo 'the hosting target refuses the incoming content' >&2\n\
                exit 1\n";
    // REUSE-IgnoreEnd
    fs::write(&hook, body).expect("write the hosting target's pre-receive hook");
    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755))
        .expect("make the pre-receive hook executable");
    hook
}

/// A scratch path the publication cannot create its worktree under.
pub fn unwritable_scratch(w: &mut DutyWorld) -> PathBuf {
    let path = site_directory(w).join("unwritable");
    fs::create_dir_all(&path).expect("create the unwritable scratch directory");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o555))
        .expect("make the scratch directory unwritable");
    path
}
