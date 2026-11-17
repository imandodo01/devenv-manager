# M2 — Environment Inventory & Runtime Detection

**Status:** Planned; implementation not started | **Dependencies:** M0 accepted; M1 UI contracts available

## Scope
Detect agreed runtimes/tools; capture version and executable location; distinguish detected, missing, ambiguous, and failed states; display inventory.

## Acceptance criteria
- Supported runtime reports verified version/location.
- Missing tool is clearly reported.
- Detection failure differs from absence.
- Detection does not modify environment configuration.
- Unit and Windows checks cover common and ambiguous installations.

## Test preparation
Define supported-runtime matrix, discovery precedence, PATH vs known-location behavior, duplicate-version cases, and safe sample environments before implementation.
