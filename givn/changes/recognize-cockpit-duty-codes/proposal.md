# Proposal: recognize-cockpit-duty-codes

## Release note

> Recognize the cockpit-crew codes U1, OFF_2, MED, WBT_*, EMCRM and EBTF3* by their confirmed meanings, so those rows publish without the unknown-duty warning.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy lens), `project-maintainer--release-owner` (evidence lens)

## Review Context

- Review kind: self-review
- Reason: the change adds confirmed code meanings to the existing classifier; it introduces no contract, storage, external boundary or deployment choice. The affected use case and glossary already own the vocabulary.

## Problem / Opportunity

The cockpit-crew plans print six code families the converter does not recognize. They currently publish as `unknown duty: <code>` with a conversion warning, even though the operator confirmed what they mean:

| Printed code | Confirmed meaning | Present behaviour |
|---|---|---|
| `U1` | Urlaub/vacation | all-day fallback, `unknown duty: U1`, warning |
| `OFF_2` | Freier Tag/day off | all-day fallback, `unknown duty: OFF_2`, warning |
| `MED` | Medical appointment (Arzttermin) | timed fallback at the printed station, `unknown duty: MED`, warning |
| `WBT_*` | Web-based training at home | timed fallback, `unknown duty: WBT_*`, warning |
| `EMCRM` | Emergency + Crew Resource Management training at the company | timed fallback, `unknown duty: EMCRM`, warning |
| `EBTF3*` | Evidence-based Training in the simulator | timed fallback, `unknown duty: EBTF3*`, warning |

A crew member importing the calendar sees the fallback label and the warning report on every cockpit month even though the rows are understood. `AMV`, `REP`, `SBAO` and `S5` remain unconfirmed and keep the fallback.

## Proposed Solution

- `U1` becomes an all-day activity whose title is the printed code, exactly like `U`; printed clocks never change it.
- `OFF_2` becomes an all-day activity whose title is the printed code, exactly like `OFF`; printed clocks never change it.
- `MED`, `WBT_*`, `EMCRM` and `EBTF3*` become timed non-flight duties using the row's printed UTC start/end, with the printed station when supplied and the printed code as the title. They no longer emit a conversion warning.
- `WBT_*` is the family whose printed code starts `WBT_`; `EBTF3*` is the family whose printed code starts `EBTF3`. The suffix is preserved in the title and is not interpreted.
- `AMV`, `REP`, `SBAO`, `S5` and every other unconfirmed code keep the existing `unknown duty: <code>` fallback and warning.
- The calendar writer, description fields, UIDs, `DTSTAMP`, reports and public support statement are unchanged.

## Capability Routing

`givn spec route --change recognize-cockpit-duty-codes` reports no signal (the top candidates tie at 13.00). The decision is recorded directly.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| export-duty-plan-calendar | no signal | EXTEND export-duty-plan-calendar | The actor goal, reading, timing, calendar and publication are unchanged; only the set of recognized codes and their confirmed kinds grows. No new actor action or capability is created. |

## Out of Scope

- `AMV`, `REP`, `SBAO`, `S5` and any other code without a confirmed meaning. They keep the fallback.
- A new location rule for `WBT_*` or `EMCRM`: the printed station stays the source's station, and no room, home address or company site is invented.
- A new description field, title translation or acronym expansion. The title stays the printed code.
- A public support-statement change; the statement already names a Lufthansa crew duty plan.
- A release.

## Open Questions

### D1 — How are the newly recognized timed codes represented?

- Question: Should `MED`, `WBT_*`, `EMCRM` and `EBTF3*` use the existing timed non-flight representation, or a new medical/training activity kind?
- Options:
  1. **Use the existing timed non-flight representation** — printed UTC interval, printed station, code as title, no description. Gain: one kind for all non-flight duty, no new event shape, and the same calendar contract as `TK`, `DTUSA` and `EM`. Cost: the calendar does not state “medical” or “training” as a word. In practice, `MED` is a timed event at MUC titled `MED`, and `EMCRM` is one at MUC titled `EMCRM`.
  2. **Add a new activity kind with a translated title** — `Medical appointment` or `Training`. Gain: a reader sees the meaning without knowing the code. Cost: a second title source, an acronym expansion the operator did not ask to publish, and a wider change to the calendar and tests. In practice, the same row would be titled `Medical appointment` rather than `MED`.
- Disposition: answered — option 1, the existing representation, because the operator confirmed meanings for recognition, not a translated title; the code remains the faithful source value.

### D2 — Do `WBT_*` and `EBTF3*` recognize the prefix or only the observed codes?

- Question: Should `WBT_*` and `EBTF3*` match the prefix, or only the observed `WBT_EM`, `WBT_GR`, `EBTF3A` and `EBTF3B`?
- Options:
  1. **Match the confirmed prefix** — any code starting `WBT_` or `EBTF3` is the family; the suffix is preserved and not interpreted. Gain: the operator's `*` is honored, and a new subject or simulator session does not warn. Cost: a code that starts the prefix but means something else would be misrecognized; the prefix is the evidence. In practice, `WBT_XY` and `EBTF3C` are recognized.
  2. **Match only the observed codes** — exact `WBT_EM`, `WBT_GR`, `EBTF3A`, `EBTF3B`. Gain: no family inference. Cost: ignored operator instruction; the next training row warns again. In practice, `WBT_XY` would keep the fallback.
- Disposition: answered — option 1, the confirmed prefix, because the operator wrote `WBT_*` and `EBTF3*` explicitly.
