# Release executable size — question log

## Q1 — What problem does a smaller executable solve?

- Status: closed 2026-10-18 (user)
- Blocks: the Frame problem statement, the stakeholder list, and every later
  phase. It decides whether this topic produces a build-profile change, a
  release gate, a dependency audit, or no change at all.
- Context: The draft lists compiler and profile levers, but no outcome it
  serves. Three facts narrow the fork. First, the published Windows archives
  moved +250 B (v0.1.0 to v0.2.0, a fix release) and −306 B (v0.2.0 to v0.3.0,
  the dependency updates), net −56 B; the executables inside moved +512 B and
  −1,024 B. Second, the archive is not the executable: inside the v0.3.0
  archive the `duty2cal.exe` is 2,097,152 B, and gzip brings it to roughly
  half. Third, the committed profile already carries `opt-level = "z"`,
  `lto = true`, `strip = true` and `codegen-units = 1`; the only unspent
  stable profile setting is `panic = "abort"`, and `opt-level = "s"` versus
  `"z"` and feature selection are experiments with unmeasured gain.
- Question: Which problem should this topic solve — an executable that can grow
  unnoticed, an executable that is larger than it needs to be, or a future
  browser payload built from the same dependency graph?
- Options:
  1. **Growth is invisible.** Define a size ceiling and check the built
     executable against it before a release is staged.
     In practice: a dependency bump adds 60 KB to `duty2cal.exe`; the release
     preparation refuses and names the executable's size and the ceiling,
     instead of publishing it silently.
     Gain: silent creep is caught once, at the release that would carry it.
     Cost: the ceiling needs a number nobody has measured the headroom for; a
     cost: a tight number blocks a legitimate dependency update, a loose number catches
     nothing. Between adjacent releases the archive moved +250 B then −306 B,
     so the gate protects against a risk not yet observed.
  2. **The executable is too big.** Spend the remaining stable levers and
     measure the attribution, then keep whatever is smaller.
     In practice: `panic = "abort"`, `opt-level = "s"` versus `"z"`, and a
     feature audit of `duty2cal-pdf` land; the next release's executable is
     smaller by whatever the measurement shows, possibly only a few percent.
     Gain: an actual reduction, if one exists, and a measured answer to "what
     is in these two megabytes".
     Cost: `panic = "abort"` removes unwinding from the release build, so a
     panic ends the process without a backtrace; the audit can reveal a
     dependency feature that cannot be dropped without losing PDF support.
     The draft itself estimates 5–20%, unmeasured.
  3. **The same graph must become a browser payload.** Treat the dependency
     graph, not the Rust profile, as the object: the WASM facade later ships
     the same crates, where bytes are load time.
     In practice: the work targets `pdfplumber`'s transitive weight — feature
     selection, or eventually a different extraction backend — rather than
     compiler flags.
     Cost: the largest candidate, a backend replacement, is a product decision
     with its own ADR; profile tuning is nearly irrelevant to it.
     Gain: effort lands where the draft says bytes are perceptible.
  4. **Nothing; the draft already answered it.** Record the measurement and the
     finding that ~2 MiB (1 MiB download) is the expected stable minimum, and
     close the topic without a change.
     In practice: no repository change; the topic state and the readable
     record state why no change follows.
     Gain: no speculative build change and no gate to maintain.
     Cost: the "1 MiB is fine" question stays unmeasured at the crate level;
     a future growth would be noticed only by chance.
- Resolution: **Reduce now, measured** because the crate-level attribution is
  the missing fact the draft itself makes the precondition for every lever, and
  the stable levers are cheap enough to apply once measured. Rejected: **Growth
  gate** because growth is not observed — 250 bytes across two releases — and a
  ceiling would be set without measured headroom; **WASM payload** because the
  facade is a later phase and its strongest lever is a product decision with
  its own ADR, not a size-profile change; **Close, no change** because the
  draft's "expected minimum" is itself unmeasured at crate level, which is the
  gap this topic exists to close.

## Q2 — Confirm the Frame

- Status: closed 2026-10-18 (user)
- Blocks: Event Storming and every later phase.
- Context: The Frame records the problem (an unattributed 2,097,152-byte
  executable and levers chosen on estimates), the vision (measured attribution,
  every lever applied or rejected with a reason, diagnostics preserved), the
  scope (measure the cross-built executable, apply the stable levers the
  measurement justifies), the exclusions (nightly levers, UPX, dynamic CRT,
  backend replacement, WASM, a hard ceiling), the inherited constraints, and
  three stakeholder types with interests.
- Question: Is this Frame adopted as recorded?
- Options:
  1. **Accept** — the Frame is recorded as written and Event Storming starts.
     In practice: the next artifact is the event chain for measuring and
     applying the levers; no repository file is edited, no build configuration
     changes, no change is started.
     Gain: the topic proceeds on a stated problem, scope and stakeholders.
     Cost: a wrong stakeholder list or scope now propagates into the event
     chain; correcting it later reopens later artifacts.
  2. **Revise** — name the section to rewrite (Problem, Vision, Scope,
     exclusions, Constraints, or Stakeholders); the Frame stays unconfirmed and
     that section is rewritten before Event Storming.
     In practice: a revised scope that includes the size ceiling moves Q3 into
     the Frame, and the event chain then owns a gate.
     Gain: the Frame matches the user's intent before artifacts depend on it.
     Cost: one more turn before the event chain.
- Resolution: **Accept** because the problem, vision, scope and exclusions
  match the direction Q1 selected, and the three stakeholder types carry the
  interests the research identified. Rejected: **Revise** because no section
  was named and no recorded statement contradicts the user's direction.

## Q3 — Is a size ceiling needed after the reduction?

- Status: open (parked; not blocking the Frame)
- Blocks: whether a gate is a change seed of its own, and what number it uses.
- Context: The draft proposes `stat -c %s target/x86_64-pc-windows-gnu/release/duty2cal.exe`
  against 900,000 B, but the executable is 2,097,152 B and the cited 1,035,358 B
  is the compressed archive. Between adjacent releases the archive moved
  +250 B then −306 B (net −56 B), and the executables moved +512 B then
  −1,024 B. Q1 selected the measured-reduction problem, not the growth problem.
- Question: After the reduction, should a size ceiling guard the executable,
  the archive, or nothing?
- Options:
  1. **Ceiling on the executable** — preparation fails when the cross-built
     `duty2cal.exe` exceeds a measured bound.
     In practice: a dependency bump that grows the exe past the bound refuses
     the release and names the size.
     Gain: the artifact that decides the download cannot creep unnoticed.
     Cost: the bound must be re-measured when a capability intentionally grows
     it; a false refusal blocks a release.
  2. **Ceiling on the archive** — the check covers the published `.tar.gz`.
     In practice: the check matches what a downloader receives, so compression
     changes are also caught.
     Gain: it gates the number users experience.
     Cost: it moves for reasons other than executable size (format, packaging),
     which makes the bound noisier.
  3. **No ceiling** — the measurement report is the only guard.
     In practice: each release records the size; nobody is refused.
     Gain: no threshold maintenance.
     Cost: a regression is visible only to a reader who compares releases.
- Resolution: _(open)_

## Q4 — Confirm the Personas for this topic

- Status: closed 2026-10-18 (user)
- Blocks: the advisory reviews of the Event Storming, the use-case definitions
  and the elaborated use cases. Personas are review lenses, never Gherkin
  Actors.
- Context: The Frame names three stakeholder types. Two match confirmed project
  Personas exactly and are reused with topic-local record entries only. The
  third — a contributor reviewing a build-configuration change — shares the
  review-only authority of `contributor--infrastructure-reviewer` but not its
  vocabulary: it judges measured size deltas and panic behavior, where the
  infrastructure reviewer judges declared configuration and previews.
- Question: Is the Persona set confirmed — the reused release owner, the
  reused crew member, and the new build-configuration reviewer?
- Options:
  1. **Confirm all three** — the two reused Personas gain topic record entries
     and the new segment becomes confirmed for this topic.
     In practice: the build-configuration reviewer later reviews the event
     chain and the elaborated use cases, and may flag "the archive's size is
     not the executable's size" as a finding.
     Gain: the measurement claim is judged by a lens that is not the release
     owner's own.
     Cost: one more persona file to keep current; three advisory lenses per
     artifact.
  2. **Keep the two reused, drop the new segment** — the contributor interest
     is folded into the release owner's lens.
     In practice: build-configuration changes are reviewed only through the
     release owner, who also authors them.
     Gain: no new persona to confirm or maintain.
     Cost: no independent lens on whether a size claim is measured; a review
     finding about evidence rigor has no home.
  3. **Keep the two reused, revise the new segment** — name the field to
     change (role, authority boundary, evaluation capability, own vocabulary,
     success criterion).
     In practice: a revised success criterion might require a reproducible
     measurement command rather than a stated delta.
     Gain: the lens matches the user's actual review standard.
     Cost: one more turn before the event chain.
- Resolution: **Confirm all three** because the release owner and the crew
  member match confirmed project Personas exactly, and the new segment
  differs from the infrastructure reviewer in one disjunctive criterion — its
  own vocabulary — while sharing the review-only authority. Rejected: **Drop
  the new segment** because it would leave measurement rigor judged by the
  change's author; **Revise the new segment** because no field was named and
  the recorded one matches the review standard.

## Q5 — Term check for the Event Storming

- Status: closed 2026-10-18 (user)
- Blocks: the Event Storming canvas, the use-case definitions and the seeds,
  which use these four terms.
- Context: The chain needs four nouns that the durable glossary does not
  define: the artifact measured, the declaration changed, the measurement, and
  the unit of change. Collision checks are recorded in `domain-terms.md`.
- Question: Term check — Event Storming: four new core terms. Confirm all, or
  name the terms to revisit.
- Options:
  1. **Distributed executable** — the artifact built for the supported
     platform and packaged in the release archive.
     In practice: the 2,097,152-byte `duty2cal.exe` inside the v0.3.0 archive.
     Gain: separates the executable from the download that carries it.
     Cost: none identified; two existing durable uses agree.
  2. **Build configuration** — the committed declaration of profiles, feature
     selection and target link flags.
     In practice: `[profile.release]` in `Cargo.toml`.
     Gain: names what a size lever changes.
     Cost: sits beside the durable Declared configuration; the glossary
     contrast already distinguishes them.
  3. **Size attribution** — the measurement of where the executable's bytes go.
     In practice: a crate-level and function-level breakdown.
     Gain: the evidence a decision consults.
     Cost: none identified; no durable use.
  4. **Size lever** — one build-configuration change with a measurable effect.
     In practice: `panic = "abort"`.
     Gain: one unit for accept/reject decisions.
     Cost: the draft's "Hebel" becomes durable vocabulary.
- Resolution: **Confirm all four** because each fills its genus slot, has a
  contrast pair and a concrete domain sentence. Rejected: **Revisit "Size
  lever"** and **Revisit "Build configuration"** because no term was named for
  a naming dialog, and the recorded definitions carry the draft's meaning
  without the metaphor.

## Q6 — Is the size attribution produced for every change or only for a release?

- Status: open
- Blocks: the measurement use case's cadence, and whether its change seed adds
  a per-pull-request CI step or only a release-preparation step.
- Context: The draft proposes a `cargo bloat` report on every pull request. The
  Frame selected measured reduction, not a gate (Q3). The project's gate on
  landing already cross-builds the Windows executable and smokes it, so the
  artifact for a measurement exists on every pushed revision; the question is
  whether the attribution is produced and shown then, or only when a release is
  prepared. A release-only measurement arrives after the lever has landed and
  can only refuse, not inform the review.
- Question: When is the size attribution produced — for every change, for every
  release, or on demand?
- Options:
  1. **Every change** — the cross-build step produces the attribution and
     publishes it for the revision.
     In practice: a pull request that adds `panic = "abort"` shows the measured
     delta before it lands.
     Gain: the decision consults evidence that exists at decision time, and a
     reviewer sees the same number as the release owner.
     Cost: CI time on every push and a report format to maintain; a noisy
     report nobody reads is pure cost.
  2. **Every release only** — the release preparation produces the attribution.
     In practice: the release owner sees the attribution when preparing the
     version, after the lever landed.
     Gain: no change to the pull-request pipeline.
     Cost: the lever is reviewed without its delta; a bad lever can only be
     caught and reverted, not rejected beforehand.
  3. **On demand** — the release owner runs the measurement locally.
     In practice: the owner runs the command when a decision needs it.
     Gain: no automation to maintain.
     Cost: no reproducible evidence in the review, and the measurement depends
     on a workstation that can cross-build.
- Resolution: _(open)_

## Q7 — Do the two remaining durable stakeholder types join the Frame?

- Status: closed 2026-10-18 (user)
- Blocks: the stakeholder list confirmed in Q2 and the use-case definitions
  that name the affected stakeholders.
- Context: The durable stakeholder table in chapter 1 names four types: the
  crew member, the person identified in a duty plan, the project
  maintainer/release owner, and the contributor reviewer. The Frame names the
  release owner, the crew member converting a plan, and the contributor
  reviewer as a segment. Two types are missing. The privacy interest matters
  because this work can publish attribution evidence and panic text. The
  source-building crew member matters because the source route is promised
  (`docs/arc42/01-introduction-and-goals.md:35`, risk R-019) and inherits the
  build configuration, including any panic behavior change.
- Question: Do the person identified in a duty plan and the crew member
  building from source on an unsupported platform join this Frame as
  stakeholder types?
- Options:
  1. **Add both, with one-line interests** — the stakeholder list becomes
     complete and the Personas stay as confirmed.
     In practice: the Frame records that no plan-derived content may appear in
     published attribution or panic evidence, and that a source build keeps the
     conversion's failure guarantee.
     Gain: the review lenses cover privacy and source builders before the use
     cases are elaborated.
     Cost: two more interests to check in every artifact; the crew-member
     Persona already covers both with a topic stance, so the rows add
     completeness rather than new analysis.
  2. **Add the source-building crew member only** — the privacy interest is
     already covered by the crew-member Persona's existing record, and this
     work measures crates, not plan content.
     In practice: the Frame records the source route's inheritance of the
     build configuration; privacy stays with the conversion topic.
     Gain: one addition.
     Cost: a published panic sample containing plan data would have no stated
     stakeholder interest against it.
  3. **Add neither** — record why: the attribution measures crates, not plan
     content; the conversion use case's guarantee already covers source
     builders.
     In practice: the Frame keeps three rows and a note naming the two types
     with the reason they add no interest here.
     Gain: no change to a confirmed list.
     Cost: the durable table and this Frame disagree, which the next reader
     must reconcile.
- Resolution: **Add both** because the durable stakeholder table names them,
  and a published attribution or panic sample is a new route by which plan
  content or a source builder's behavior could be affected. Rejected: **Add
  the source-building crew member only** because it leaves published panic
  evidence without a stated privacy interest; **Add neither** because it leaves
  this Frame in conflict with chapter 1. The Personas stay as confirmed: the
  reused crew-member record covers both interests.

## Q8 — Which concrete example carries the reduction's success guarantee?

- Status: open (parked; resolved by the first measurement, not by the user)
- Blocks: the Examples section of `reduce-distributed-executable-size` and its
  scenario skeleton.
- Context: The use case's success guarantee is a measured difference, and the
  first size lever's delta does not exist yet. Inventing a number would
  fabricate evidence, so the elaboration records the gap instead.
- Question: Which measured lever and delta becomes the success example?
- Options:
  1. **The first measured lever** — the delta from the measurement seed's first
     applied lever, recorded with its revision and baseline.
     In practice: the example states the baseline size and the landed size
     measured on `ubuntu-latest`.
     Gain: a real, reproducible example.
     Cost: the example is written after the measurement, not during ideation.
  2. **A generated comparison** — a synthetic build pair in the project's tests.
     In practice: two builds differing only by the lever, generated in CI.
     Gain: an example independent of the first release.
     Cost: a test fixture the size work otherwise does not need.
  3. **No numeric example** — the guarantee stands without one.
     In practice: the Examples section carries only the rejection example.
     Gain: nothing invented.
     Cost: the elaboration review found the section incomplete; a later reader
     cannot see the success path concretely.
- Resolution: _(open)_

## Q9 — Are the Handoff records confirmed for publication?

- Status: closed 2026-10-18 (user)
- Blocks: publishing the topic document, writing the two change seeds,
  promoting the Persona, and proposing the four terms for the durable glossary.
- Context: The Handoff records are the permanent mapping of the two `!` leaves,
  the promotion of the build-configuration reviewer Persona, the four confirmed
  terms, and the two change seeds in start order. Nothing is durable until the
  user confirms them together.
- Question: Are these Handoff records confirmed for publication?
- Options:
  1. **Confirm all** — publish `docs/ideation/release-executable-size.md`,
     write the two seeds, promote the Persona and add the four terms to the
     glossary.
     In practice: two new permanent use-case directories enter the spec corpus,
     and `/givn-propose` can start from either seed.
     Gain: the topic closes with a readable record and ready seeds.
     Cost: two permanent leaves for build-process goals.
  2. **Keep the Persona topic-local** — everything else as shown; the new
     Persona stays under the topic directory.
     In practice: a later build-configuration change reuses the Persona from
     topic history.
     Gain: no project-wide Persona yet.
     Cost: the project collection lacks the review lens.
  3. **Revise a record** — name the mapping, seeds or terms to change; nothing
     is published until the revision is shown again.
     In practice: the work attaches to `publish-public-release` instead, or the
     two seeds merge into one change.
     Gain: the records match the user's intent before becoming durable.
     Cost: one more round.
- Resolution: **Confirm all** because the two `!` leaves passed the definition
  boundary and the mapping rule assigns each its own permanent leaf; the Persona
  adds a review lens without a new authority; and the seeds ship in a real
  order. Rejected: **Keep the Persona topic-local** because the lens belongs to
  the project's review vocabulary; **Revise a record** because no record was
  named for change.
