# M2 — Environment Inventory & Runtime Detection

**Development:** Not Started | **Test Preparation:** Preparing | **Acceptance:** Pending  
**Issue:** [#5 — Environment Inventory & Runtime Detection](https://github.com/imandodo01/devenv-manager/issues/5)  
**Dependency:** M0 and M1 accepted

## Goal

Provide a truthful, read-only inventory of an explicitly agreed initial set of Windows developer tools/runtimes, including version and executable location where available.

## Scope

1. Audit the current Windows/runtime detection options and decide the initial supported tool matrix.
2. Define the typed detection result and stable Rust/frontend contract.
3. Implement bounded, read-only discovery from approved sources only.
4. Validate executable paths and obtain version metadata safely.
5. Display real inventory results in the Environment page.
6. Add automated tests, local Windows verification instructions, and acceptance records.

## Explicit non-goals

- Installing, downloading, upgrading, uninstalling, or switching runtimes.
- Changing Process, User, or Machine environment variables or PATH.
- Whole-disk recursive scanning.
- Executing project scripts or constructing shell commands from detected values.
- Selecting an arbitrary installation when multiple candidates exist.
- Claiming support for every candidate ecosystem before the support matrix is approved.

## Open decisions — resolve before implementation

- Which ecosystems/tools are in the first supported matrix?
- Which bounded detection sources are trusted for each ecosystem (PATH lookup, known install locations, registry, version-manager directories, or other sources)?
- How are duplicate installations represented and how is ambiguity surfaced?
- Which version command/arguments are permitted for each adapter?
- What timeout and output limits are appropriate?
- Which filesystem/process permissions are required, and can the design avoid adding broad Tauri capabilities?
- How are localized/unexpected version outputs represented without guessing?

Candidate tools for evaluation, not pre-approved requirements: Node.js, PHP, Java, Git, Composer, Rust/Cargo, Python.

## Proposed detection result states

The exact names/types are an implementation decision, but the contract must preserve these distinct outcomes:

- **Detected:** one or more validated installations with known metadata.
- **Not found:** approved sources were checked successfully and no installation was found.
- **Ambiguous:** multiple candidates exist and no single selection is justified.
- **Inspection error:** a source could not be inspected or a candidate could not be validated.

Do not collapse missing, ambiguous, and error into an empty list or a generic false value.

## Safety requirements

- Read-only inspection only.
- Bounded paths and bounded work; no unbounded whole-disk search.
- Validate and normalize candidate executable paths before use.
- If a version process is needed, invoke a fixed executable with fixed arguments; do not use shell interpolation.
- Enforce timeout and bounded stdout/stderr.
- Treat process exit code and parsed version as separate evidence.
- Never change PATH or other machine configuration.
- Do not expose sensitive environment values in UI/logs.

## Acceptance criteria

- [ ] Initial supported tool/runtime matrix and detection sources are documented and approved.
- [ ] Detection result contract is typed, documented, and distinguishes detected, missing, ambiguous, and error.
- [ ] Multiple installations are retained; no arbitrary default is implied.
- [ ] Discovery is read-only, bounded, deterministic, and limited to approved sources.
- [ ] Executable paths are validated before metadata retrieval.
- [ ] Version retrieval handles malformed output, process errors, timeout, and access failures.
- [ ] Environment UI shows real detected data and honest unavailable/ambiguous/error states.
- [ ] Tests cover found, missing, duplicate/ambiguous, invalid path, malformed output, process failure, timeout, and permission/error cases as applicable.
- [ ] Lint, typecheck, frontend tests/build, and relevant Cargo checks pass.
- [ ] Windows local verification confirms expected detections and no PATH/environment changes.
- [ ] Product docs, test strategy, and acceptance register are updated.
- [ ] User acceptance is recorded before merge.

## Verification plan

### Automated
- Unit-test path validation, candidate deduplication, result-state mapping, version parsing, timeout/error mapping, and deterministic ordering.
- Test Tauri command serialization/contract where practical.
- Frontend tests cover detected, empty, ambiguous, and error UI states.
- Run the project-standard npm and Cargo checks.

### Manual Windows
- Record Windows version and relevant tool versions.
- Verify each supported tool when installed, missing, and duplicated where practical.
- Compare displayed executable paths/versions against independently run commands.
- Confirm no Process/User/Machine PATH or environment values changed.
- Capture concise, redacted evidence and record pass/fail.

## Parallel preparation

While M2 implementation owns the runtime detection contract:
- Workstream C (M5) may research candidate project metadata formats and prepare sanitized parser fixtures/test cases.
- Workstream B (M4) may prepare safety ADR options and failure/rollback test cases.
- Workstream D (M6) may audit UX states and prepare E2E scenarios.

These activities must not introduce competing runtime types, start environment mutation, or integrate against an unapproved M2 contract.

## Estimate

5–7 hours initial planning estimate. Re-estimate after the support matrix and detection-source audit.
