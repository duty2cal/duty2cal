# Tasks: windows-only-release

Release note (the subject of every revision this change records):

> Offer one downloadable archive for Windows, the only platform the project distributes.

Capability: `publish-public-release`. Change scope: the published platform set
narrows from five platforms to Windows x86_64. No converter crate changes.

## Preconditions verified while writing this file

- `givn status --change windows-only-release` reports proposal, specs, design
  and design-review done; `design-review.md` ends `DESIGN-REVIEW: PASS`.
- The delta use-case `## Interactions` table has two rows, each with an existing
  `@e2e` scenario:
  - prepare the release for a version → `Maintainer prepares a Windows release and inspects the archived executable` (`@givn.added @e2e`);
  - generate the change log for a version → `Maintainer generates the change log for a version and reads its newest section` (permanent `@e2e`, unchanged).
- The on-disk delta carries exactly six executable scenarios: four
  `@givn.modified`, one `@givn.added @e2e`, one `@givn.added` (Linux refusal).
  The two added scenarios and the four modified scenarios all carry `@wip`.
- `givn/specs/publish-public-release/publish-public-release.feature` is the
  permanent counterpart; its superseded blocks are removed at assembly, not
  edited.

## Standing rules for the implementer

1. **Pinned generator.** `scripts/render-changelog.sh` refuses any `git-cliff`
   other than 2.14.2. Every runner command in this file is shown as
   `./run-tests.sh ...`; execute it with the pinned binary on `PATH`:
   `nix-shell -p git-cliff --run '<command>'`. On this workstation
   `nix-shell -p git-cliff --run 'git-cliff --version'` prints
   `git-cliff 2.14.2`. Do not edit `run-tests.sh`, `givn/commands.yaml` or the
   generator.
2. **Strict mode is already configured.** `apps/cli/tests/features_runner.rs`
   calls `.fail_on_skipped()`. `givn/commands.yaml` has
   `verify.command: "./run-tests.sh"` and
   `verify.e2e_command: "./run-tests.sh --e2e"`. Do not change either.
3. **One scenario at a time, checked off immediately.** Tick a box only after
   its evidence is pasted and its commit hash is recorded. Never mass-check.
4. **Never an empty step body.** Use `unimplemented!()` or `todo!()` only while
   a step is a deliberate RED stub; GREEN replaces it with a real assertion or a
   real reused setup action.
5. **Never edit `givn/specs/**`.** The permanent corpus is the archived truth;
   the assembly filters it. Only the delta spec under
   `givn/changes/windows-only-release/specs/**` changes (removing `@wip`).
6. **Do not remove `@e2e`.** The tag on the new scenario is immutable.
7. **Commit with named paths** so unrelated pending changes (the docs tree) do
   not ride along:
   `givn commit -m "<message>" <path> [<path>...]`.
8. **The suite is intentionally red until the six `@wip` scenarios are
   implemented.** Setup's zero-exit proof is a targeted run of a permanent,
   already-green scenario, not the full suite. Never widen a filter or delete
   `@wip` to make the full suite green early.
9. **Two scenarios are re-baselined keep-true evidence** (`Each archive's
   checksum covers that archive`, `An unreadable executable stops the
   preparation`). `design.md` §"RED drivers and re-baselined evidence" says they
   are immediate-green after the one-platform fixture and are not production RED
   drivers. Their RED fields below record the real pre-GREEN run; never
   manufacture a failure.
10. **No production Rust changes.** The only Rust touched is step definitions.

---

## Task 0 — Setup: strict mode, e2e-subset proof, Gherkin-aware assembly

- [x] 0.1 Prove the configured strict mode (evidence below).
- [x] 0.2 Prove the runner rejects an undefined step, then restore a green
      targeted run (evidence below).
- [x] 0.3 Record full-suite and `@e2e` scenario counts, before and after
      assembly (evidence below).
- [x] 0.4 Implement `scripts/assemble_features.py` and make
      `scripts/assemble-features.sh` delegate to it.
- [x] 0.5 Fixture smoke: outline/`Examples`, multi-line tags, doc string
      containing `Scenario:`, duplicate title, two active changes.
- [x] 0.6 Real-corpus smoke: one copy of each modified title, no retired
      placeholder, both added scenarios, total count.
- [x] 0.7 Commit and record the revision.

### 0.1 Strict-mode configuration (already present — prove, do not change)

Evidence to paste:

```
$ grep -n 'fail_on_skipped' apps/cli/tests/features_runner.rs
$ sed -n '1,80p' givn/commands.yaml | grep -nE 'command:|e2e_command:'
$ nix-shell -p git-cliff --run 'git-cliff --version'
```

```
apps/cli/tests/features_runner.rs:57:        .fail_on_skipped()
31:  command: "./run-tests.sh"            # project-owned test script
35:  e2e_command: "./run-tests.sh --e2e"  # same script, @e2e filter
git-cliff 2.14.2
```

Expected: `.fail_on_skipped()` on the `Cucumber` builder; `command:
"./run-tests.sh"`; `e2e_command: "./run-tests.sh --e2e"`; `git-cliff 2.14.2`.

### 0.2 Strictness proof (undefined step fails, then restore)

The delta already has a unique new scenario whose `Given` is not implemented
yet: `An executable for the previously offered Linux platform is refused`
(`givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature`).
Run it before writing that step:

```
$ nix-shell -p git-cliff --run './run-tests.sh --name "An executable for the previously offered Linux platform is refused"'
```

```
Feature: Publish a public release
  Scenario: An executable for the previously offered Linux platform is refused
   ✔  Given an executable built for every supported platform
   ✘  And an executable presented as the previously offered platform "linux-x86_64"
      Step doesn't match any function
[Summary]
1 feature
1 scenario (1 failed)
2 steps (1 passed, 1 failed)
exit=101
```

Required: non-zero exit because
`Given an executable presented as the previously offered platform "linux-x86_64"`
has no step definition (cucumber-rs reports an undefined/failed step). If it
exits 0, stop: strict mode is not effective.

Restore: no probe file was created, so nothing is removed. Prove the runner is
green on a permanent scenario:

```
$ nix-shell -p git-cliff --run './run-tests.sh --name "An executable for an unrecognised platform is refused"'
```

```
Feature: Publish a public release
  Scenario: An executable for an unrecognised platform is refused
   ✔  Given an executable built for every supported platform
   ✔  And an executable presented as the unrecognised platform "freebsd-x86_64"
   ✔  When the maintainer prepares the release for version 0.1.0
   ✔  Then the preparation fails and names "freebsd-x86_64"
   ✔  And the prepared release holds no archive
[Summary]
1 feature
1 scenario (1 passed)
5 steps (5 passed)
exit=0
```

### 0.3 Scenario counts: `verify.e2e_command` is a strict subset

Capture the pre-assembly baseline (the current shell copies the permanent
corpus plus every delta, including superseded and retired blocks):

```
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-full-before.json ./run-tests.sh' || true
$ cat /tmp/givn-full-before.json
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-e2e-before.json ./run-tests.sh --e2e' || true
$ cat /tmp/givn-e2e-before.json
```

(The `|| true` is only so a `set -e` shell reaches the `cat`; the run's
non-zero status is expected because the `@wip` scenarios are unimplemented.)

```
pre-assembly (old shell copied every feature verbatim):
127 scenarios (92 passed, 35 failed)
e2e: 4 scenarios (1 passed, 3 failed)
```

After 0.4–0.6, capture the authoritative counts on the assembled corpus:

```
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-full.json ./run-tests.sh' || true
$ cat /tmp/givn-full.json
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-e2e.json ./run-tests.sh --e2e' || true
$ cat /tmp/givn-e2e.json
```

```
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-full.json ./run-tests.sh'
121 scenarios (117 passed, 4 failed)
{"scope":"regular","total":121,"passed":117,"failed":,"skipped":0}
$ nix-shell -p git-cliff --run 'GIVN_RESULT_FILE=/tmp/givn-e2e.json ./run-tests.sh --e2e'
3 scenarios (2 passed, 1 failed)
{"scope":"e2e","total":3,"passed":2,"failed":,"skipped":0}
```

Required: `e2e.total < regular.total` in both captures. The run exits non-zero
only because the six `@wip` scenarios are deliberately unimplemented; the
counts are still written by `run-tests.sh`. The only `@e2e` scenarios are the
permanent conversion scenario, the permanent change-log scenario, the
retained permanent publish scenario, and the added publish scenario (four after
the old publish scenario is replaced). Paste the actual numbers; do not
estimate.

### 0.4 Implement the assembly helper

Files to create/change:

- `scripts/assemble_features.py` — new, Python 3 standard library only,
  `# SPDX-License-Identifier: MPL-2.0` on the second line.
- `scripts/assemble-features.sh` — keep the no-argument entry point, keep
  printing the assembled root on stdout, delegate all Gherkin work to the
  helper. It must own the temporary root:
  ```
  root="$(mktemp -d)"
  python3 "$(dirname "$0")/assemble_features.py" --root "$PWD" --out "$root"
  printf '%s\n' "$root"
  ```
  Keep `set -euo pipefail` and the `cd "$(dirname "$0")/.."`; the caller
  (`run-tests.sh`, `measure-coverage.sh`) still cleans up the printed root.

Helper interface and rules (the design constraints; implement exactly):

- CLI: `assemble_features.py --root <repo-root> --out <dir>`; `--root`
  defaults to the current directory, `--out` is required. Write only under
  `--out`; never rewrite a file on disk. No stdout requirements for the helper
  (the shell prints the root); diagnostics and failures go to stderr.
- Inputs: every `--root/givn/specs/**/*.feature` (permanent) and every
  `--root/givn/changes/*/specs/**/*.feature` (delta), recursively, sorted for a
  deterministic output.
- Path mapping: `givn/changes/<change>/specs/<usecase>/<capability>.feature`
  maps to `givn/specs/<usecase>/<capability>.feature`. A delta with no permanent
  counterpart is assembled as a new capability (all its non-removed scenarios).
  A delta that carries `@givn.modified`/`@givn.removed` but has no permanent
  counterpart is an error naming the delta file.
- Parse Gherkin by blocks, never by a title grep:
  - A scenario block starts at the `Scenario:` or `Scenario Outline:` line
    together with the contiguous tag line(s) immediately above it.
  - It ends at the next tag line that introduces a scenario, at the next
    `Scenario:`/`Scenario Outline:` line, or at EOF.
  - An `Outline` keeps its `Examples:` table inside its block, byte for byte.
  - Do **not** treat `Scenario:` text inside a doc string (`"""` or ``` fences)
    or in a step/data-table line as a block boundary. Track doc-string fences.
  - Tag lines may contain several tags (`@givn.modified @wip`); tokenize them.
- For each delta, collect the exact titles tagged `@givn.modified` and
  `@givn.removed`, keyed to its mapped permanent file. In that permanent file
  only, remove those scenario blocks. Do not union titles across capabilities
  or across changes: two active changes filter their own permanent files.
- The helper must fail loudly (non-zero, message naming **both** the title and
  the file) when:
  - a `@givn.modified`/`@givn.removed` title is not found in its mapped
    permanent feature;
  - a title occurs more than once in the permanent feature (ambiguous removal);
  - two active deltas claim the same title for the same permanent feature
    (ambiguous assembly).
- Omit every `@givn.removed` scenario block from the copied delta output; its
  placeholder step must never reach cucumber-rs. Copy `@givn.added` and
  `@givn.modified` delta scenarios unchanged apart from removed placeholders.
- Preserve the feature-level prefix (feature tag line, `Feature:` line and
  description) of each file.
- Output files: `f%04d.feature` in `--out`, one per source file after filtering.
- Scope: Gherkin tags and scenario blocks only. Do not validate scenario shape;
  `givn lint` remains the delta validator. No permanent unit tests for the
  helper (the repository has no script-test harness); the fixture smoke in 0.5
  is the verification.

### 0.5 Fixture smoke (throwaway, outside the repository)

Create a fixture under `/tmp/givn-assembly-fixture` with this structure and
content (use a throwaway script or `cat >`; do not add it to the repo):

```
givn/specs/cap-a/cap-a.feature
  @keep
  Feature: cap a
    Scenario: Modified one
      Given a step

    @outline-tag
    Scenario Outline: Outline kept
      Given a value <v>
      Examples:
        | v |
        | 1 |
        | 2 |

    Scenario: Retired placeholder
      Given a step

    Scenario: Untouched
      Given a step whose text mentions Scenario: not a header
      """
      Scenario: inside a doc string
      """
      Given a step

    @first-tag
    @second-tag
    Scenario: Multi-tag kept
      Given a step

givn/specs/cap-b/cap-b.feature
  Feature: cap b
    Scenario: Shared title
      Given a step

    Scenario: Modified one
      Given a step in cap b, which delta one must not remove

givn/changes/one/specs/cap-a/cap-a.feature
  @givn.delta @cap-a
  Feature: cap a
    @givn.modified @wip
    Scenario: Modified one
      Given a new step

    @givn.removed
    Scenario: Retired placeholder
      Given this scenario is retired

givn/changes/two/specs/cap-b/cap-b.feature
  @givn.delta @cap-b
  Feature: cap b
    @givn.modified
    Scenario: Shared title
      Given a changed step
```

Run:

```
out=$(mktemp -d)
before=$(sha256sum /tmp/givn-assembly-fixture/givn/specs/cap-a/cap-a.feature \
                  /tmp/givn-assembly-fixture/givn/specs/cap-b/cap-b.feature)
python3 scripts/assemble_features.py --root /tmp/givn-assembly-fixture --out "$out"
after=$(sha256sum /tmp/givn-assembly-fixture/givn/specs/cap-a/cap-a.feature \
                 /tmp/givn-assembly-fixture/givn/specs/cap-b/cap-b.feature)
test "$before" = "$after"
grep -RFc 'Scenario: Modified one' "$out" | awk -F: '{s+=$2} END {print s}'   # 2: delta cap-a + permanent cap-b
grep -RF 'Given a step in cap b, which delta one must not remove' "$out"      # cap-b copy kept
grep -RFc 'Scenario: Shared title' "$out" | awk -F: '{s+=$2} END {print s}'   # 1
grep -RFc 'Scenario Outline: Outline kept' "$out" | awk -F: '{s+=$2} END {print s}' # 1
grep -RF 'Examples:' "$out"                                                   # present
grep -RF 'Scenario: inside a doc string' "$out"                               # preserved
grep -RF -B2 'Scenario: Multi-tag kept' "$out" | grep -E '@first-tag|@second-tag' # both tags preserved
! grep -RFq 'Scenario: Retired placeholder' "$out"                            # absent
```

Then prove the loud failures on copies (never on the real corpus):

```
cp -R /tmp/givn-assembly-fixture /tmp/givn-assembly-bad-title
sed -i 's/Scenario: Modified one/Scenario: Wrong title/' \
  /tmp/givn-assembly-bad-title/givn/specs/cap-a/cap-a.feature
python3 scripts/assemble_features.py --root /tmp/givn-assembly-bad-title --out "$(mktemp -d)"
# non-zero, message names "Modified one" and givn/specs/cap-a/cap-a.feature

cp -R /tmp/givn-assembly-fixture /tmp/givn-assembly-duplicate
printf '\n  Scenario: Modified one\n    Given a duplicate\n' \
  >> /tmp/givn-assembly-duplicate/givn/specs/cap-a/cap-a.feature
python3 scripts/assemble_features.py --root /tmp/givn-assembly-duplicate --out "$(mktemp -d)"
# non-zero, message names the duplicate title and file
```

Paste the fixture assertions and both non-zero failure messages:

```
modified-one count: 2
f0002.feature: Given a step in cap b, which delta one must not remove
shared-title count: 1
outline count: 1
Examples: present
Scenario: inside a doc string preserved
@first-tag / @second-tag preserved
Retired placeholder absent
fixture assertions passed
bad-title exit=1: 'Modified one' is not found in givn/specs/cap-a/cap-a.feature
duplicate exit=1: 'Modified one' occurs 2 times; cannot remove it unambiguously
```

### 0.6 Real-corpus smoke

```
root=$(scripts/assemble-features.sh)
echo "assembled root: $root"
for title in \
  'A prepared release offers one archive for every supported platform' \
  'A platform whose executable was not built stops the release' \
  "Each archive's checksum covers that archive" \
  'An unreadable executable stops the preparation'
do
  printf '%s: ' "$title"
  grep -RFc "Scenario: $title" "$root" | awk -F: '{s+=$2} END {print s+0}'
done
! grep -RFq 'Scenario: Maintainer prepares a release and runs the archived executable' "$root"
grep -RFq 'Scenario: An executable for the previously offered Linux platform is refused' "$root"
grep -RFq 'Scenario: Maintainer prepares a Windows release and inspects the archived executable' "$root"
grep -RhE '^[[:space:]]*Scenario( Outline)?:' "$root"/*.feature | wc -l
git diff --stat -- givn/specs          # must be empty
```

Paste the output. Required: each modified title exactly 1; retired placeholder
absent; both added scenarios present; a single total scenario count recorded.
(`root` is temporary; remove it after capturing the evidence.)

```
assembled root: /tmp/tmp.tdP2BzQ8Ps
A prepared release offers one archive for every supported platform: 1
A platform whose executable was not built stops the release: 1
Each archive's checksum covers that archive: 1
An unreadable executable stops the preparation: 1
retired placeholder absent
linux refusal present
new e2e present
112
(no output from git diff --stat -- givn/specs)
```

```
givn commit -m "test(release): Prepare the feature assembly for the narrowed platform set" scripts/assemble_features.py scripts/assemble-features.sh givn/changes/windows-only-release
```

Revision: `49e52c0109283f4e7f378ee3d84740050697b73b`

- [x] 0.8 `git show --stat <revision>` touches the two scripts and this file
      (paste the stat line).

```
49e52c0 test(release): Prepare the feature assembly for the narrowed platform set
 scripts/assemble-features.sh                       |  19 +-
 scripts/assemble_features.py                       | 207 +++
 givn/changes/windows-only-release/tasks.md         | 1520 ++++++++++++++++++++
```

---

## Task B1 — Public text and public gates

- [x] Rewrite the README download section.
- [x] Update the issue form and security policy.
- [x] Strengthen `scripts/public-text-check.sh` (presence + negative checks).
- [x] Prove the gate fails on the old text and passes on the new text.
- [x] Commit and record the revision.

### Changes

1. `README.md`, section `## Download a release` through the end of
   `### macOS: the one-time approval` (immediately before `## Prerequisites`):
   delete that whole block and write exactly this replacement. Keep each of the
   three promise sentences on **one physical line** — the gate uses line-based
   `grep -Fq`, and a wrapped sentence is a missing promise.

   ````markdown
   ## Download a release

   Every version tag publishes one release at the stable address
   <https://github.com/kressnerd/duty2cal/releases/latest>, and every release
   holds exactly one archive with its checksum file beside it. The only
   supported platform is Windows x86_64:

   | Your machine | Archive |
   |---|---|
   | Windows x86_64 | `duty2cal-<version>-windows-x86_64.tar.gz` |

   Replace `<version>` with the released version, for example `0.1.0`. The
   archive is a `.tar.gz` holding only `duty2cal.exe`, so one extraction route
   serves the one supported platform:

   ```powershell
   tar -xzf duty2cal-0.1.0-windows-x86_64.tar.gz
   .\duty2cal.exe .\2026-06.pdf
   ```

   Other platforms are not supported or distributed; the source can be built with a Rust toolchain. A platform may be added back when users request it.

   ### Verify what you received

   Verify the archive's checksum before extracting it, and verify the build
   provenance and the bill of materials of the file in hand with the
   [GitHub CLI](https://cli.github.com/):

   ```powershell
   Get-FileHash duty2cal-0.1.0-windows-x86_64.tar.gz -Algorithm SHA256
   ```

   The digest must match the one in
   `duty2cal-0.1.0-windows-x86_64.tar.gz.sha256`. In a POSIX shell the checksum
   file verifies itself:

   ```sh
   sha256sum --check duty2cal-0.1.0-windows-x86_64.tar.gz.sha256
   gh attestation verify duty2cal-0.1.0-windows-x86_64.tar.gz --repo kressnerd/duty2cal
   ```

   `gh attestation verify` proves that the archive was built by this
   repository's release automation from the tagged revision, and shows the bill
   of materials the build was made from.

   Already-published releases keep their original assets and links; only future releases are Windows-only.
   ````

   Do not carry over the Linux/macOS rows, the non-Windows `./duty2cal`
   example, the sentence containing `all five platforms`, the `chmod +x` note,
   the `shasum -a 256` example, or the macOS Gatekeeper approval section.
2. `.github/ISSUE_TEMPLATE/bug.yml`: dropdown `platform.options` becomes exactly
   `Windows x86_64` and `built from source`; delete the Linux and macOS options.
3. `SECURITY.md`: delete the out-of-scope bullet `The Gatekeeper approval the
   unsigned macOS archives need; the [README](README.md) documents it.` Leave
   the rest unchanged (reviewed by hand; no gate reads this file).
4. `scripts/public-text-check.sh`: keep the `[readme]` interface and the
   per-promise failure naming; add a `forbid()` that fails and names the
   pattern. Exact checks from `design.md` §"Public text and documentation":

   | Kind | Name | Pattern |
   |---|---|---|
   | require | clone-address | `git clone https://github\.com/kressnerd/duty2cal\.git` |
   | require | windows-archive | `duty2cal-[^[:space:]]*-windows-x86_64\.tar\.gz` |
   | require | checksum-command | `sha256sum --check` |
   | require | checksum-command-powershell | `Get-FileHash` |
   | require | provenance-command | `gh attestation verify` |
   | require | licence-statement | `MPL-2\.0` |
   | require | licence-text-link | `LICENSES/MPL-2\.0\.txt` |
   | require | unicode-data-licence | `Unicode-3\.0` |
   | require | unicode-data-licence-link | `LICENSES/Unicode-3\.0\.txt` |
   | require (fixed string) | source-route-statement | `Other platforms are not supported or distributed; the source can be built with a Rust toolchain.` |
   | require (fixed string) | platform-request-statement | `A platform may be added back when users request it.` |
   | require (fixed string) | published-history-statement | `Already-published releases keep their original assets and links; only future releases are Windows-only.` |
   | forbid | non-windows-archive | `duty2cal-[^[:space:]]*-(linux-x86_64\|linux-arm64\|macos-arm64\|macos-x86_64)\.tar\.gz` |
   | forbid | non-windows-download-row | `^\|[[:space:]]*(Linux x86_64\|Linux arm64\|macOS arm64( \([^)]*\))?\|macOS x86_64( \([^)]*\))?)[[:space:]]*\|` |

   Use `grep -Fq` for the three fixed strings and `grep -Eq` for the rest.
   A `forbid` match must print `forbidden text: <name>` and exit 1; a missing
   `require` keeps printing `missing promise: <name>`. Failure summary names
   the readme file.

   Paste these exact EREs into the script (the `\|` in the table above is
   Markdown escaping; these are the shell strings, where `\|` is a literal
   pipe and the bare `|` characters are alternation):

   ```
   require 'clone-address' 'git clone https://github\.com/kressnerd/duty2cal\.git'
   require 'windows-archive' 'duty2cal-[^[:space:]]*-windows-x86_64\.tar\.gz'
   require 'checksum-command' 'sha256sum --check'
   require 'checksum-command-powershell' 'Get-FileHash'
   require 'provenance-command' 'gh attestation verify'
   require 'licence-statement' 'MPL-2\.0'
   require 'licence-text-link' 'LICENSES/MPL-2\.0\.txt'
   require 'unicode-data-licence' 'Unicode-3\.0'
   require 'unicode-data-licence-link' 'LICENSES/Unicode-3\.0\.txt'
   require_fixed 'source-route-statement' 'Other platforms are not supported or distributed; the source can be built with a Rust toolchain.'
   require_fixed 'platform-request-statement' 'A platform may be added back when users request it.'
   require_fixed 'published-history-statement' 'Already-published releases keep their original assets and links; only future releases are Windows-only.'
   forbid 'non-windows-archive' 'duty2cal-[^[:space:]]*-(linux-x86_64|linux-arm64|macos-arm64|macos-x86_64)\.tar\.gz'
   forbid 'non-windows-download-row' '^\|[[:space:]]*(Linux x86_64|Linux arm64|macOS arm64( \([^)]*\))?|macOS x86_64( \([^)]*\))?)[[:space:]]*\|'
   ```

### Evidence

Negative control (run against the previous README from the committed revision):

```
$ git show HEAD:README.md > /tmp/old-readme.md
$ scripts/public-text-check.sh /tmp/old-readme.md
missing promise: source-route-statement
missing promise: platform-request-statement
missing promise: published-history-statement
forbidden text: non-windows-archive
forbidden text: non-windows-download-row
public-text-check: FAILED - 5 of 14 promise(s) violated in /tmp/old-readme.md
exit=1
```

After the rewrite:

```
$ scripts/public-text-check.sh
$ grep -nE 'duty2cal-[^[:space:]]*-(linux-x86_64|linux-arm64|macos-arm64|macos-x86_64)\.tar\.gz' README.md
$ grep -nE '^\|[[:space:]]*(Linux x86_64|Linux arm64|macOS arm64|macOS x86_64)' README.md
$ grep -nE 'chmod \+x|shasum|Gatekeeper|all five platforms' README.md
```

```
$ scripts/public-text-check.sh
public-text-check: clean - all 14 public-text promises present in /home/dan/dev/PRIVATE/duty2ical/README.md
$ grep -nE 'duty2cal-[^[:space:]]*-(linux-x86_64|linux-arm64|macos-arm64|macos-x86_64)\.tar\.gz' README.md
(no output, exit 1)
$ grep -nE '^\|[[:space:]]*(Linux x86_64|Linux arm64|macOS arm64|macOS x86_64)' README.md
(no output, exit 1)
$ grep -nE 'chmod \+x|shasum|Gatekeeper|all five platforms' README.md
(no output, exit 1)
```

```
$ grep -nA8 'id: platform' .github/ISSUE_TEMPLATE/bug.yml
$ grep -n 'Gatekeeper' SECURITY.md
```

```
$ grep -nA8 'id: platform' .github/ISSUE_TEMPLATE/bug.yml
24:    id: platform
25-    attributes:
26-      label: Platform
27-      description: Which archive or build did you run?
28-      options:
29-        - Windows x86_64
30-        - built from source
31-    validations:
32-      required: true
$ grep -n 'Gatekeeper' SECURITY.md
(no output, exit 1)
```

### Commit

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Update the public text and its gate." README.md .github/ISSUE_TEMPLATE/bug.yml SECURITY.md scripts/public-text-check.sh givn/changes/windows-only-release/tasks.md
```

Revision: `880eed0b0cd870dcb6281b9b5c76278788805dc2`

---

## Task B2 — Release workflow narrowing

- [x] Narrow the `build` matrix to one Windows entry.
- [x] Delete the non-Windows start step and the musl linker configuration.
- [x] Validate the YAML and the structural assertions.
- [x] Commit and record the revision.

### Changes

`.github/workflows/release.yml`:

- `build.strategy.matrix.include` becomes exactly one entry:
  ```
  - platform: windows-x86_64
    runner: ubuntu-latest
    target: x86_64-pc-windows-gnu
    binary: duty2cal.exe
    apt: mingw-w64
  ```
  Delete the `linker: ""` field, the `linux-x86_64`, `linux-arm64`,
  `macos-arm64` and `macos-x86_64` entries.
- In `Build the release executable`, delete the `env:` block containing
  `CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER` and
  `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER` and its comment; the step
  keeps `run: cargo build --release --target ${{ matrix.target }}`.
- `Install the linker this target needs` keeps installing `matrix.apt` only;
  no linker variable is passed.
- Delete the entire `Start the built executable` step (it was guarded by
  `matrix.platform != 'windows-x86_64'` and never ran for the remaining entry).
- Keep `smoke-windows` unchanged (it starts the cross-built `duty2cal.exe` on
  `windows-latest` and is the real execution evidence), and keep `prepare`,
  `publish`, `verify` unchanged.
- Update stale comments that still speak of several platforms/five builds.

### Evidence

```
$ if command -v docker >/dev/null; then docker run --rm -v "$PWD:/repo" -w /repo rhysd/actionlint:1.7.12; else echo 'docker unavailable'; fi
$ python3 -c 'import yaml; yaml.safe_load(open(".github/workflows/release.yml")); print("yaml ok")'
$ grep -c 'platform: windows-x86_64' .github/workflows/release.yml
$ grep -nE 'linux-x86_64|linux-arm64|macos-arm64|macos-x86_64|^[[:space:]]*linker:|CARGO_TARGET_|Start the built executable' .github/workflows/release.yml
$ grep -n 'smoke-windows' .github/workflows/release.yml
```

```
$ nix-shell -p actionlint --run 'actionlint -color .github/workflows/release.yml'
(no output, exit 0)
$ docker --version
error: command not found: docker
$ python3 -c 'import yaml; yaml.safe_load(open(".github/workflows/release.yml")); print("yaml ok")'
ModuleNotFoundError: No module named 'yaml'
$ grep -c 'platform: windows-x86_64' .github/workflows/release.yml
1
$ grep -nE 'linux-x86_64|linux-arm64|macos-arm64|macos-x86_64|^[[:space:]]*linker:|CARGO_TARGET_|Start the built executable' .github/workflows/release.yml
(no output, exit 1)
$ grep -n 'smoke-windows' .github/workflows/release.yml
27:          # distribution constraint) and started by the smoke-windows job. It
50:  smoke-windows:
88:    needs: [build, smoke-windows]
```

### Commit

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Narrow the release workflow to the one Windows build." .github/workflows/release.yml givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

## Task B3 — Durable documentation verification (and the authorized R-012 fix)

The fourteen arc42 files are already modified in the working tree by the
arc42-docs artifact. This task verifies them, applies one authorized one-line
correction (below), and commits them. Do not rewrite the chapters otherwise.

- [x] Run the stale-promise greps and record the survivors.
- [x] Apply the authorized R-012 correction; re-run the greps.
- [x] Commit the docs tree and record the revision.

### Authorized correction (explicitly approved by the main agent)

`docs/arc42/11-risks-and-technical-debt.md` line 18 (R-012) ends its Measure
cell with:

> The README states which releases need the one-off `chmod +x`, and the
> packaging fix (forcing the mode) is in place for later versions; the
> immutable `0.1.0` archives stay as published.

`design.md` deletes the `chmod +x` note from the README, so this contradicts the
design. Replace the whole sentence beginning `The README states` through the
end of the cell with exactly:

> The `0.1.0` archives stay as published; only future releases are Windows-only. The packaging fix (forcing the mode) is in place for later versions.

The rest of the R-012 row (the immutability statement) stays. Do not make any
other documentation edit.

### Verification commands

```
$ grep -RnE 'five (supported )?platform|five executables|five downloads|Gatekeeper|shasum|chmod \+x' docs/arc42 --include='*.md'
$ grep -RnE 'linux-x86_64|linux-arm64|macos-arm64|macos-x86_64' docs/arc42 --include='*.md'
$ grep -RnE 'README (states|documents)' docs/arc42 --include='*.md'
$ grep -RnE 'Supported platform|Release archive' docs/arc42/12-glossary.md
$ git diff --stat -- docs/arc42
```

```
--- five/gatekeeper ---
docs/arc42/adr/ADR-0003-release-channel-and-platforms.md:13:> platform set is narrowed from five platforms to Windows x86_64. The release
--- platform names ---
docs/arc42/adr/ADR-0003-release-channel-and-platforms.md:142:`linux-x86_64` — are covered by the `publish-public-release` scenarios.
--- README claims ---
(no output, exit 1)
--- glossary ---
12-glossary.md:47 Release archive: one per version, the Windows x86_64 archive
12-glossary.md:62 Supported platform: Windows x86_64 only; Linux is CI/test host
12-glossary.md:115/118 register of the promoted terms
--- diffstat ---
14 files changed, 207 insertions(+), 113 deletions(-)
```

- `docs/arc42/12-glossary.md` "five terms added by the change to export
  released roster calendars" — unrelated historical promotion note.
- `docs/arc42/11-risks-and-technical-debt.md` TD-002 "five overlong lines" —
  historical oracle note.
- `docs/arc42/adr/adr-template.md` "five mandatory" dimensions — template.
- ADR-0003's context/considered-options/superseded-set passages that name the
  earlier five-platform decision — the amendment's deliberate history.
- `docs/arc42/07-deployment-view.md:69` and R-010/TD-006's "already-published
  Linux and macOS archives stay as released" — intentional immutability.
- `docs/arc42/02-architecture-constraints.md` / `07-deployment-view.md` Linux
  build/test-host statements — intentional engineering context, not a download
  promise.
- R-019's statement that other platforms' source builds are unverified — the
  recorded risk.

Required: after the R-012 fix, every remaining Linux/macOS/`five` reference is
one of the survivors above. If any other concrete contradiction appears, stop,
report it, and do not edit the chapter.

### Commit

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Record the narrowed platform set in the durable documentation." docs/arc42 givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

# Scenario tasks (non-`@e2e` first, in `.feature` order)

The delta file is
`givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature`.
Its scenario order is: the four `@givn.modified` scenarios, the
`@givn.removed` placeholder (not a test), the `@givn.added @e2e` scenario, then
the `@givn.added` Linux-refusal scenario. The retired placeholder is never
assembled and needs no step; do not add one.

## Task C1 — A prepared release offers one archive for every supported platform

`@givn.modified` at lines 9–16.

- [ ] RED (remove `@wip`; fixture narrowed; production still five platforms)
- [ ] GREEN (production script narrowed to one platform)
- [ ] REFACTOR (stale wording removed; targeted run still zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### Design constraints (copied from `design.md`)

- Supported platform set is one canonical value in `scripts/prepare-release.sh`:
  `PLATFORMS='windows-x86_64'`.
- `<binaries>/windows-x86_64/duty2cal.exe` is the only required input.
- Archive is `duty2cal-0.1.0-windows-x86_64.tar.gz`, holding only
  `duty2cal.exe`, with `sha256sum` output (`…tar.gz.sha256`) beside it.
- The release text and change-log staging, the already-prepared refusal, the
  version validation, and the drift check are unchanged.
- The whitelist scan rejects any other platform directory; the completeness
  scan rejects a missing or unreadable `duty2cal.exe`, naming Windows x86_64,
  and stages nothing.
- Test helper: `SUPPORTED` becomes
  `[("windows-x86_64", "duty2cal.exe")]`;
  `supported_platforms()`/`executable_for()` keep their shape;
  `machine_platform()`/`machine_executable()` are deleted and callers use
  `supported_platform()`/`supported_executable()`.
- `executable_bytes()` returns the deterministic stand-in for every platform
  (the fixed bytes `stand-in executable for windows-x86_64\n`); it never reads
  `CARGO_BIN_EXE_duty2cal`. This keeps the local suite host-independent.
- `build_binaries()` keeps its shape. `every_supported_platform_except()`
  remains (used by no delta scenario) and keeps working with the one-entry set.
- `holds_exactly_these_archives` keeps reading the scenario's table; its
  failure message becomes
  `"the staged set is not the one Windows archive, its checksum, the release text and the change log"`.
- Do **not** write the two steps of the deleted duplicate scenario:
  `then the prepared release holds exactly the Windows archive for version <version>`
  and `then the prepared release holds no archive for a platform outside Windows x86_64`.
  The table scenario already asserts the exact staged set.
- RED driver: with the one-platform fixture and the old five-platform script,
  the preparation refuses for the missing `linux-x86_64` executable and stages
  nothing; the one-row exact set cannot be satisfied by the old set.

### RED

Remove only the `@wip` tag from line 9 (leave every other `@wip`).

Apply these **test-code** changes now (they define the new fixture; production
is untouched):

- `apps/cli/tests/steps/publish_public_release.rs`: `SUPPORTED` → one entry;
  add `pub(crate) fn supported_platform() -> &'static str` returning
  `"windows-x86_64"` and `pub(crate) fn supported_executable() -> &'static str`
  returning `"duty2cal.exe"`; delete `machine_platform()` and
  `machine_executable()`; rewrite `executable_bytes(platform)` to
  `format!("stand-in executable for {platform}\n").into_bytes()`;
  update the exact-set failure message.
- `apps/cli/tests/steps/e2e/publish_public_release.rs`: replace the
  `machine_platform`/`machine_executable` imports and call sites with
  `supported_platform`/`supported_executable` so the target compiles. The old
  e2e step bodies stay until C6; that old scenario is not assembled.
- Compile first:
  `cargo test -p duty2cal --test features_runner --no-run`
  (must succeed).
- Run only this scenario:
  `nix-shell -p git-cliff --run './run-tests.sh --name "A prepared release offers one archive for every supported platform"'`

Expected: non-zero; the preparation transcript contains
`no executable was built for platform linux-x86_64` (old five-platform
completeness scan), or the exact-set assertion fails because the old script
stages five archives. Paste the output.

Also capture, **before changing `scripts/prepare-release.sh`**, the RED runs for
C3 and C4 by name (they still carry `@wip`; `--name` does not filter tags):

```
nix-shell -p git-cliff --run './run-tests.sh --name "Each archive'"'"'s checksum covers that archive"'
nix-shell -p git-cliff --run './run-tests.sh --name "An unreadable executable stops the preparation"'
```

```
<paste the three command outputs here, labelled C1/C3/C4>
```

### GREEN

Production change: `scripts/prepare-release.sh`

- `PLATFORMS='windows-x86_64'`.
- Update its canonical-set comment so it names the one supported platform;
  leave `executable_for()` (`windows-x86_64 → duty2cal.exe`), the whitelist
  loop, the completeness loop, the staging loop and every refusal unchanged.

Files created/modified in GREEN:

- `scripts/prepare-release.sh`

Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "A prepared release offers one archive for every supported platform"'
```

```
<paste command and exit-0 output here>
```

### REFACTOR

- Grep the touched step file and script for stale five-platform wording
  (`five`, `all five`, `every supported platform` in comments) and adjust only
  comments/messages; no behaviour change. Do not touch the `HISTORY` fixture
  subject "Offer an archive for every supported platform".
- Re-run the targeted command; paste exit-0 output.

```
<paste command and exit-0 output here>
```

### COMMIT

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: A prepared release offers one archive for every supported platform" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature scripts/prepare-release.sh apps/cli/tests/steps/publish_public_release.rs apps/cli/tests/steps/e2e/publish_public_release.rs givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

## Task C2 — A platform whose executable was not built stops the release

`@givn.modified` at lines 18–23.

- [ ] RED (new Given step undefined, non-zero)
- [ ] GREEN (dedicated empty-binaries Given added; targeted run zero)
- [ ] REFACTOR (targeted run still zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### Design constraints (copied from `design.md`)

- The new Given `no executable was built for the supported platform` is a
  dedicated step that calls no build helper: it creates the empty `<binaries>`
  directory, so the scenario asserts the refusal rather than a filtered partial
  set.
- The existing Then `the preparation fails and names "windows-x86_64"` asserts
  the refusal names the supported platform; `the prepared release holds no
  archive` asserts nothing was staged.
- The production script's completeness refusal is retained unchanged and names
  Windows x86_64.
- This scenario is re-affirmed evidence, not a production RED driver; its RED
  is the undefined new step.

### RED

Remove only the `@wip` tag from line 18. Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "A platform whose executable was not built stops the release"'
```

Expected: non-zero because `Given no executable was built for the supported
platform` has no step definition. Paste the output (including the undefined
step text).

```
<paste command and non-zero output here>
```

### GREEN

Add to `apps/cli/tests/steps/publish_public_release.rs`:

```rust
#[given("no executable was built for the supported platform")]
async fn no_executable_built_for_the_supported_platform(w: &mut DutyWorld) {
    let dir = scenario_dir(w);
    let binaries = dir.join("binaries");
    let _ = std::fs::remove_dir_all(&binaries);
    std::fs::create_dir_all(&binaries).expect("create the empty binaries directory");
    let out = dir.join("out");
    w.binaries = Some(binaries);
    w.staged = Some(out);
}
```

No other production or step change. The step must not call `build_binaries` or
write any executable.

Files created/modified in GREEN:

- `apps/cli/tests/steps/publish_public_release.rs`

Compile first (must succeed):

```
cargo test -p duty2cal --test features_runner --no-run
```

Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "A platform whose executable was not built stops the release"'
```

```
<paste command and exit-0 output here>
```

### REFACTOR

- Confirm the step body has a real directory action (not empty), and that no
  executable is written.
- Re-run the targeted command; paste exit-0 output.

```
<paste command and exit-0 output here>
```

### COMMIT

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: A platform whose executable was not built stops the release" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature apps/cli/tests/steps/publish_public_release.rs givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

## Task C3 — Each archive's checksum covers that archive

`@givn.modified` at lines 25–29.

- [ ] RED (re-baselined: pre-GREEN capture, non-zero)
- [ ] GREEN (supported-archive assertion; targeted run zero)
- [ ] REFACTOR (targeted run still zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### Design constraints (copied from `design.md`)

- The checksum rule is unchanged: one `.sha256` file beside the one archive,
  its digest covers that archive, and the file names the archive it covers.
- The prepared-release fixture now stages the one supported executable.
- `design.md` classifies this scenario as re-baselined keep-true evidence:
  against the old script it failed at preparation because the fixture supplied
  only the Windows executable, not because the checksum rule changed. It becomes
  meaningful once C1's preparation and fixture agree; it is **not** a production
  RED driver.
- The step already performs a three-way check: the `sha256sum --check`
  transcript reports exactly one verified file, the checksum file names that
  archive, and `sha256_of(archive)` equals the digest. Keep those assertions.
- `staged_archive(w, platform)` resolves the archive; the scenario names
  `windows-x86_64`.

### RED (re-baselined; output captured in C1 RED)

Paste the C1-RED capture of this scenario (before `scripts/prepare-release.sh`
was narrowed). It must be non-zero at `Given a prepared release for version
0.1.0` because the old five-platform completeness scan rejects the one-platform
fixture. If the C1 RED capture was lost, re-derive it is **not** acceptable by
reverting production; report the gap instead of fabricating output.

```
<paste the C3 block captured in C1 RED here>
```

Remove only the `@wip` tag from line 25 now.

### GREEN

Add one assertion to
`the_checksum_matches_the_archive_it_names` in
`apps/cli/tests/steps/publish_public_release.rs`, after `archive` is determined:

```rust
assert_eq!(
    archive,
    staged_archive(w, supported_platform()),
    "the verified archive is not the supported platform's archive"
);
```

This ties the consumer-visible checksum to the one supported archive. No
production change.

Files created/modified in GREEN:

- `apps/cli/tests/steps/publish_public_release.rs`

Compile first (must succeed):

```
cargo test -p duty2cal --test features_runner --no-run
```

Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "Each archive'"'"'s checksum covers that archive"'
```

```
<paste command and exit-0 output here>
```

### REFACTOR

- Keep the assertion message specific; no behaviour change.
- Re-run the targeted command; paste exit-0 output.

```
<paste command and exit-0 output here>
```

### COMMIT

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: Each archive's checksum covers that archive" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature apps/cli/tests/steps/publish_public_release.rs givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

## Task C4 — An unreadable executable stops the preparation

`@givn.modified` at lines 31–36.

- [ ] RED (re-baselined: record the actual pre-GREEN result)
- [ ] GREEN (supported-platform assertion; targeted run zero)
- [ ] REFACTOR (targeted run still zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### Design constraints (copied from `design.md`)

- The readability refusal is unchanged: an unreadable
  `<binaries>/windows-x86_64/duty2cal.exe` fails the preparation naming Windows
  x86_64 and stages nothing.
- `an_unreadable_executable()` asserts the named platform is supported, stages
  the Windows executable and removes its read permission.
- `design.md` classifies this scenario as re-baselined keep-true evidence, not a
  production RED driver.

### RED / re-baseline

Remove only the `@wip` tag from line 31. Paste the actual result of the run
captured in C1 RED (before the script narrowed). Record exactly what happened:
with the one-platform fixture and the old script, the Windows executable is the
first `PLATFORMS` entry, so its unreadable path may already be refused before
the missing-Linux refusal. Do **not** claim a failure that did not occur.

```
<paste the C4 block captured in C1 RED here, with its actual exit status>
```

### GREEN

In `an_unreadable_executable()`, before staging, add:

```rust
assert!(
    supported_platforms().contains(&platform.as_str()),
    "the scenario names a supported platform: {platform}"
);
```

Keep `build_binaries(w, &supported_platforms())` (the one Windows executable)
and the `0o000` permission removal. No production change.

Files created/modified in GREEN:

- `apps/cli/tests/steps/publish_public_release.rs`

Compile first (must succeed):

```
cargo test -p duty2cal --test features_runner --no-run
```

Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "An unreadable executable stops the preparation"'
```

```
<paste command and exit-0 output here>
```

### REFACTOR

- No behaviour change; keep the failure message specific.
- Re-run; paste exit-0 output.

```
<paste command and exit-0 output here>
```

### COMMIT

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: An unreadable executable stops the preparation" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature apps/cli/tests/steps/publish_public_release.rs givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

## Task C5 — An executable for the previously offered Linux platform is refused

`@givn.added` at lines 52–58.

- [ ] RED (new Given undefined, non-zero)
- [ ] GREEN (new step; targeted run zero)
- [ ] REFACTOR (targeted run still zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### Design constraints (copied from `design.md`)

- Supported set is Windows x86_64 only. A `linux-x86_64` directory is outside
  the set and is refused by the whitelist scan, naming the directory and
  staging nothing, even though earlier releases offered that platform.
- New step `given an executable presented as the previously offered platform "<platform>"`:
  writes a directory and executable for an unsupported platform (the
  `linux-x86_64` case), reusing the existing unrecognised-platform fixture
  mechanics (`an_unrecognised_platform`: create the directory name and write a
  file named `duty2cal` directly). Do **not** call `executable_for(platform)`:
  it panics for a platform outside the one-entry `SUPPORTED`.
- The scenario first builds every supported platform (one), then adds the Linux
  executable. The preparation must fail naming `linux-x86_64` and stage no
  archive.
- Production RED driver: the old `PLATFORMS` accepted `linux-x86_64` and the
  preparation succeeded, so `the preparation fails and names "linux-x86_64"`
  was red. C1 has already narrowed `PLATFORMS`; this task's RED is the
  undefined step, and the GREEN proves the narrowed whitelist.

### RED

Remove only the `@wip` tag from line 52. Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "An executable for the previously offered Linux platform is refused"'
```

Expected: non-zero because
`Given an executable presented as the previously offered platform "linux-x86_64"`
has no step definition. Paste the output.

```
<paste command and non-zero output here>
```

### GREEN

Add to `apps/cli/tests/steps/publish_public_release.rs`:

```rust
#[given(regex = r#"^an executable presented as the previously offered platform "([^"]+)"$"#)]
async fn a_previously_offered_platform(w: &mut DutyWorld, platform: String) {
    assert!(
        !supported_platforms().contains(&platform.as_str()),
        "the scenario names a platform outside the supported set: {platform}"
    );
    let binaries = w
        .binaries
        .clone()
        .expect("the scenario prepared no executables");
    let dir = binaries.join(&platform);
    std::fs::create_dir_all(&dir).expect("create the previously offered platform directory");
    std::fs::write(
        dir.join("duty2cal"),
        b"stand-in executable for a previously offered platform\n",
    )
    .expect("write the previously offered executable");
}
```

Mirror the existing `an_unrecognised_platform` fixture mechanics; do not add a
compatibility branch that makes the old platform pass. No production change.

Files created/modified in GREEN:

- `apps/cli/tests/steps/publish_public_release.rs`

Compile first (must succeed):

```
cargo test -p duty2cal --test features_runner --no-run
```

Run:

```
nix-shell -p git-cliff --run './run-tests.sh --name "An executable for the previously offered Linux platform is refused"'
```

```
<paste command and exit-0 output here>
```

### REFACTOR

- If the new step duplicates `an_unrecognised_platform`, extract one private
  fixture helper used by both steps; the observable behaviour must not change.
- Re-run; paste exit-0 output.

```
<paste command and exit-0 output here>
```

### COMMIT

```
givn commit -m "feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: An executable for the previously offered Linux platform is refused" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature apps/cli/tests/steps/publish_public_release.rs givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

# E2E scenario (last, after every non-`@e2e` scenario is GREEN)

## Task C6 — Maintainer prepares a Windows release and inspects the archived executable

`@givn.added @e2e` at lines 42–50. Do not remove `@e2e`.

- [ ] RED (new e2e steps undefined; `--e2e` targeted run non-zero)
- [ ] GREEN (e2e steps written; targeted run zero; transcript committed)
- [ ] REFACTOR (targeted and full `@e2e` runs zero)
- [ ] COMMIT (revision recorded below)
- [ ] Evidence and revision filled in

### E2E setup (design-specific; no service to start)

- `design.md` §"Local runnability & digital twins": no local run command beyond
  `./run-tests.sh`, no isolated network, no container and **no digital twin**.
  The only external service is GitHub, and only GitHub's own automation talks
  to it; the prepare interaction is driven as a real local subprocess. There is
  therefore no compose stack to bring up — record that fact rather than
  inventing one.
- Framework: the same cucumber-rs `features_runner`; `verify.e2e_command` is
  `./run-tests.sh --e2e`, proven a strict subset of `verify.command` in Task
  0.3. Do not change it.
- Step location: `apps/cli/tests/steps/e2e/publish_public_release.rs`, declared
  by `apps/cli/tests/steps/e2e/mod.rs`. Strict mode is the proven
  `.fail_on_skipped()` from Task 0.1/0.2.
- Confirm the e2e scope reaches the runner and the real preparation before
  writing steps: the RED run below must show this scenario executing (and
  failing on undefined steps), not a missing feature root or skip.

### Design constraints (copied from `design.md`)

- Step file: `apps/cli/tests/steps/e2e/publish_public_release.rs`, one file per
  capability, separate from the in-process steps. `apps/cli/tests/steps/e2e/mod.rs`
  already declares it.
- Real interface: `scripts/prepare-release.sh` as a real subprocess with real
  arguments; assertions read the staged files it wrote and its transcript.
- Local infrastructure: a throwaway directory holding
  `<binaries>/windows-x86_64/duty2cal.exe` (a deterministic stand-in written by
  the step), the staged output directory, and `tar`/`sha256sum`. No network,
  no credentials, no container.
- `given a stand-in executable built for "windows-x86_64"`: writes the fixed
  stand-in bytes to `<binaries>/windows-x86_64/duty2cal.exe` directly; it does
  not build or copy a host binary. The stand-in is
  `stand-in executable for windows-x86_64\n` (the same `executable_bytes`
  bytes), so the archive digest is deterministic.
- `the preparation reports the archive and its checksum for "windows-x86_64"`:
  resolves `windows-x86_64` explicitly; asserts the transcript names
  `duty2cal-0.1.0-windows-x86_64.tar.gz` and the digest; asserts three-way
  equality: the digest reported in the transcript, the digest in the `.sha256`
  file, and `sha256_of(staged archive)`.
- The release-text assertion runs `cat release-notes.md` through `run_command`
  before asserting version 0.1.0, so the release-text line appears in the
  transcript. (Modify the shared
  `the_release_text_names_version` step in the non-e2e file accordingly.)
- `extracts the archive for "windows-x86_64"`: resolves Windows explicitly and
  runs `tar -xzf` through `run_command`.
- `runs_the_extracted_executable` and `prints_usage_and_reports_failure` are
  deleted; no local step executes the Windows binary. Execution evidence stays
  with the `windows-latest` `smoke-windows` jobs.
- New Then `the extracted archive holds exactly the executable "duty2cal.exe"`:
  runs `tar -tzf` through `run_command` so the member listing is in the
  transcript, then asserts the member list is exactly `["duty2cal.exe"]`.
- New Then `the extracted executable is the executable the preparation staged`:
  compares the extracted bytes with the staged
  `<binaries>/windows-x86_64/duty2cal.exe`, asserts equality, and is the final
  Then; it calls `commit_transcript(w)` after the assertion.
- `transcript_path()` follows the active change like `render_transcript_path()`:
  use `givn/changes/windows-only-release/` while it exists, otherwise
  `givn/archive/windows-only-release/`; slug
  `maintainer-prepares-a-windows-release-and-inspects-the-archived-executable`;
  file `evidence/visual/<slug>/transcript.txt`.
- `commit_transcript()` replaces the scenario directory with `<scenario-dir>`
  and appends the exit status. Perform **no digest scrub**: the stand-in is
  deterministic and is the only executable this e2e stages.
- The retired `@givn.removed` placeholder is never assembled and needs no step.

### RED

Remove only the `@wip` tag from line 42 (keep `@e2e`). Run:

```
nix-shell -p git-cliff --run './run-tests.sh --e2e --name "Maintainer prepares a Windows release and inspects the archived executable"'
```

Expected: non-zero because the new Given/Then steps are undefined. Paste the
output.

```
<paste command and non-zero output here>
```

### GREEN

Implement the e2e step changes above. Suggested concrete Rust:

- Step annotations (the feature text is authoritative):
  ```rust
  #[given(regex = r#"^a stand-in executable built for "([^"]+)"$"#)]
  #[then(regex = r#"^the preparation reports the archive and its checksum for "([^"]+)"$"#)]
  #[when(regex = r#"^the maintainer extracts the archive for "([^"]+)"$"#)]
  #[then(regex = r#"^the extracted archive holds exactly the executable "([^"]+)"$"#)]
  #[then("the extracted executable is the executable the preparation staged")]
  ```
  Delete the old `the_executable_built_for_this_machine`,
  `the_preparation_reports_archive_and_checksum`, `extracts_the_archive`,
  `runs_the_extracted_executable` and `prints_usage_and_reports_failure`
  definitions, replacing them with the annotated steps above.
- Import `supported_platform`/`supported_executable` (and the new stand-in
  helper) from the non-e2e module instead of the deleted `machine_*` helpers.
- Add a `pub(crate) fn stage_stand_in_executable(w: &mut DutyWorld, platform: &str)`
  in the non-e2e module that creates the scenario's `binaries/<platform>`
  directory, writes `format!("stand-in executable for {platform}\n")` as the
  platform's executable name, sets mode `0o755`, and sets `w.binaries` and
  `w.staged`; call it from the e2e Given. (It may reuse the private
  `scenario_dir`/`write_executable` helpers inside that module.)
- Three-way digest:
  ```rust
  let checksum = std::fs::read_to_string(out.join(format!("{archive}.sha256")))?;
  let digest = checksum.split_whitespace().next().expect("digest");
  assert!(w.command_transcript.contains(&archive));
  assert!(w.command_transcript.contains(digest));
  assert_eq!(sha256_of(&out.join(&archive)), digest);
  ```
- Member list via `run_command` + `tar -tzf`;
  bytes via `std::fs::read(extracted.join("duty2cal.exe"))` versus
  `std::fs::read(binaries.join("windows-x86_64").join("duty2cal.exe"))`.
- Final Then calls `commit_transcript(w)` after both assertions.
- Replace the body of the shared `the_release_text_names_version` step so the
  release text is printed into the transcript before the assertion:
  ```rust
  let path = release_text(w);
  let mut command = Command::new("cat");
  command.arg(&path);
  run_command(w, &mut command);
  assert_eq!(w.exit_status, Some(0), "reading the release text failed: {}", w.command_transcript);
  assert!(
      w.command_transcript.contains(&version),
      "the release text does not name version {version}: {}",
      w.command_transcript
  );
  ```
  All non-e2e scenarios that use this step keep passing: the `cat` transcript
  contains the file's text, which names the version.

Files created/modified in GREEN:

- `apps/cli/tests/steps/e2e/publish_public_release.rs`
- `apps/cli/tests/steps/publish_public_release.rs` (shared release-text step and
  the stand-in helper)

Compile first:
`cargo test -p duty2cal --test features_runner --no-run` (must succeed).

Run the targeted e2e scenario:

```
nix-shell -p git-cliff --run './run-tests.sh --e2e --name "Maintainer prepares a Windows release and inspects the archived executable"'
```

```
<paste command and exit-0 output here>
```

Then confirm the committed evidence exists and contains the required lines:

```
$ test -f givn/changes/windows-only-release/evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt
$ cat givn/changes/windows-only-release/evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt
```

```
<paste path existence and transcript here>
```

Required transcript contents: the preparation command and arguments, the
reported `duty2cal-0.1.0-windows-x86_64.tar.gz` and digest, the release-text
line naming version 0.1.0, the `tar -tzf` listing with exactly `duty2cal.exe`,
the extraction command, the `<scenario-dir>` placeholder (no `/tmp` path), and
the final exit status.

### REFACTOR

- Remove the now-dead old e2e imports/helpers; ensure no empty step bodies
  remain; keep `commit_transcript` free of digest scrubbing.
- Re-run the targeted e2e scenario and the whole e2e scope:

```
nix-shell -p git-cliff --run './run-tests.sh --e2e --name "Maintainer prepares a Windows release and inspects the archived executable"'
nix-shell -p git-cliff --run './run-tests.sh --e2e'
```

```
<paste both exit-0 outputs here>
```

### COMMIT

```
givn commit -m "test(e2e): Offer one downloadable archive for Windows, the only platform the project distributes.

Scenario: Maintainer prepares a Windows release and inspects the archived executable" givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature apps/cli/tests/steps/publish_public_release.rs apps/cli/tests/steps/e2e/publish_public_release.rs givn/changes/windows-only-release/evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt givn/changes/windows-only-release/tasks.md
```

Revision: `<paste revision id>`

---

# Final acceptance (after C6)

- [ ] All six scenario boxes checked, each with RED/GREEN/REFACTOR output,
      production-file list, and revision id.
- [ ] No `@wip` remains in the delta feature (`grep -n '@wip'` prints nothing).
- [ ] `nix-shell -p git-cliff --run './run-tests.sh'` exits 0 on the full
      assembled corpus; paste the final summary line.
- [ ] `nix-shell -p git-cliff --run './run-tests.sh --e2e'` exits 0; paste the
      final summary line and confirm its scenario count is strictly less than
      the full-suite count recorded in Task 0.
- [ ] `scripts/public-text-check.sh` exits 0.
- [ ] No file under `givn/specs/**` was modified (`git diff --stat -- givn/specs`
      is empty).
- [ ] Step files contain no empty bodies and no `machine_platform` /
      `machine_executable` references.
- [ ] The commit subjects and bodies follow the rule: release-note subject for
      every scenario revision, scenario title in the body; setup uses
      `test(release): Prepare the feature assembly for the narrowed platform set`;
      e2e uses `test(e2e): <release note>` as the served instruction directs.

## Commit plan at a glance

| Task | Subject | Contents |
|---|---|---|
| 0 | `test(release): Prepare the feature assembly for the narrowed platform set` | assembly helper + shell delegation; initial change artifacts (proposal/design/specs/tasks) |
| B1 | `feat(publish-public-release): Offer one downloadable archive for Windows, the only platform the project distributes.` | README, bug form, SECURITY, public-text gate |
| B2 | same release-note subject | release workflow |
| B3 | same release-note subject | arc42 docs (incl. authorized R-012 clause fix) |
| C1 | same release-note subject + scenario title body | one archive / one platform; preparation script |
| C2 | same | missing executable STOPS |
| C3 | same | checksum covers archive |
| C4 | same | unreadable executable STOPS |
| C5 | same | Linux refused |
| C6 | `test(e2e): Offer one downloadable archive for Windows, the only platform the project distributes.` + scenario title body | e2e stand-in/archive/transcript |

No other files are expected. Do not modify the converter crates, `run-tests.sh`,
`measure-coverage.sh`, `features_runner.rs`, `givn/commands.yaml`, or any file
under `givn/specs/**`.
