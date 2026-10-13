# History rewrite: dropping the real duty plan's conversion before the first push

The operator decided on 2026-10-13, answering the design review's first question, to
rewrite the local history before the repository is published, rather than publish a
history-free repository, publish the exposure, or keep the repository private.

## What the leak was

The first draft's committed fixture `apps/cli/tests/fixtures/expected/2026-06.ics` (blob
`ee5886024b06d063579bbf4ec8c18bad66a7bd08`) was the conversion of the operator's own
private June-2026 duty plan, not the synthetic fixture its surrounding brief claimed.
The design-review grilling converted `private/MLP-2026-06.pdf` with this project's own
binary and diffed the result against that blob: 93 of 94 lines of designators, routes,
UTC departure and arrival times, all-day entries, layovers and EM duties matched
exactly; only `DTSTAMP`, the snapshot fingerprint and one newer `LOCATION:FRA` differed.
The file existed in five revisions (`af93202` … `1409de4`) and was removed from the tree
by the reader rework (`09a4297`), which left it reachable from the published tip and
therefore published by a plain `git push`.

## What was done

```
git filter-repo --path apps/cli/tests/fixtures/expected/2026-06.ics --invert-paths --force
```

Tree contents are unchanged: the tree of the rewritten tip equals the tree of the
pre-rewrite tip (`686ec2b560d37a1d939a2b1a2eb148a7db2044b3`), the commit count stays at
98, and no other path was touched.

## Verification

- The private-data guard (`scripts/pii-guard.sh`) failed before the rewrite, naming the
  file in five revisions; after the rewrite it reports `pii-guard: clean` over the tree
  and over every revision reachable from the published tip.
- The leaked blob is pruned from the local object store (`git cat-file -e
  ee5886024b06d063579bbf4ec8c18bad66a7bd08` fails).
- The file is absent from every revision (`git grep 'DTSTART:2026'` across all 98
  revisions returns nothing).

## Revision mapping for the archived change's citations

The archived change (`givn/archive/export-released-roster-calendar`) cited 26 revision
ids. Twenty-three of them still resolve unchanged, because they lie outside the range in
which the file existed. These three changed, and are the archived citations that need the
mapping to resolve:

| Archived citation | Rewritten id | Subject (unchanged) |
|---|---|---|
| `af93202` | `e0d890476` | `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF…` |
| `26c9002` | `0a8222d60` | `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF…` |
| `09a4297` | `fa387fce7` | `fix(export-duty-plan-calendar): read the producer's own export layout` |

`af93202`/`26c9002` are also the ids this change's own guard-repair revision cites when
it records the leak; they resolve through this table. Empty-body `feat` subjects are
indistinguishable by title, which is why the mapping is by position in the commit map
rather than by subject.

The full 98-row mapping is in the local, uncommitted `.git/filter-repo/commit-map`,
written by the rewrite tool; it is not published, because it would republish the
rewritten ids of every revision at once for no reader's benefit.
