# Proposal: recognize-standby-shift-codes

## Release note

Standby shift assignments printed as SB_F_CF, SB_M_CF or SB_S_CF are recognized as standby availability windows, so a supported duty plan that contains them converts without warnings for those rows.

## Use-Case Context

- Use-case ID: export-duty-plan-calendar
- Ideation topic: duty-plan-calendar-conversion
- Seed path: none — this change originates from the operator's confirmed statement about the printed codes, not from an ideation change seed.
- Confirmed Personas: crew-member--personal-calendar-owner — accuracy/privacy lens: the calendar must show the crew member's actual availability windows and must not raise a finding about a row whose meaning is confirmed. The `contributor--web-reviewer` Persona is not affected: this change touches no browser surface — no page copy, no CSP, no no-network proof, no WASM bundle — so its lens has nothing to review here. The confirmed operator statement of 2026-10-27 is the source of the meaning; the maintainer Persona's support statement and release claim are unchanged; the support-evidence record's accepted-code sentence gains the three codes so the evidence stays truthful.

## Problem / Opportunity

The supported November 2026 operator plan contains six detail rows whose activity code is `SB_F_CF` (2026-11-15 and 2026-11-16, printed UTC 03:30-13:00), `SB_M_CF` (2026-11-07 and 2026-11-08, printed UTC 08:00-15:30) and `SB_S_CF` (2026-11-11 and 2026-11-12, printed UTC 14:00-22:00). Every one of them is currently an unexpected activity code, so the conversion represents each row from its printed evidence and reports it as a conversion warning: those six rows are the plan's entire warning report (`private/MLP-2026-11.parse-warnings.txt`).

The operator has now stated what those codes mean: `SB` is Standby, and the letter names the shift — `SB F CF` is Standby Früh (early), `SB M CF` is Standby Mittel (middle), `SB S CF` is Standby Spät (late). The printed windows agree with that reading. `CF` remains unknown to the operator and is not interpreted.

The cost of the current behavior is a warning report on a supported plan whose rows are in fact ordinary standby availability. A warning is the converter's admission that it represented a row by a documented fallback; issuing it for a confirmed standby window makes the plan look less trustworthy than it is and forces the crew member to read and dismiss six findings that are not findings. It also hides a genuine fallback, should one appear later in the same plan, among confirmed rows.

## Proposed Solution

A detail row whose printed activity code is exactly `SB_F_CF`, `SB_M_CF` or `SB_S_CF` is treated as a standby availability window, in the same way as `SB`:

- The calendar carries one timed activity titled by the printed code — `SB_F_CF`, `SB_M_CF` or `SB_S_CF`, never a German or English translation — located at the printed station, starting and ending at that row's own printed UTC times.
- Consecutive dates keep their own windows: two `SB_F_CF` days are two activities, and a `FREE` day between them stays separate.
- The conversion reports no warning for these rows, and the warning report and the browser page list no entry for them. A plan whose only previously warned rows are these six is published as a plan with no warnings and no warning report.
- Any other activity code, including another string of the shape `SB_<letter>_CF`, keeps today's behavior: it is represented from its printed evidence and reported as a conversion warning. Recognizing the three codes is not a prefix or family rule, and `CF` gains no meaning.
- A row carrying one of the three codes that cannot supply a resolvable interval — one printed endpoint, or an interval that is not positive — is a conversion error exactly as for other recognized timed codes: no calendar is published and the failure report lists the row under its errors, with the existing wording.
- Nothing else changes: every other code keeps its kind, the printed title convention is unchanged, and no new finding kind, report section or diagnostic wording is introduced.

## Capability Routing

`givn spec route --change recognize-standby-shift-codes` reported no signal (the top candidates tie), so this table records the decision without a recommendation column.

| Proposed capability | Decision | Rationale |
|---|---|---|
| export-duty-plan-calendar | `EXTEND export-duty-plan-calendar` | The change widens the activity codes the existing export recognizes and removes a warning from it. The Crew member's goal, the conversion flow, the calendar, the warning report and the failure report are the ones `export-duty-plan-calendar` already owns; no new actor goal is introduced, so a new capability would be a capability created for a data change. |

## Out of Scope

- Translating calendar titles into German or English (`Standby Früh` and similar): the printed code stays the title, as for every other availability code.
- Interpreting `CF`, or asserting any expansion of it.
- A prefix or family rule for `SB_<letter>_CF` or any other `SB_`-shaped code; an unseen variant keeps the warning.
- Any other currently unexpected code, including other shift-like names.
- The meaning, window, or title of `SB`, `RE`, `SBX`, `TK`, `DTUSA`, `EM`, the all-day codes and travel.
- Activation, call-to-report notice, check-in, or any scheduling/legality judgement about a standby shift.
- New diagnostic wording, a new severity, or a new report section.
- Browser-page copy, the page's ready/warning states, the CSP, the WASM bundle and the no-network proof: the page already lists whatever warnings the conversion produces, and this change only removes warnings from a subset of rows.

## Open Questions

### D1 — Should the three printed codes be recognized standby availability windows?

- Question: Should a row whose printed activity code is `SB_F_CF`, `SB_M_CF` or `SB_S_CF` be represented as standby availability, or keep today's unexpected-code fallback with its warning?
- Options:
  1. **Status quo — keep warning forever** — the codes stay unexpected and every such row is represented from its printed endpoints with a warning. Gain: no recognition decision is taken on the operator's word, and the fallback stays visible. Cost: a supported plan carries six warnings for rows whose meaning the operator has confirmed, the warning report loses its signal, and the crew member must dismiss findings that are not findings. In practice: November 2026 keeps `private/MLP-2026-11.parse-warnings.txt` with its six entries.
  2. **Recognize the three codes as standby availability** — each is a timed availability window from its own printed UTC start/end and station, titled by the printed code, with no warning. Gain: the plan converts with no warnings for confirmed rows and a genuine fallback stands out again. Cost: the converter now asserts a semantics the operator supplied rather than one derived from the plan itself, and the three codes must be maintained as literal recognition data. In practice: `SB_F_CF` on 2026-11-15 becomes an activity 03:30-13:00 UTC at the printed station and the warning report is not written at all.
- Disposition: answered — the operator's confirmed statement of 2026-10-27 ("The SB is Standby. The F, M, S are German for: SB F CF = Standby Früh; SB M CF = Standby Mittel; SB S CF = Standby Spät"), read together with the existing contract that `SB` is standby with its own UTC window. Option 2 chosen.

### D2 — How far does recognition reach: exact strings, a prefix family, or the `CF` letters?

- Question: Should recognition be limited to the three exact printed strings, extended to every `SB_<letter>_CF` string, or should `CF` (or the trailing `_CF` suffix) be interpreted as a rule?
- Options:
  1. **Exact three strings** — only `SB_F_CF`, `SB_M_CF`, `SB_S_CF` are recognized. Gain: the converter claims only what the operator confirmed; an unseen code still warns, so a new meaning is never invented. Cost: a fourth shift variant in a later source needs its own explicit recognition. In practice: `SB_X_CF` prints a timed activity titled `SB_X_CF` and one conversion warning.
  2. **Prefix/family rule for `SB_<letter>_CF`** — any code of that shape is standby availability. Gain: future shift variants need no code change. Cost: the shape is turned into a meaning the operator did not state; a wrongly shaped code would be silently accepted as availability with no warning. In practice: `SB_X_CF`, a code nobody has explained, becomes a warning-free availability window.
  3. **Interpret `CF`** — expand the suffix or treat it as a modifier of the window. Gain: the whole family could be derived. Cost: the operator explicitly does not know what `CF` means; any expansion would be invented. In practice: a report or title would claim an expansion the source does not support.
- Disposition: answered — operator statement of 2026-10-27, which names the three codes and says "The CF is unknown to me". Option 1 chosen; options 2 and 3 are rejected because they assert unstated semantics.

### D3 — What do these rows look like in the calendar?

- Question: Should the calendar title these activities with the printed code, with the operator's German shift name, or with an English translation?
- Options:
  1. **Printed code as the title** — `SB_F_CF`, `SB_M_CF`, `SB_S_CF`, as every other availability code is titled. Gain: the calendar matches the plan the crew member holds, one title convention covers all codes, and no translation can drift from the source. Cost: a reader who does not know the shift letters gets no help from the calendar. In practice: the event reads `SB_M_CF` at the printed station, 08:00-15:30 UTC.
  2. **German shift name** — `Standby Früh`/`Standby Mittel`/`Standby Spät`. Gain: the shift is readable without the decoder. Cost: introduces a second title convention and a translation layer the rest of the calendar does not have; the printed evidence disappears from the event. In practice: an event titled `Standby Früh` where the plan prints `SB_F_CF`.
  3. **English translation** — `Standby Early`/`Standby Middle`/`Standby Late`. Gain: readable for a non-German reader. Cost: the same second convention, plus a translation the operator did not ask for. In practice: an event titled `Standby Late` for a `SB_S_CF` row.
- Disposition: answered — existing contract rule that a code-named activity is titled by the printed code, and the operator's statement names the shifts but asks for no retitling. Option 1 chosen.

### D4 — What happens to findings for these rows, and to an unusable one?

- Question: Once the three codes are recognized, should their rows produce no finding at all, keep a warning, or gain a new informational finding — and how should a recognized shift row that cannot produce an interval be reported?
- Options:
  1. **No finding for a representable row; the existing conversion error for an unusable one** — a valid shift row is an ordinary recognized activity; a row with one endpoint or a non-positive interval reaches the same conversion-error path as any recognized timed code. Gain: one severity rule holds for every recognized code, the six warnings disappear, and nothing new is emitted. Cost: a crew member can no longer tell from the output that this code was only recently recognized. In practice: six representable shift rows produce no warning; a `SB_F_CF` row with a printed start and no end fails the conversion with the existing row error.
  2. **Keep a warning on recognized shift rows** — recognition changes the kind but still warns. Gain: the change of behavior stays visible per row. Cost: the warning stops meaning "this row was represented by a fallback", which is the severity rule the converter states; a supported plan still reads as partly uncertain. In practice: the November plan keeps a six-entry warning report.
  3. **Add a separate informational finding** — a third severity for newly recognized codes. Gain: the operator could see which codes recently became known. Cost: a new severity, report section and page state for a data change, with no reader action attached. In practice: a plan would need an "informational" list in the warning report and on the page.
- Disposition: answered — the contract's severity rule ("a conversion warning means the row was represented by a documented fallback") and the operator's confirmed meaning. Option 1 chosen; the conversion-error path for an unusable row is the existing path, with no new wording.

No unresolved question remains; specs may proceed.
