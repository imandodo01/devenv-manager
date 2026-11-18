# DEM — Project Status and Continuation Handoff

**Last updated:** 2026-11-19  
**Repository:** [imandodo01/devenv-manager](https://github.com/imandodo01/devenv-manager)  
**Authoritative branch:** `main`  
**Current milestone:** M3 — Runtime Information & Installed-Version Inventory

## Executive summary

DEM has completed its foundation and application shell. M2's read-only environment inventory has been implemented, merged, and exercised on the user's Windows workstation. The frontend and Rust quality checks passed, and the native desktop app displayed live inventory data. M2's final acceptance remains open for independent executable/version comparison, evidence that Process/User/Machine PATH values were unchanged, and an explicit acceptance record.

## Milestone status

| Milestone | Development | Verification | Acceptance |
|---|---|---|---|
| M0 — Foundation Setup | Completed | Passed | Accepted 2026-11-17 |
| M1 — Application Shell | Completed | Passed | Accepted 2026-11-17 |
| M2 — Environment Inventory | Merged to main | Automated checks passed; Windows visual verification passed; read-only PATH comparison not recorded | Pending final evidence/sign-off |
| M3 — Runtime Information | Not started | Test preparation needed | Pending |
| M4 — Global Runtime Switching | Not started | Safety design required | Pending |
| M5 — Requirement Inspector | Not started | Scope and metadata matrix required | Pending |
| M6 — Integration & UX | Not started | Final E2E plan pending | Pending |

## M2 delivery record

- PR [#12 — Read-only Environment Inventory](https://github.com/imandodo01/devenv-manager/pull/12) merged to `main`.
- Merge commit: [`f2d53e2`](https://github.com/imandodo01/devenv-manager/commit/f2d53e2e9c1027f95fdfee074be5a26eb32ed44b).
- Superseded PR #11 closed; issue #11 closed by merge.
- Local workstation synchronized successfully. Latest status supplied by the user: `## main...origin/main`, clean working tree.

### Verified on user's Windows workstation

| Check | Result |
|---|---|
| `npm run lint` | Passed |
| `npm run typecheck` | Passed |
| `npm test` | 8 passed across 3 files |
| `npm run build` | Passed |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | Passed after follow-up formatting fix |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 17 passed, 0 failed, 2 ignored |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | Passed |
| `npm run tauri dev` | Native app launched |
| Environment UI | Seven tool entries displayed; multiple installations retained; Scan again worked |

### Remaining M2 acceptance evidence

- Compare displayed paths/versions with independently resolved executable paths and version commands.
- Run the before/after Process/User/Machine PATH comparison from `docs/testing/M2_LOCAL_VERIFICATION.md`.
- Record explicit M2 acceptance after reviewing the evidence.
- Observed Node.js diagnostic: configured `NVM_HOME` and `NVM_SYMLINK` roots did not exist, so the Node.js entry was marked inspection-incomplete even though Node.js 22.23.4 was detected. Decide later whether those stale settings should be removed from the user's environment; DEM must not modify them.

## Approved M2 decisions carried forward

- Supported entries: Node.js, PHP, Java, Rust compiler, Cargo, Git, Composer. Python deferred.
- Discovery remains bounded and read-only, using current process PATH, approved manager roots, and documented conventional Windows locations.
- No registry scanning, whole-drive scanning, network catalogs, telemetry, or environment mutation.
- Version commands use validated absolute paths, fixed arguments, timeouts, and bounded output.
- Multiple candidates are retained; no arbitrary active/default version is selected.

## Remaining milestones and effort forecast

| Work | Baseline remaining effort | Main dependency/risk |
|---|---:|---|
| M2 closeout | 0.25–0.5 h | PATH evidence and sign-off |
| M3 Runtime Information | 4–6 h | Consume stable M2 contract; metadata/provenance model |
| M4 Runtime Switching | 6–8 h | Safety ADR for scope, elevation, preview, verification, rollback, audit |
| M5 Requirement Inspector | 4–6 h | Approve repository input and metadata support matrix |
| M6 Integration & UX | 3–5 h | M1–M5 accepted interfaces and end-to-end regression |

**Total current baseline:** approximately **17.25–25.5 focused hours**, excluding separately scoped Repository Inventory work, release packaging/signing, and substantial defects discovered later. These are planning ranges, not commitments.

At 2–3 focused hours per working session, this suggests roughly 6–13 sessions. Calendar time depends on real availability and the spacing of Windows verification/review sessions.

### Important scope risk

The Product Specification includes Repository Inventory (discover/register repositories and maintain Existing, Planned, Pending Setup, Archived states), but the M0–M6 roadmap has no dedicated milestone for delivering that capability. M5 currently assumes an explicitly selected or registered repository. Before M5 implementation, decide whether repository selection/registration is a small prerequisite, a separate milestone, or intentionally deferred. Any added work must be estimated separately.

## Next session — recommended sequence

1. Pull latest `main` and confirm clean status.
2. Close the small M2 PATH evidence/sign-off gap, if practical.
3. Start issue [#6 — M3 Runtime Information](https://github.com/imandodo01/devenv-manager/issues/6) on a focused branch.
4. Define metadata/provenance and installed-version grouping before coding.
5. Reuse M2 discovery and typed detection results; do not introduce a second detector.
6. Keep external catalogs, installation, switching, and PATH edits out of M3.
7. Update this handoff and roadmap as evidence changes.

## Known project-level open decisions

- Repository discovery/registration and worklist persistence.
- M4 Process/User/Machine scope, elevation, preview, rollback, and audit retention.
- Whether external trusted provider catalogs are needed for M3.
- Runtime installation/provider integration (not currently in approved scope).
- Supported Windows versions, installer/signing/update channel, final icon/branding.
- Telemetry/privacy policy; default remains no inventory transmission.

## Git and verification notes

- No local files were changed by the user's final synchronization; working tree was clean.
- This handoff is the current repository record for milestone status and should be updated when new verification evidence, decisions, or estimates are recorded.
- No Actions workflow/status checks are configured in the repository; the M2 test results above are user-run local Windows evidence, not CI results.
