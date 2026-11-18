# DEM — Parallel Workstream Plan (M2–M6)

**Status:** Updated execution baseline — 2026-11-19  
**Prepared:** 2026-11-17  
**Base:** `main` after M1 acceptance and the M2 planning decision in PR #5  
**Purpose:** Enable useful parallel preparation and implementation without allowing unstable contracts or Windows-only verification to become integration risks.

## 1. Current baseline (updated 2026-11-19)

- M0 Foundation Setup — accepted.
- M1 Application Shell — accepted.
- M2 Environment Inventory — implemented and merged to `main` via PR #12 (`f2d53e2`); user Windows automated checks and visual application verification passed. Final PATH before/after evidence and explicit acceptance remain pending.
- M3–M6 — not implemented; M3 is next.
- The native application now has a read-only Tauri inventory command and live Environment page. The M2 detection contract is the foundation for M3; do not duplicate discovery logic.
- The default Tauri capability remains narrow (`core:default`); any filesystem/process permissions or plugins must be justified and reviewed before introduction.
- Product Specification and ADRs still leave repository discovery, environment scopes, and version provisioning open.

This plan does not convert open product questions into approved requirements.

## 2. Workstream map

| ID | Workstream | Primary issue | Outcome | Execution mode |
|---|---|---|---|---|
| A | Runtime Core | [#11 M2 implementation](https://github.com/imandodo01/devenv-manager/issues/11), [#6 M3](https://github.com/imandodo01/devenv-manager/issues/6) | Read-only inventory, detection contract, runtime metadata and installed-version model | M2 first; M3 follows stable M2 contract |
| B | Runtime Operations | [#7 M4](https://github.com/imandodo01/devenv-manager/issues/7) | Safe, previewed, approved global runtime switching with verification and recovery | Design/test preparation can begin after M2 contract review; implementation after M2/M3 |
| C | Project Intelligence | [#8 M5](https://github.com/imandodo01/devenv-manager/issues/8) | Parse agreed project metadata and compare requirements with local capabilities | Research, fixtures, and test design can run alongside M2; implementation after capability contract is stable |
| D | Integration & UX | [#9 M6](https://github.com/imandodo01/devenv-manager/issues/9) | Integrated, accessible, consistent end-to-end workflows | UX audit and test planning can run early; final integration after M1–M5 |

## 3. Dependency graph

```text
                         M1 accepted
                              |
                              v
                     M2 contract + detection
                       /              \
                      v                v
                M3 runtime model   M5 implementation
                      |                ^
                      v                |
                M4 switching       M2 capability contract
                       \              /
                        \            /
                         v          v
                       M6 integration
```

M5 preparation is independent; M5 implementation is not independent of the M2 capability contract. M4 design and safety-test preparation can proceed early, but no environment-changing implementation starts before M2/M3 and the safety ADR are accepted.

## 4. Workstream boundaries

### A — Runtime Core (M2 → M3)

**M2 — Environment Inventory & Runtime Detection**
- First decide the initial supported runtime/tool matrix and trusted detection sources.
- Define a typed, stable result contract before parallel consumers implement against it.
- Distinguish detected, missing, ambiguous, and inspection-error states.
- Preserve multiple installations; do not silently choose a version.
- Keep discovery read-only, bounded, deterministic, and limited to approved locations/sources.
- Validate executable paths before use.
- If version commands are needed, use fixed executable/argument boundaries, no shell interpolation, timeouts, and bounded output.
- No PATH/environment mutation, installation, or switching.

Candidate ecosystems to evaluate — not approved scope: Node.js, PHP, Java, Git, Composer, Rust/Cargo, Python. Select based on actual product value and testability, not simply because they are installed on one developer's machine.

**M3 — Runtime Information & Version Management**
- Consume M2's contract; do not create a second discovery mechanism.
- Model metadata, provenance, multiple installed versions, and version normalization/comparison.
- Decide whether information covers installed versions only or also trusted external catalog references.
- Installation/download/update is excluded unless separately approved.

### B — Runtime Operations (M4)

- Resolve Process/User/Machine scope, permission/elevation, preview, verification, rollback, and audit retention in an ADR before implementation.
- Preview the exact target and every affected environment entry.
- Require explicit confirmation immediately before modification.
- Preserve unrelated environment entries.
- Verify effective state; exit code alone is insufficient.
- Explain that already-running processes may retain inherited values while newly launched processes generally observe updated values.
- Never change Machine PATH automatically.

### C — Project Intelligence (M5)

Preparation may run in parallel with M2:
- Inventory candidate project metadata formats.
- Build sanitized fixtures for valid, malformed, conflicting, missing, and unsupported declarations.
- Draft parser/comparison test cases.
- Propose requirement result states and source attribution.

Implementation waits for the M2 capability contract and an explicit decision on repository input.

Candidate metadata to evaluate — not approved scope: `package.json`, lockfiles, `.nvmrc`, `.node-version`, `composer.json`, `composer.lock`, `Cargo.toml`, `rust-toolchain.toml`, `pyproject.toml`, `.python-version`, `pom.xml`, and Gradle toolchain files.

**Product-scope gap to resolve:** Product Specification includes Repository Inventory (discover/register local repositories and maintain Existing/Planned/Pending Setup/Archived states), but the current M0–M6 roadmap has no dedicated repository-inventory milestone. M5 must not silently grow into broad disk discovery or a general project manager. Before M5 implementation, decide whether its input is an explicitly selected folder, an already registered repository, or a separately scoped repository-inventory deliverable.

### D — Integration & UX (M6)

Preparation can start early:
- Audit current placeholder pages and navigation.
- Define common loading, empty, missing, ambiguous, permission, timeout, and failure patterns.
- Prepare accessibility and keyboard-navigation checks.
- Prepare cross-workstream regression scenarios.

Final integration waits for accepted interfaces and implementations. Do not introduce new runtime ecosystems or metadata formats during M6.

## 5. Safe parallel schedule

| Stage | Workstream A | Workstream B | Workstream C | Workstream D |
|---|---|---|---|---|
| Planning gate | M2 support matrix + contract | Safety ADR outline and threat/failure cases | Metadata research + fixture/test design | UX audit + E2E test outline |
| M2 implementation | Implement and test detection | Review contract only; no modifying code | Continue fixtures; no capability integration yet | Prepare UI states and accessibility checks |
| After M2 accepted | M3 runtime metadata/version model | Finalize M4 design against stable runtime identity | Implement parser/comparison against stable capability contract and approved repository input | Integrate completed slices incrementally |
| After M3 accepted | Fix/maintain core | Implement M4 in isolated branch | Finish M5 | Cross-workstream integration |
| Final gate | Regression support | Windows safety verification | Requirement/repository verification | M6 E2E, UX, documentation, acceptance |

Use only the smallest useful concurrency. If the same contract or high-contention files need to change, pause and agree the interface/ownership first.

## 6. Shared contract and file ownership

The M2 issue owns:
- Runtime detection domain model and frontend/backend contract.
- Rust-side detection boundary and approved process-execution policy.
- Environment page's initial real-data integration.
- Detection test fixtures and Windows verification guide.

The M3 issue owns:
- Runtime metadata/version model built on M2 types.
- Runtime detail/version presentation.

The M4 issue owns:
- Environment mutation service/commands, preview, approval flow, verification, rollback, and audit record.
- It must not redefine detection identity or version semantics.

The M5 issue owns:
- Metadata readers/parsers, requirement model, comparison logic, and Requirements UI.
- It consumes M2 capability results; it does not own runtime discovery.

The M6 issue owns:
- Cross-feature integration and shared UX refinements.
- Avoid concurrent edits to shared navigation/layout files; agree ownership or integrate in order.

Any change to a shared public type must be proposed in its owning issue/PR and communicated before dependent implementation proceeds.

## 7. Verification matrix

### M2 detection
- Supported executable found and valid version parsed.
- Executable absent.
- Multiple installations / ambiguous selection.
- Invalid or stale path.
- Malformed, unexpected, or localized version output.
- Process start failure, non-zero exit, timeout, oversized output.
- Permission/access error.
- Repeated scan gives deterministic result.
- No PATH or environment values changed.

### M3 runtime metadata
- Version normalization and comparison for supported formats.
- Multiple versions retained.
- Unknown or malformed version remains unknown, not guessed.
- Installed vs external catalog information remains distinguishable.

### M4 switching
- Preview accurately describes target and changes.
- User cancellation is a no-op.
- Permission denial and invalid target are safe.
- Unrelated PATH entries preserved.
- Partial failure and verification mismatch produce a recoverable outcome.
- New process sees expected value; existing-process inheritance is explained/tested.
- Machine scope is never changed without explicit approval and required elevation.

### M5 requirements
- Missing, malformed, conflicting, and unsupported metadata.
- Version/range parsing and comparison.
- Source file attribution for every extracted requirement.
- Missing local inventory and unknown requirement remain distinct.
- Parser never executes project code or scripts.
- Repository files remain unchanged.

### M6 integration
- End-to-end Environment → Runtime Information → Switching flow.
- Repository → Requirements flow with source attribution.
- Loading/empty/error states, accessibility, keyboard use, narrower window.
- No fabricated data or unapproved machine/repository mutation.

## 8. Branch and PR policy

- Keep `main` as the single authoritative base.
- One focused branch and PR per issue; use names such as `feature/m2-environment-inventory`, `feature/m3-runtime-information`, `feature/m4-runtime-switching`, `feature/m5-requirement-inspector`, and `feature/m6-integration-ux`.
- Do not create dependent feature branches from an unmerged feature branch unless a temporary stacked-PR plan is explicitly documented.
- Every PR links its issue, states scope/non-goals, records automated checks, and gives exact Windows local verification steps.
- User performs Windows local verification and explicitly accepts before merge.
- Development, Test Preparation, and Acceptance are tracked independently.
- Keep PRs focused; do not bundle all milestones into one branch.

## 9. Next work session — M3 planning and implementation

1. Start from a clean, synchronized `main`.
2. Review M2's Rust/TypeScript contract and issue #6; M3 must consume the existing contract rather than add a second discovery mechanism.
3. Decide the M3 metadata/provenance schema and installed-version grouping rules.
4. Keep external version catalogs out of scope unless a trusted provider and freshness policy are explicitly accepted.
5. Add version normalization/comparison only for the approved supported formats.
6. Add tests, UI states, and a Windows verification guide; no PATH/environment changes or runtime installation.
7. In a separate short M2 closeout step, record Process/User/Machine PATH equality and explicit acceptance if desired.

### Remaining effort forecast (planning, not a deadline)

- M2 final read-only evidence/acceptance: 0.25–0.5 focused hours.
- M3: 4–6 focused hours (issue #6 baseline).
- M4: 6–8 focused hours (issue #7 baseline; safety ADR must be approved before implementation).
- M5: 4–6 focused hours after repository-input and metadata-scope decisions (issue #8 baseline; repository inventory may require separate scope/estimate).
- M6: 3–5 focused hours (issue #9 baseline).
- **Current remaining baseline: approximately 17.25–25.5 focused hours**, excluding any separately scoped Repository Inventory work, substantial defects found in Windows verification, or release packaging/signing.
- Confidence: Medium-low. M3–M6 estimates are ticket baselines; M5 has unresolved input-scope risk and M4 has Windows permission/rollback risk.
- At an assumed 2–3 focused hours per working session, this is roughly 6–13 sessions. Calendar duration depends on available evenings/weekends and review/verification gaps; this is not a delivery date.

## 10. Previous M2 preparation checklist — superseded

The original M2 preparation checklist has been superseded by the 2026-11-19 execution baseline and handoff in [PROJECT_STATUS.md](PROJECT_STATUS.md). Keep this plan focused on milestone dependencies and scope; use the project status record for the current next action and estimate.

## 10. Definition of Ready / Done

A work item is Ready when:
- Scope and non-goals are explicit.
- Dependencies and shared contracts are identified.
- Acceptance criteria are observable.
- Automated and manual verification are defined.
- Open product decisions are either resolved or explicitly excluded.

A work item is Done only when:
- Implementation and documentation meet scope.
- Relevant automated checks actually pass.
- Windows-specific verification is recorded by the user.
- PR review and user acceptance are complete.
- Remaining limitations are recorded.

## 11. Estimates

Initial issue estimates (planning only):
- M2: 5–7 hours.
- M3: 4–6 hours.
- M4: 6–8 hours.
- M5: 4–6 hours after scope decisions.
- M6: 3–5 hours.

Total serial planning estimate: 22–32 hours. Parallel preparation may reduce calendar waiting but does not remove engineering, review, or Windows verification effort. Re-estimate after the M2 support-matrix audit.
