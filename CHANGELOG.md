## [0.4.0] - 2026-10-18

### New capabilities
- A release is published only after every check on the tagged revision has concluded successfully, so the branch and tag can be pushed together without racing the gate.
- Offer one downloadable archive for Windows, the only platform the project distributes.
- A travel activity's calendar title now ends with its report time in local time — `LU 742 Tychon-Mareum Report 07:30 LT` — so the briefing time is visible without opening the event, while the event details keep the labelled value.

### Fixes
- Find the built Windows executable in the downloaded release artifact, so the release preparation stages the archive instead of failing before publication.
[0.3.0...0.4.0](https://github.com/duty2cal/duty2cal/compare/v0.3.0...v0.4.0)
## [0.3.0] - 2026-10-17

### Dependencies
- Bump the rust-deps group across 1 directory with 7 updates
[0.2.0...0.3.0](https://github.com/duty2cal/duty2cal/compare/v0.2.0...v0.3.0)
## [0.2.0] - 2026-10-14

### Fixes
- A travel activity's calendar title prints the crew position and the flight number as two words separated by a space — an operating leg reads `FB 123 ABC-DEF` instead of `FB123 ABC-DEF`, while positioning travel keeps reading `DH 007 ABC-DEF`.
[0.1.0...0.2.0](https://github.com/duty2cal/duty2cal/compare/v0.1.0...v0.2.0)
## [0.1.0] - 2026-10-13

### New capabilities
- Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.
- Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

### Fixes
- Read the producer's own export layout
