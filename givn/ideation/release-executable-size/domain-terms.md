# Release executable size — domain terms

Status: four terms confirmed by the user on 2026-10-18 through the Event
Storming term check (Q5). The durable arc42 glossary is not edited here; the
confirmed terms are proposed for promotion at Handoff.

| Term | Status | Definition | Contrast | First use | Collision check |
|---|---|---|---|---|---|
| Distributed executable | confirmed | The artifact that is built for `x86_64-pc-windows-gnu` and packaged in the release archive; the thing whose bytes a downloader runs. | The Linux host build, the compressed archive and the source are not the distributed executable. | `docs/arc42/11-risks-and-technical-debt.md:19` | `git grep -ic 'distributed executable' -- ':!target' ':!givn/ideation'` → 2 files (risk R-013 and the archived windows-only-release proposal, both using the same meaning); sibling search `git grep -ic 'distributed executable' -- givn/ideation ':!givn/ideation/release-executable-size'` → 0. |
| Build configuration | confirmed | The committed declaration that decides what the distributed executable contains: Cargo profiles, feature selection and target link flags. | Not the declared configuration under `infra/`, which states how the organization and its repositories should be configured and is applied to the live public home; not the release workflow, which decides what an archive holds. | `docs/arc42/12-glossary.md:45` | `grep -ic 'build configuration' docs/arc42/12-glossary.md` → 1; sibling search `git grep -ic 'build configuration' -- givn/ideation` → 2 files (`public-home-infrastructure/domain-terms.md:21`, `handoff.md:25`), both the same Declared-configuration contrast; repository-wide `git grep -ic 'build configuration' -- ':!target'` → 3 files. No competing meaning. |
| Size attribution | confirmed | The measurement that breaks down the code the distributed executable contains, by crate and function, from a symbol-bearing rebuild of the same revision; the stripped artifact's bytes are measured separately. | Not the stripped file's byte count; not the archive's size; not an estimate or a percentage borrowed from another project. | `givn/ideation/release-executable-size/frame.md:16` | `grep -ric 'size attribution' givn/ideation` → 2; `git grep -ic 'size attribution' -- ':!target' ':!givn/ideation'` → 0; `grep -ic 'size attribution' docs/arc42/12-glossary.md` → 0; sibling search → 0. |
| Size lever | confirmed | The build-configuration change whose effect on the distributed executable can be measured and accepted or rejected. | Not a nightly-only compiler flag, excluded by the stable-toolchain constraint; not UPX or dynamic linking, excluded by the distribution constraints. | `givn/ideation/release-executable-size/frame.md:9` ("stable profile levers") | `git grep -ic 'size lever' -- ':!target' ':!givn/ideation'` → 0; `grep -ic 'size lever' docs/arc42/12-glossary.md` → 0; `grep -ric 'size lever' givn/specs` → 0; sibling search → 0. |

## Term introduction test

Each entry fills the genus slot, keeps a contrast pair, and has one concrete
domain sentence.

- **Distributed executable** is an **artifact** that is built for the supported
  platform and packaged in the release archive. Distributed executable, not
  release archive: the archive carries the executable, and its compressed size
  is a different number. Sentence: the v0.3.0 distributed executable is
  2,097,152 bytes, while the release archive that carries it is 1,035,358
  bytes.
- **Build configuration** is a **committed declaration** that decides what the
  distributed executable contains. Build configuration, not declared
  configuration: the latter states how the organization and its repositories
  should be configured, declared under `infra/` and applied to the live public
  home; the product's compilation settings are outside it.
  Sentence: changing `panic` in the build configuration changes the distributed
  executable, not the archive's name.
- **Size attribution** is a **measurement** that names where the code inside
  the distributed executable spends its size, by crate and by function, from a
  symbol-bearing rebuild. Size attribution, not stripped artifact size: the
  attribution's totals describe a rebuild with symbols, while the stripped
  artifact's bytes are measured separately. Sentence: the size attribution
  names the crates, and the stripped artifact's 2,097,152 bytes are reported
  beside it.
- **Size lever** is a **build-configuration change** whose effect on the
  distributed executable is measurable. Size lever, not nightly compiler flag:
  a flag the stable toolchain cannot carry is not a candidate at all. Sentence:
  `panic = "abort"` is a size lever only if the measurement shows the bytes it
  removes.
