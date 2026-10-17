# Design: migrate-0-8-4-to-0-9-1

## Migration metadata

- source_version: `0.8.4`
- target_version: `0.9.1`
- upgrade_commit: `76f1602771c6bf7c4430ac691657cff1e0a5b86c`
- migration_type: aggregate maintenance change without a product specification

## Execution order

Work the following phases strictly in sequence. Follow each bundle's own completion rules; a bundle may direct that the repairs run as named follow-up changes. The complete sequence is also recorded in `migration.yaml`.

### Phase 1: migrate-0-8-0-to-0-9-0 (0.8.0 -> 0.9.0)

# Migration Design: 0.8.4 to 0.9.1

Self-contained procedure for the project LLM. You have no access to the Givn
source repository; everything required is in this document. The managed
refresh and the `givn_config_version` bump are already done by `givn upgrade`
(Commit 76f1602771c6bf7c4430ac691657cff1e0a5b86c); this maintenance change carries the
project-specific work and intentionally has no product specification.

## Execution model

Unlike the 0.7.0-to-0.8.0 bundle, this migration carries its work directly: it
records surviving project rules in the project-owned `givn/config.yaml`. The
inventory below finds no surviving rule, so Phase 2 is a no-op: this change
edits no project file and only records the evidence and the verification. There
are no corpus repairs.

- Work the phases in order. Record the inventory as a table in this change's
  `design.md` (append it below this bundle text).
- Derive this change's tasks from the inventory rows before implementing; the
  change runs through the normal gates (design-review before tasks, review,
  archive), and its tasks evidence the verification below.
- When a disposition is not derivable from the project's own artifacts, stop
  and ask the human — one question at a time, with the old rule text and the
  candidate dispositions.
- If the project never customized a generated rule file and did not hand-edit
  the managed AGENTS.md block, record the "none found" finding with the file
  list you checked; the change then completes with the verification only.

## Preflight

1. Confirm `givn/config.yaml` declares
   `givn_config_version: "0.9.1"` and that this maintenance
   change exists (its id is `migrate-0-8-4-to-0-9-1`,
   or the aggregate id of the route that produced it).
2. Confirm the checkout is clean and committed before the first edit.
3. Read `skills.targets` in `givn/config.yaml` (default `opencode`). The
   generated rule directories are `<target>/skills/givn-*/SKILL.md` for every
   target, and `<target>/commands/givn-*.md` plus `grill-*.md` for the
   command-owning targets (`opencode`, `omp`).
4. Confirm the pre-upgrade revision: `76f1602771c6bf7c4430ac691657cff1e0a5b86c^` is the revision
   before the managed refresh and holds the old generated files. If the
   generated rules were never tracked (the default `rules: local-only`
   policy), the old content is not recoverable from history — ask the human
   for any project-specific rules they kept, and treat "none" as the finding.

## Phase 1 — Inventory the displaced project rules

1. List the managed rule paths present in the pre-upgrade revision, filtered
   to the configured targets, for example:

   ```sh
   git ls-tree -r --name-only 76f1602771c6bf7c4430ac691657cff1e0a5b86c^ \
     | grep -E '(skills/givn-[^/]+/SKILL\.md|commands/(givn|grill)-[^/]+\.md)$'
   ```

   A Fossil project uses the equivalent revision listing; if no listing is
   available, ask the human to supply the previous files.

2. Map every old file to its instruction id:

   | Installed file | Instruction id |
   | --- | --- |
   | `givn-propose` | `proposal` |
   | `givn-spec` | `specs` |
   | `givn-design` | `design` |
   | `givn-design-review` | `design-review` |
   | `givn-tasks` | `tasks` |
   | `givn-implement` | `tasks` (both files route to the same instruction) |
   | `givn-review` | `review` |
   | `givn-archive` | `archive` |
   | `givn-explore` | `explore` |
   | `givn-ideation` | `ideation` |
   | `givn-steps` | `steps` |
   | `givn-reverse-engineer` | `reverse-engineer` |
   | `givn-characterize` | `characterize` (reverse review: `review-reverse`) |
   | `givn-domain-modeling` (when enabled) | `domain-modeling` |
   | `givn-dev-principles` (when enabled) | `dev-principles` |
   | `grill-me` / `grill-with-docs` (when enabled) | `grill-me` / `grill-with-docs` |

3. For each old file, read its content from the pre-upgrade revision and the
   served instruction (`givn instructions <id> --change <this-change-id>`).
   Collect every rule statement the old file carries and the served content
   does not. Compare semantically: built-in rules may be reworded, moved
   between sections, or merged — that is not a displacement; a statement with
   no counterpart in the served content is.

4. Classify every collected statement:

   - **Project rule to keep** → an extension entry under the mapped
     instruction id (Phase 2). This includes house conventions, domain rules,
     and local constraints.
   - **Stale built-in text** → drop it; record the reason in the inventory
     row ("superseded by the served rules").
   - **Uncertain** → ask the human before choosing a disposition.

5. Inventory the `<!-- givn:begin:givn-project -->` …
   `<!-- givn:end:givn-project -->` block in AGENTS.md in the pre-upgrade
   revision the same way: the block is machine-owned and was refreshed by the
   upgrade, so an edit inside it is a displacement. Project guidance that must
   stay in AGENTS.md belongs outside the managed block.

6. Record the inventory in this change's `design.md` as one row per displaced
   statement:

   ```markdown
   | Old file | Displaced statement | Classification | Target instruction id |
   |---|---|---|---|
   | .opencode/skills/givn-propose/SKILL.md | <text> | project rule | proposal |
   ```

   Record the checked file list and a "none found" line when there are no
   displacements.

## Phase 2 — Record the extensions

1. Add or extend the `extensions:` map in `givn/config.yaml` (uncommented,
   project-owned; `givn upgrade` never touches user-owned lines):

   ```yaml
   extensions:
     proposal:
       - <the project rule>
     specs:
       - <the project rule>
   ```

2. Respect the recorded semantics: an id maps to a list of non-empty rule
   strings; an empty list is valid and adds nothing; list a rule under several
   ids to extend each of them; a repeated instruction key keeps the last
   entry; an unknown id or a malformed entry warns and is skipped at request
   time, and the built-in rules are served alone.

3. Verify each extended instruction:
   `givn instructions <id> --change <this-change-id>`. The served content
   must end with the marked `## Project extension` section carrying the rules
   in config order, and the built-in rules above it must stay unchanged. The
   section states that extensions add to the built-in rules and never
   override, suppress, or reorder them; a rule that contradicts a built-in
   rule stays visible to the project to resolve.

4. When the instruction has a project-owned replacement
   (`givn/artifacts/<id>/instructions.md`), the replacement is the base the
   extension extends — do not edit the replacement file.

## Phase 3 — Verify and complete

Run these exact checks and record their output in `tasks.md`:

1. All 15 installed instruction triggers in each configured target carry only
   direction. For every target, assert `grep -c '^## Project extension'` is 0
   and that each `skills/givn-*/SKILL.md` and command file matches the
   trigger/direction shape:
   `find <target>/skills -path '*/givn-*/SKILL.md' -o -path '<target>/commands/*.md'`
   then grep for the direction sentence. The expected result for the four
   targets is 15 skills and 15 commands for `opencode` and `omp`, 15 skills and
   0 commands for `agents` and `claude`, all trigger-only.
2. `givn skills sync` followed by `diff -r` against a checksum manifest taken
   before the sync: every generated skill, command, plugin, hook,
   `.claude/settings.local.json`, and `.githooks/commit-msg` is byte-identical.
   No extension text appears in any generated file.
3. `givn instructions <id> --change migrate-0-8-4-to-0-9-1` for each instruction
   id in the Phase 1 mapping returns exit 0 and no `## Project extension`
   section (`grep -c '^## Project extension'` = 0).
4. `givn lint --change migrate-0-8-4-to-0-9-1` exits 0 or 2.
5. `./run-tests.sh` (under `nix-shell -p rustc cargo git-cliff`) passes; the
   last run recorded 103 scenarios / 390 steps, all passing.
6. Derive this change's tasks from the inventory rows, run design-review
   before tasks, and complete the normal review and archive gates.

## Validation and completion

1. Every displaced statement has a disposition: an extension entry, a
   recorded "dropped as stale" reason, or a named follow-up change; the
   inventory records the checked file list (or the "none found" finding).
2. Each extended instruction serves its rules under the marked section and
   the built-in rules stay unchanged.
3. The installed files carry only trigger and direction.
4. The maintenance change archives through the normal gates; the migration is
   complete with that archive.


## Completion boundary

This generated design does not contain `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or implementation evidence. The project LLM must derive tasks and complete the normal Givn gates after the ordered bundle phases are understood, including any follow-up changes a bundle names.

## Phase 1 inventory — displaced project rules

### Checked file list

The pre-upgrade revision `76f1602771c6bf7c4430ac691657cff1e0a5b86c^` contains no tracked generated rule files: the project uses the default `rules: local-only` policy, so `git ls-tree` finds no `skills/givn-*/SKILL.md` or `commands/(givn|grill)-*.md`. `git log --all -- .opencode .agents .claude .omp` is empty, confirming no generated rule was ever tracked.

Two evidence classes were used, and they are not equivalent:

**Content recovered and compared.** The full pre-upgrade bodies of `givn-propose`, `givn-spec`, `givn-design`, `givn-design-review`, `givn-tasks`, `givn-review`, `givn-archive`, and `givn-domain-modeling` were recovered from the retained omp session transcripts under `~/.omp/agent/sessions/-dev-PRIVATE-duty2ical/`. They match the generated assets of the baseline version the project synced (givn `v0.8.5`, present at `/home/dan/dev/PRIVATE/givn`, tag `v0.8.5`) byte-for-byte, apart from transcript framing. `givn-implement` was compared through the transcript's grep evidence against the same baseline. `givn-ideation` was compared through the edit event recorded below and the baseline asset.

**Edit events scanned.** For `givn-explore`, `givn-steps`, `givn-reverse-engineer`, `givn-characterize`, and `givn-dev-principles`, and for every command file, the pre-upgrade body was not captured in any retained transcript. Their generated paths were scanned for writes: no `edit`, `write`, or redirecting `bash` tool call under a generated path appears in any omp transcript, and `opencode.db` contains no write/edit/patch/shell write under `.opencode/skills` or `.opencode/commands`. The absence of a write event is the evidence; it is weaker than a recovered body.

Checked generated paths across the four configured targets (`opencode`, `agents`, `claude`, `omp`):

- Skills: the 15 `skills/givn-*/SKILL.md` files in each target.
- Commands: `.opencode/commands/givn-*.md`, `.opencode/commands/grill-*.md`, `.omp/commands/givn-*.md`, `.omp/commands/grill-*.md`.
- Attribution plumbing: `.opencode/plugins/givn-attribution.js`, `.omp/hooks/pre/givn-attribution.js`, `.claude/hooks/givn-attribution.sh`, `.claude/settings.local.json`, `.githooks/commit-msg`.
- Managed `AGENTS.md` block: `git diff 76f1602771c6bf7c4430ac691657cff1e0a5b86c^ -- AGENTS.md` is empty; the `givn-project` block was not hand-edited.

**Evidence boundary.** A manual edit to an untracked generated file that never appeared in a retained agent session leaves no recoverable trace after the managed rewrite. The operator confirmed the ideation workaround is the only change they can identify and could not rule out an unknown manual edit; the migration records that boundary rather than inventing a rule. The ideation workaround itself is the one displacement with direct evidence.

### Displacements

| Old file | Displaced statement | Classification | Target instruction id |
|---|---|---|---|
| `.opencode/skills/givn-ideation/SKILL.md`, `.agents/skills/givn-ideation/SKILL.md`, `.claude/skills/givn-ideation/SKILL.md`, `.omp/skills/givn-ideation/SKILL.md` | The template's `## Diagram` heading was replaced with `**The actor-goal diagram.**` to stop the validator treating the template heading as a document-schema section. | Dropped as stale: a temporary workaround for a misdiagnosed schema error. The served 0.9.1 `ideation` instruction carries `## Diagram` in the use-case template, and the project's own permanent use cases (`givn/specs/export-duty-plan-calendar/usecase.md`, `givn/specs/publish-public-release/usecase.md`, the ideation contract, and the archived change) use a `## Diagram` section. The workaround was superseded before the upgrade. | `ideation` (no extension entry) |

The remaining recovered files match the `v0.8.5` baseline; the differences seen against `v0.8.4` (`givn-spec`'s `docs/use-case-model.md` phrase, `givn-domain-modeling`'s naming guidance) are built-in updates that the served 0.9.1 instructions already carry, not project rules.

### Disposition

No project rule survives into `givn/config.yaml` `extensions:`: the single displaced statement is stale and is dropped with the reason above. The next phase therefore verifies the served rules and the installed trigger files, and the migration adds no extension entry.
