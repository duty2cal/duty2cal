# Review — export-released-roster-calendar

Stage: `tasks-done` → `review`. This pass covers the producer-layout reader rework and the re-shaped,
invented fixtures; the archived review record lives in the change's earlier sections of `tasks.md`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -S"@e2e" -- givn/changes/…/specs/` | CLEAN — the only commit that ever touched the tag is the planning commit; the single `@e2e` scenario still carries it and no tag was removed |
| 1 | Empty or no-op step bodies | grep for `{}`, bare `pass`/`return`, whitespace-only bodies across `apps/cli/tests/steps/**/*.rs` | CLEAN — no match |
| 2 | Every `[x]` task has a commit touching production source | 186 boxes, all with a recorded revision id; sampled `8166734` (4 src files), `053163f` (4), `d81f34a` (1), `d59cd70` (5), `fb164c8` (0) | CLEAN — `fb164c8` is the task whose record states a GREEN with no production change (spec + step definitions only), which is the intended disposition, not a stub |
| 3 | Every component design.md names exists | design.md's modules vs the tree | CLEAN — `duty2cal-core` (domain, application, adapters/ics), `duty2cal-pdf`, `apps/cli` |
| 4 | Strict-mode proof present with output | `tasks.md:13-35` | CLEAN — the temporary `strict-mode-proof.feature` failed under strict mode before any scenario existed, and the e2e filter is proven a strict subset (51 vs 1) |
| 5 | `@e2e` Then steps assert the real interface | delegated to the conformance pass | see *Conformance checks* below |
| 6 | Driving mechanism is the real interface, not an in-process shortcut | delegated | see below (this capability has no browser interface: a CLI subprocess is the interface) |
| 7 | Exactly one `@e2e` implementation for the capability | tree search, including untracked files | CLEAN — one e2e step file, and it is the one `verify.e2e_command` binds |
| 10 | `verify.command` vs `verify.e2e_command` isolation | `givn/commands.yaml`: `./run-tests.sh` vs `./run-tests.sh --e2e`; observed 51 scenarios vs 1 | CLEAN — the commands differ and the e2e run reports strictly fewer scenarios |
| 11 | Implementation matches what design.md names | file split, tag filter, runner commands | delegated |
| 13 | Interaction coverage cross-reference (inventory ↔ matrix ↔ scenarios ↔ driver) | delegated | see below |
| 14 | Coverage measurement validity | `measure_command` then `merge_command`, fresh output | CLEAN — see the measurement note below |

## Coverage measurement (check 14)

The measurement was **invalid** at the start of this review and is now fixed and re-run:

- `./measure-coverage.sh` failed with `error: no such command: llvm-cov`, and the merge command then
  combined reports from 12 Oct — a stale output, which check 14(e) rejects.
- Fixed by providing the toolchain: `nix-shell -p rustc cargo cargo-llvm-cov llvmPackages.llvm` with
  `LLVM_COV`/`LLVM_PROFDATA` exported from that shell. Both runners instrument the production processes
  and write separate Cobertura reports, and the merge command combines both.
- Fresh result: **885 production lines, 830 covered, 55 uncovered** across `crates/` and `apps/cli/src`
  (test-only paths excluded), produced by the merge command from the two fresh reports.

## Coverage classification (55 uncovered production lines)

Every gap is classified into exactly one bucket. There is **no dead code** to delete and **no
hard-to-test exception**: all 55 lines are **missing test coverage** (bucket 2), each named with the
scenario it needs.

| File | Lines | What the uncovered code is | Scenario it needs |
|---|---|---|---|
| `crates/duty2cal-core/src/application/mod.rs` | 136-138 | the required-column failure ("the detail table carries no {label} column") | a plan whose header omits a required label fails with that message and a numbered diagnostic |
| | 156 | an arrival fragment with no matching departure | an arrival-only row fails with "no unique compatible partner" |
| | 215-219 | a continuation row with no dated row above it | a plan whose first detail row prints no date fails, naming the reason |
| | 222-223 | an unreadable date cell | a row whose date cell cannot be read fails, naming the cell |
| | 269-274 | a `Layover:` marker whose station sits in the following column | a layover marker with the station in the next cell still yields the layover station |
| | 347-349 | a `Layover:` marker with no station at all | a stationless layover marker fails explicitly |
| | 381 | a row that prints in a body column but carries neither code nor route | such a row fails explicitly rather than being dropped |
| | 402 | a split leg where only the departure prints an optional field | a leg whose departure alone prints the timezone difference keeps that value |
| `crates/duty2cal-pdf/src/lib.rs` | 106, 132, 142 | the header-continuation exit, the gap branch, the unsupported-family rejection | a plan with no detail header fails as an unsupported family; material below the table is ignored |
| | 289-312 | the date-resolution guards (no heading month, no date band, unexpected token count, unparsable day, unknown weekday, weekday mismatch, invalid calendar date) | a date cell whose weekday contradicts the day is not accepted as timing; a plan with no heading month fails explicitly |
| | 334 | the joined month-and-year stamp (`January2099`) | a heading whose month touches its year still supplies the month |
| | 361, 379, 385, 394, 403-404 | the footer fallbacks (no `Created` stamp, ISO stamp, unparsable compact stamp) | a plan without a revision footer falls back to the construction clock; an ISO-stamped footer is read |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 85-89, 125-126 | text escaping (`\\`, newline, carriage return) and the second fold boundary | a description carrying a backslash or newline escapes correctly; a long description folds twice |
| `crates/duty2cal-core/src/domain/mod.rs` | 179-181 | the date accessor for travel, availability and non-flight duty | a scenario that reads the date of each of those entries |
| `apps/cli/src/main.rs` | 28, 48-49, 55, 67, 102 | no-argument usage, unreadable source, no unused calendar name, no unused report name | invocation without an argument prints usage and exits 2; an unreadable source, an exhausted calendar name and an exhausted report name each fail explicitly |
| `apps/cli/src/fs.rs` | 26, 46 | the fallback arms of the path helpers | a source whose name yields no usable calendar or report name |

## README impact decision

`README-IMPACT: updated - Description, Quick start, Prerequisites, Usage, Contributing`

## Sign-off

Not yet: the conformance tables (checks 5, 6, 11, 13, arc42 implementation conformance, ubiquitous
language) and the coverage dispositions above must land first. `REVIEW: PASS` follows both.
