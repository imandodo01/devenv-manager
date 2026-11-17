# Acceptance Test Register

This register describes expected behavior; it is not evidence that features exist. Add concrete data, edge cases, commands, and evidence per ticket.

| ID | Area | Acceptance expectation | Verification | Status |
|---|---|---|---|---|
| AT-001 | M0 | Tauri app launches and React UI renders on target Windows | Manual + build | Passed — 2026-11-17 |
| AT-002 | M0 | Rust backend integrated and Cargo tests execute | Automated + manual | Passed — 2026-11-17 |
| AT-003 | M0 | Frontend lint/typecheck/test/build scripts documented and work | Automated + inspection | Passed — 2026-11-17 |
| AT-004 | Detection | Supported runtime shows accurate version and executable location | Automated + Windows | Not Prepared |
| AT-005 | Detection | Missing/ambiguous runtime reported without crash | Automated + Windows | Not Prepared |
| AT-006 | Switching | Proposed environment change previewed and requires explicit approval | Automated + Windows | Not Prepared |
| AT-007 | Switching | Machine PATH is not changed without explicit approval | Automated + Windows | Not Prepared |
| AT-008 | Switching | Result verified; failure reported without trusting exit code alone | Automated + Windows | Not Prepared |
| AT-009 | Inspector | Metadata parsed safely; requirement cites source file | Unit + integration | Not Prepared |
| AT-010 | Compatibility | Satisfied, missing, incompatible, and unknown states distinguished | Unit + integration | Not Prepared |
| AT-011 | Repository | Existing, Planned, Pending Setup, Archived remain distinguishable and user-controlled | UI + integration | Not Prepared |\n| AT-012 | M1 | Application shell, primary navigation, honest placeholder states, and responsive layout work | Automated + Windows manual | Preparing |
