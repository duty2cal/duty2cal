<!-- SPDX-FileCopyrightText: 2026 Daniel Kressner -->
<!-- SPDX-License-Identifier: CC0-1.0 -->
# duty2cal

Turns a crew duty plan (PDF) into an iCalendar (`.ics`) file on your own
machine. The conversion runs locally: no account, no network, no service, and
no file leaves your device. A plan it cannot read is reported as a failure,
never as a partial calendar.

**Download it:** [releases](https://github.com/duty2cal/duty2cal/releases/latest)
for Windows x86_64. Drag a duty plan PDF onto the executable.

The source, the conversion rules, the tests and the release automation live in
[duty2cal](https://github.com/duty2cal/duty2cal). Other platforms are not
supported or distributed; their source can be built with a Rust toolchain.

| Repository | Purpose |
|---|---|
| [duty2cal](https://github.com/duty2cal/duty2cal) | Source: Rust core, CLI, infrastructure and release automation |
| [duty2cal.github.io](https://github.com/duty2cal/duty2cal.github.io) | Project website — generated output; it does not host the converter today |
| [.github](https://github.com/duty2cal/.github) | This profile and the organization's default security policy |

The project is licensed under MPL-2.0; this page is CC0-1.0. Third-party
Unicode data is licensed under
[Unicode-3.0](https://github.com/duty2cal/duty2cal/blob/main/LICENSES/Unicode-3.0.txt).
Provided "as is", without warranty; verify every generated calendar against your
official roster. Not affiliated with any duty-plan provider or airline.

Found a bug? [Open an issue](https://github.com/duty2cal/duty2cal/issues).
Security concern? See [SECURITY.md](https://github.com/duty2cal/.github/blob/main/SECURITY.md).
