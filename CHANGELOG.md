## [0.3.0] - 2026-10-17

### Dependencies
- Bump the rust-deps group across 1 directory with 7 updates
[0.2.0...0.3.0](https://github.com/kressnerd/duty2cal/compare/v0.2.0...v0.3.0)
## [0.2.0] - 2026-10-14

### Fixes
- A travel activity's calendar title prints the crew position and the flight number as two words separated by a space — an operating leg reads `FB 123 ABC-DEF` instead of `FB123 ABC-DEF`, while positioning travel keeps reading `DH 007 ABC-DEF`.
[0.1.0...0.2.0](https://github.com/kressnerd/duty2cal/compare/v0.1.0...v0.2.0)
## [0.1.0] - 2026-10-13

### New capabilities
- Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.
- Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

### Fixes
- Read the producer's own export layout
