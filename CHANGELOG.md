## [1.2.1] - 2026-10-29

### New capabilities
- The converter reads the supported duty plan's file structure itself, so the Windows download and the browser bundle no longer carry a third-party PDF library while every published calendar, warning, report and refusal stays the same.

### Fixes
- Slice at the header and pin every accepted spelling
[1.2.0...1.2.1](https://github.com/duty2cal/duty2cal/compare/v1.2.0...v1.2.1)
## [1.2.0] - 2026-10-28

### New capabilities
- The Windows download and the browser bundle now carry the converter's own reading of the supported duty plan instead of a general-purpose PDF extraction stack, so both artifacts shrink substantially while the calendar, warnings and numbered reports stay the same.
- Every calendar entry for an unrecognized duty code is prefixed with `unknown duty: <code>`, and the browser's ready page lists those duties and offers the calendar and the publisher warning with clear save labels.
[1.1.0...1.2.0](https://github.com/duty2cal/duty2cal/compare/v1.1.0...v1.2.0)
## [1.1.0] - 2026-10-27

### New capabilities
- Every public statement now names the duty plan duty2cal converts: a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export.
- The public website carries an Impressum and a Datenschutzerklärung that every page links, and the publication refuses a page set that lacks them.
- Every row of a supplied duty plan is reported in one run: an activity code the converter does not recognize is converted from its printed times and station with a warning, a row that cannot be represented at all still fails the conversion, and both lists appear in the report file and on the conversion page.

### Fixes
- The converter page's footer names the Built revision, the local-processing note and the two notice links, and no longer repeats the statements panel's sentences.

### Dependencies
- Bump tokio from 1.53.3 to 1.54.1 in the rust-deps group
[1.0.0...1.1.0](https://github.com/duty2cal/duty2cal/compare/v1.0.0...v1.1.0)
## [1.0.0] - 2026-10-27

### New capabilities
- A crew member can convert one supported duty plan to a complete calendar in a current browser, with the plan never leaving the tab, or receive an explicit failure with an offered diagnostic.
- The public website stays as it is when a revision changes none of its inputs, instead of republishing an unchanged page
- A release tag publishes the site from the tagged revision, so the page and the release name the same source
- A page that requests another origin is refused before it can replace the published one
- The public website and the project's README show the duty2cal mark as a scalable vector icon, and the site's tab declares that vector icon with the existing PNG as its fallback for browsers that cannot use it.
[0.4.0...1.0.0](https://github.com/duty2cal/duty2cal/compare/v0.4.0...v1.0.0)
## [0.5.0] - 2026-10-19

### New capabilities
- A travel activity's calendar title now ends with its report time in local time — `LU 742 Tychon-Mareum Report 07:30 LT` — so the briefing time is visible without opening the event, while the event details keep the labelled value.
- The Windows executable is smaller
[0.4.0...0.5.0](https://github.com/duty2cal/duty2cal/compare/v0.4.0...v0.5.0)
## [0.4.0] - 2026-10-18

### New capabilities
- A release is published only after every check on the tagged revision has concluded successfully, so the branch and tag can be pushed together without racing the gate.
- Offer one downloadable archive for Windows, the only platform the project distributes.
- A travel activity's calendar title now ends with its report time in local time — `LU 742 Tychon-Mareum Report 07:30 LT` — so the briefing time is visible without opening the event, while the event details keep the labelled value.

### Fixes
- Find the built Windows executable in the downloaded release artifact, so the release preparation stages the archive instead of failing before publication.
- Read the release-workflow check suites from each API page, so the publication gate runs with the installed GitHub CLI.
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
