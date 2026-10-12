# 8. Cross-cutting Concepts

## Complete-or-explicit-failure

Every conversion either produces a complete calendar or fails explicitly. The `DutyPlanCalendar` aggregate gates publication: no side effect occurs until every required activity and field resolves, and a required-value contradiction or an unknown required code rejects the conversion. A cron-like "best effort" partial calendar is never produced, because a calendar that looks complete but omits a leg is worse than a visible failure.

## No-clobber publication

New calendars and diagnostics are allocated with create-new semantics and an enumerated suffix; an occupied or concurrently claimed path is never replaced, and allocation retries the next unused name. This is enforced in the publication and diagnostic adapters, not in the domain, so the domain stays pure. The rule protects both the user's earlier outputs and a competing process.

## Explicit failure reporting

A failure carries a primary reason and is reported independently of diagnostic persistence. The diagnostic path is reported only after a successful write; when no new report can be saved, the message states that and never presents an older report as current. Exit status is zero only for a real publication.

## Source-snapshot-scoped identity and determinism

Component identity is derived from the source snapshot and the represented activity, so converting the identical input twice is deterministic (same identifiers) while a revised input yields separate identities. Output filename counters and human titles are never identity. DTSTAMP prefers the source's revision footer and falls back to the actual construction timestamp; revision metadata never supplies an activity date.

## Privacy

Real duty plans are read for analysis but never committed. Fixtures, golden calendars, and public examples are generated or anonymized, and privacy checks consider identifying combinations (dates, itineraries, trip identifiers) rather than names alone. No real plan or identifying output is sent to an external service.

## Classification and timing as pure rules

Source-code classification (all-day vs timed vs travel vs layover vs unknown) and UTC endpoint date resolution are pure functions over source-derived data. Keeping them free of I/O lets return-driven edge cases (adjacent-day qualifiers, the narrow overnight rollover, zero-length intervals) be specified and tested without a real PDF, and prevents the extraction dependency from leaking into the domain.
