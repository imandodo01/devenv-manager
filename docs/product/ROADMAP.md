# DEM Roadmap

**Status:** M0 and M1 accepted on 2026-11-17. M2 implementation is in progress; acceptance remains pending.

| Milestone | Focus | Exit outcome | Dependency |
|---|---|---|---|
| M0 | Foundation Setup | Tauri app runs/builds locally; React and Rust integrated; npm/Cargo checks available | User prepares/verifies |
| M1 | Application Shell | Layout, navigation, dashboard, reusable UI foundation | M0 — Accepted |
| M2 | Environment Inventory & Runtime Detection | Supported tools/runtimes shown with versions and locations | M0; M1 UI contracts |
| M3 | Runtime Information & Version Management | Runtime metadata and available-version information | M2 detection model |
| M4 | Runtime Switching | Controlled global selection, permissions, verification, traceability | M2/M3; safety design |
| M5 | Dependency & Configuration Inspector | Repository requirements compared with local capabilities | Inventory and repository model |
| M6 | Integration & UX Refinement | Integrated behavior, errors, docs, final acceptance | M1–M5 as applicable |

## Parallel preparation and test queue

Parallel workstream planning for M2–M6 is documented in [WORKSTREAM_PLAN_M2_M6.md](../development/WORKSTREAM_PLAN_M2_M6.md).

While a milestone is implemented or locally tested, prepare subsequent technical investigations, acceptance criteria, automated test design, and manual Windows verification. Do not implement dependent features against unstable/unapproved interfaces. Keep PRs independently reviewable.

## Track three statuses independently

- **Development:** Not Started / In Progress / Ready for Review / Completed
- **Test Preparation:** Not Prepared / Preparing / Ready
- **Acceptance:** Pending / Passed / Failed

Development completion does not imply acceptance. Milestone acceptance requires relevant tickets to meet Definition of Done and user acceptance to be recorded.

## Estimates

Earlier estimates (M0 3–5h; feature work roughly 26–38h) are rough planning references, not commitments. The current M2–M6 planning estimate is 22–32h; revise after the M2 technical audit and scope decisions.
