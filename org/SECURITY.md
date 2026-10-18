<!-- SPDX-FileCopyrightText: 2026 Daniel Kressner -->
<!-- SPDX-License-Identifier: CC0-1.0 -->
# Security policy

This is the organization-wide default security policy for the `duty2cal`
organization. It applies to a repository in the organization that has no
security policy of its own.

## What is in scope

- A public artifact that exposes a duty plan or the people in it.
- A conversion defect that silently misrepresents a duty plan.
- A release asset that does not match its checksum or provenance.
- A credential in a repository or its history.
- A defect in the project's own automation.

## How to report, privately

Use the private vulnerability reporting route:
<https://github.com/duty2cal/duty2cal/security/advisories/new>.

Do not open a public issue for a report that exposes a duty plan or the people
in it. Never attach a real duty plan or a converted calendar to a report; a
stripped sample is enough.

## Supported versions

Only the latest published release is supported. Earlier releases are not
supported, because releases are immutable.

## How a report is handled

A report is acknowledged, assessed, and fixed and released if it is in scope.
An advisory is published with credit unless you ask to remain anonymous. No
bounty is offered.
