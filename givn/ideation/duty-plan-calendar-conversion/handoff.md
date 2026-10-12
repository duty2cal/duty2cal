# Duty-plan calendar conversion — confirmed Handoff record

Status: published through the user's explicit confirmation on 2026-10-12. The approved preview below is retained for traceability; actual outputs/verification are recorded at the end. No normal change or code was created.

## Source and approved outcome

- Source topic: Duty-plan calendar conversion (`duty-plan-calendar-conversion`).
- Approved contract: Export a duty-plan calendar (`export-duty-plan-calendar`), at `use-cases/export-duty-plan-calendar.md`.
- Approval: the user adopted the complete contract on 2026-10-11 through the complete-contract decision (Q31); the full alternatives remain in `questions.md`.
- Privacy: private PDFs may be read but never committed. The public record contains only generic structural observations and generated examples, not private filenames/itineraries or identifying source values.

## Use-case-to-permanent mapping

| Approved Actor goal | Level and primary Actor | Future permanent leaf | Owning boundary |
|---|---|---|---|
| Export a duty-plan calendar (`export-duty-plan-calendar`): obtain one complete importable representation of one supported supplied duty plan. | `!`; Crew member. | `givn/specs/export-duty-plan-calendar/usecase.md` | Duty-plan calendar, the confirmed whole-plan completeness boundary. |

This maps the same stable ID to its own leaf, not a capability inside another actor goal. The permanent path is verified against the generated local spec skill. It is **not written during this Handoff**; normal proposal/spec/archive work creates/maintains it.

No separate use case or shipping seed is minted for PDF extraction, clock assembly, calendar writing, output naming, failure reports, or release packaging. They contribute to the same complete actor outcome; the maintainer's release approval remains an external evidence/review interest, not a converter product command.

## Proposed deliverable cut and suggested start

| Proposed seed | End-to-end deliverable guarantee | Suggested start / dependencies |
|---|---|---|
| Export complete Released Roster calendars (`export-released-roster-calendar`) | A standalone Windows converter for the selected Released Roster table family produces a complete RFC 5545 calendar at an unused name, or exposes failure and attempts an unused numbered diagnostic, preserving source/prior outputs and applying the approved activity/date/metadata rules. Source, build, licensing, privacy, and byte-format evidence are gates before claiming release readiness. | First and only proposed seed; no prerequisite change. |

- Only after Handoff confirmation, write `changes/export-released-roster-calendar.md` using the skill's complete seed schema, with `Status: ready` as the first line after its title.
- Use-case IDs: `export-duty-plan-calendar` only; source topic remains `duty-plan-calendar-conversion`.
- The seed carries each accepted decision's context, genuine alternatives, examples, gains/costs, chosen outcome and rejected alternatives—not an outcome-only summary. Relevant source records include topic/frame/privacy, Persona approvals, compatibility, whole-plan boundary, all activity/timing choices, output preservation, all-failure reports, source-snapshot identity, source/fallback timestamp, trip context, adjacent dates, and complete-contract adoption.
- The seed retains the full standalone parked questions below and the empirical validation obligations. `ready` means ready input to the normal proposal flow, not code or a validated release.
- It is one whole deliverable, not an intermediate parser or writer whose shipment would violate the complete-export guarantee or inherited single-executable constraints.
- Handoff never starts this change. The user separately invokes `/givn-propose` with this seed when ready; no `givn new`, proposal scaffold, spec delta, tasks, code, fixture, or golden file is created by confirmation here.

## Persona reuse inventory and proposed promotions

The project collection contains no Persona files. No sibling topic exists with a reusable Persona. This topic has two already-confirmed segments; neither new-segment claim relies on cosmetic biography differences.

| Persona | Distinguishing authority/evaluation and stable core | Proposed project file |
|---|---|---|
| Crew member exporting their own duty plan (`crew-member--personal-calendar-owner`) | Supplies their own plan and judges calendar fidelity/usability; cannot change the official schedule or certify release/API/RFC compliance. Interests: correct activities/dates, clear failure, private information kept out of public artifacts. Own vocabulary: duty plan, flight leg, report time, deadhead, layover, printed codes. | `givn/personas/crew-member--personal-calendar-owner.md` |
| Maintainer responsible for a public release (`project-maintainer--release-owner`) | Accepts/rejects a public release against agreed constraints and evidence; cannot relax those constraints or invent code meanings. Evaluates supported-source tests, calendar validation, licenses, Windows distribution, and private-data exclusion. Own vocabulary: supported export, generated fixture, golden calendar, validation evidence, license, release artifact. | `givn/personas/project-maintainer--release-owner.md` |

Proposed decision for **each** file: promote its existing approved core and running topic/review records. Stable Role, Authority, Evaluation, Vocabulary, Interests, Goals, and Success criterion remain unchanged. The same owner Persona also covers the identified plan owner's privacy interest; no duplicate data-subject biography is created.

Both are advisory review lenses, not Gherkin Actors, autonomous authorities, or release approvals. Their topic files remain historical. The complete source records shown in the approved topic files are retained, including dated stances and all three phase-review reactions; they are not flattened into a new outcome-only biography.

### Proposed crew-member copy/merge diff

The destination is absent, so this is a non-destructive copy, not a move or overwrite. The exact source is `personas/crew-member--personal-calendar-owner.md`; all content not shown in the transformation below is copied unchanged. Artifact references inside its dated running record remain scoped to this origin topic.

```diff
diff --git a/givn/ideation/duty-plan-calendar-conversion/personas/crew-member--personal-calendar-owner.md b/givn/personas/crew-member--personal-calendar-owner.md
copy from givn/ideation/duty-plan-calendar-conversion/personas/crew-member--personal-calendar-owner.md
copy to givn/personas/crew-member--personal-calendar-owner.md
@@
-New: crew-member--personal-calendar-owner — evaluates fidelity to their own duty plan, unlike a maintainer who evaluates release conformance.
-
 # Persona: crew-member--personal-calendar-owner
@@
-Status: confirmed by the user on 2026-10-10; topic-local only, not promoted.
+Status: confirmed by the user on 2026-10-10; promoted to the project collection by confirmed Handoff on 2026-10-11.
+
+Origin topic: Duty-plan calendar conversion (duty-plan-calendar-conversion). Relative artifact references in the running record refer to givn/ideation/duty-plan-calendar-conversion/.
@@ end of existing running record
+
+## Promotion record
+
+### 2026-10-11 — Confirmed ideation Handoff
+
+- The user confirmed project-level promotion through the Handoff decision (Q32).
+- The stable core and validated dated topic/review records were copied unchanged from the origin topic; the topic file remains historical.
+- Promotion grants a reusable advisory lens only, not Actor identity, code authority, or automatic approval.
```

### Proposed maintainer copy/merge diff

The destination is absent. The exact source is `personas/project-maintainer--release-owner.md`; unchanged content and origin-scoped dated records are retained, and the source is not removed.

```diff
diff --git a/givn/ideation/duty-plan-calendar-conversion/personas/project-maintainer--release-owner.md b/givn/personas/project-maintainer--release-owner.md
copy from givn/ideation/duty-plan-calendar-conversion/personas/project-maintainer--release-owner.md
copy to givn/personas/project-maintainer--release-owner.md
@@
-New: project-maintainer--release-owner — authorizes a public release and evaluates release-conformance evidence, unlike the crew member who evaluates fidelity to their own duty plan.
-
 # Persona: project-maintainer--release-owner
@@
-Status: confirmed by the user on 2026-10-10; topic-local only, not promoted.
+Status: confirmed by the user on 2026-10-10; promoted to the project collection by confirmed Handoff on 2026-10-11.
+
+Origin topic: Duty-plan calendar conversion (duty-plan-calendar-conversion). Relative artifact references in the running record refer to givn/ideation/duty-plan-calendar-conversion/.
@@ end of existing running record
+
+## Promotion record
+
+### 2026-10-11 — Confirmed ideation Handoff
+
+- The user confirmed project-level promotion through the Handoff decision (Q32).
+- The stable core and validated dated topic/review records were copied unchanged from the origin topic; the topic file remains historical.
+- Promotion grants a reusable advisory release-evidence lens only, not a product Actor, constraint override, or release approval.
```

If either destination appears or differs before publication, re-inventory/read it and show a revised merged diff rather than overwrite unfamiliar work. A named promotion can be deferred independently when the user confirms the preview; no silent promotion is performed.

## Proposed durable vocabulary

Only currently `confirmed` terms are proposed for `docs/arc42/12-glossary.md`. Their full genus/contrast/concrete-use and pre-introduction collision records stay in `domain-terms.md`.

| Term to promote | Definition/meaning being promoted | Distinction preserved |
|---|---|---|
| Additional vacation day | FZ denotes an additional vacation-related day off; approved all-day timing ignores source clocks. | No verified age/quota/agreement enforcement or unconfirmed acronym expansion. |
| Consulate visa duty | DTUSA denotes non-flight consulate attendance to obtain a USA visa; printed UTC window. | Not booking a visa or inferring unknown DT suffixes. |
| Deadhead duty | Pos DH denotes positioning travel in uniform without usual operating work, still duty time; title marker retained. | Assignment on one travel event, not time off or a second event. |
| DT prefix | DT expands to Duty Time in the observed activity designation. | Activity-code text, not a numeric duty-duration metric or generic suffix classifier. |
| Duty-plan calendar | Complete activity composition from one supplied duty plan governs readiness to export. | The only aggregate; PDF, calendar file, reports, snapshots, and trip groups are records/projections. |
| Office or seminar duty | TK denotes required office/professional-development/team-information attendance; printed UTC window. | Not flight or vacation. |
| Reserve duty | RE is a bounded availability assignment with UTC start/end; notice time distinguishes it from SB. | No activation/reporting calculator or service. |
| Roster trip | Source grouping of assigned travel legs associated with one Trip ID. | Descriptive grouping, not an independent aggregate or a single leg. |
| Single-day standby | SBX is isolated/replacement standby with its own UTC window. | Single-day is not all-day; no old-flight time reuse or import cancellation logic. |
| Source revision timestamp | UTC snapshot metadata from the user-accepted Created footer for this family. | Not activity timing; construction-time fallback is available when source metadata is unusable. |
| Source snapshot | Fixed input PDF version scopes deterministic distinct calendar-component identities. | Output numbering/renaming does not define the source version; revised snapshots have separate identities. |
| Standby duty | SB is a start/end availability assignment for potential activation. | Not time off or an annual-statistic event; consecutive windows stay separate. |
| Vacation day | U means Urlaub/vacation; all-day with source times ignored. | U remains distinct from FREE/OFF/FZ, without invented entitlement rules. |

Proposed glossary sections preserve alphabetic term ordering and add the one declared aggregate, its complete-export invariant, other approved timing/output-preservation invariants, and the seven reviewed lifecycle events. Code symbols/enforcement implementations are explicitly not implemented, not fabricated.

- **Candidates not promoted:** Duty plan, Flight leg, Calendar export. They remain candidate introductions in topic state; Handoff does not silently convert them to confirmed glossary entries.
- **Rejected entries:** none. No rejected record is deleted; the uncertain FZ expansion/provenance is still reported as unverified background.
- The durable glossary currently has no real definitions. If that changes before publication, show a real merge rather than replacing new user vocabulary.

## Parked questions retained in the human record and seed

### Optional descriptive values disagree

The descriptive-fragment question (Q28) concerns a generated, unobserved robustness case, not an actual contradiction in the supplied corpus. Given otherwise compatible fragments with Report (LT) 07:30 versus 08:00, should conversion reject a conflicting optional field, or preserve both with fragment-origin labels?

- **Reject conflicting descriptions:** a generated disagreement yields an explicit diagnostic, not an arbitrary chosen report time. Gain: avoids apparently authoritative conflicting details. Cost: legitimate multi-valued annotations need another evidenced representation before support.
- **Preserve both with source roles:** the description distinguishes departure-fragment 07:30 LT from arrival-fragment 08:00 LT, neither used to compute UTC instants. Gain: retains evidence. Cost: longer descriptions and explicit provenance; required date/route/assignment contradictions still fail.

This policy is not selected by contract or Handoff approval. Ordinary complementary values combine and equal repeats coalesce as already approved. The seed must not claim unconditional support for the differing-optional-value case before a deliberate disposition.

### Source-derived publication rights, if derivatives are considered

The publication-rights question (Q5) asks what authoritative source-use terms permit publishing anonymized derivatives or adapted source material. Private access/anonymization is not by itself a license. The initial fully generated fixture route remains available and is preferred by the first draft; no real source PDF is committed.

This is a parked factual request, not license evidence fabricated by Handoff. It affects any future source-derived public material, not independent generated cases or permission to read the private examples.

### Formal FZ expansion and entitlement origin

The FZ-provenance question (Q13) asks for authoritative acronym/agreement evidence if a verified claim is ever needed. “Frei zusätzlich” is a suggested expansion; the described annual/age background is not an eligibility validator. This remains non-blocking for representing an already assigned FZ activity.

## Later validation and technical work, not solved by publication

- Validate expert-provided SB/SBX/RE rows, UTC columns, and station association against a later supplied duty plan.
- Validate month/year qualifiers and the selected unqualified travel case against real source variations; generated examples specify desired behavior but do not manufacture producer evidence.
- Validate source coverage/fragment/trip boundaries and dynamic table-header/body extraction; API availability or word count alone does not prove correct activity coverage.
- Preserve byte-level CRLF, text escaping, 75-octet UTF-8 folding, valid endpoints, globally scoped/distinct snapshot IDs, and source/fallback timestamp behavior in future tests.
- Reconcile the first draft's oracle/data mismatches, overlong lines, incorrect synthetic weekday, ineffective Blocktime-independence sample, missing standby coverage, and PII-regex timestamp false positive before treating that evidence as valid. The user draft remains unedited here.
- Verify the exact Rust dependency graph/licenses, no-native PDF dependencies, actual pinned API use, Windows GNU cross-build, single-executable runtime imports/distribution, and output no-clobber under races/failed writes.
- Retain real-source local validation outside version control. Do not upload actual private plans/calendars/reports to public validators or issue trackers.

## Proposed publication actions and exclusions

After explicit Handoff confirmation only:

1. Publish a start-to-finish human record to `docs/ideation/duty-plan-calendar-conversion.md`.
2. Publish the one ready seed under this topic's `changes/` using the complete required schema and retained question records.
3. Perform the specifically accepted Persona copies/record appends and confirmed-term glossary additions; leave topic files historical.
4. Mark this Handoff and its chosen/deferred promotions in `session.md` and `questions.md`.

Never as a side effect: create a normal givn change, write a permanent leaf/spec delta, add code/tests/fixtures/goldens, run a scenario commit, alter the exact pin, or publish private source values. The normal proposal flow remains the user's next separate action.

## Confirmed publication

- User confirmation: Handoff publication decision (Q32), closed 2026-10-12 in [the full decision log](questions.md#q32--publish-the-ideation-handoff-records).
- Actual promotion/publication date: 2026-10-12. The copy diffs above are the historical approved preview; their planned 2026-10-11 promotion metadata is replaced by this actual date in the published Persona records.
- Human record: [Duty-plan calendar conversion](../../../docs/ideation/duty-plan-calendar-conversion.md).
- Ready planning seed: [Export complete Released Roster calendars](changes/export-released-roster-calendar.md).
- Project Personas: [Crew member exporting their own duty plan](../../personas/crew-member--personal-calendar-owner.md) and [Maintainer responsible for a public release](../../personas/project-maintainer--release-owner.md).
- Durable vocabulary: [The project glossary](../../../docs/arc42/12-glossary.md).
- No normal change or permanent spec leaf is created. The approved parked questions and future validation obligations remain in the readable record and seed.
- Verification: ready seed schema/status and local links pass read-only checks; promoted stable cores/existing records match the topic originals exactly; the glossary contains thirteen confirmed introductions only. Private sources remain ignored/untracked and no actual private PDF filename was copied into public records.
