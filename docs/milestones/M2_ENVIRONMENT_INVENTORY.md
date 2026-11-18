# M2 — Environment Inventory & Runtime Detection

**Development:** Completed (merged 2026-11-19) | **Test Preparation:** Ready | **Acceptance:** Pending final read-only verification and explicit sign-off  
**Issue:** [#11 — Implement read-only environment inventory](https://github.com/imandodo01/devenv-manager/issues/11)  
**Planning decision:** [PR #5 — approved support matrix and detection policy](https://github.com/imandodo01/devenv-manager/pull/5)
**Dependency:** M0 and M1 accepted

## Goal

Provide a truthful, read-only inventory of an explicitly agreed initial set of Windows developer tools/runtimes, including version and executable location where available.

## Approved initial support matrix

The initial M2 matrix is accepted:

| Area | Tool | Detection target |
|---|---|---|
| Runtime | Node.js | `node.exe` |
| Runtime | PHP | `php.exe` |
| Runtime | Java | `java.exe` |
| Runtime | Rust toolchain | `rustc.exe` and `cargo.exe` |
| Development tool | Git | `git.exe` |
| Development tool | Composer | `composer.phar` executed through PHP |

Python is deferred to a later phase. Git and Composer are development tools, not runtimes. Rust is one supported toolchain with two independently reported executables.

## Approved detection-source policy

This policy was reviewed and approved on 2026-11-18. Detection remains intentionally conservative and best-effort; it does not promise discovery of every custom installation.

1. Inspect direct executable candidates in the current process `PATH` entries. Do not invoke `where.exe` or a shell to resolve commands.
2. Inspect explicit tool/manager roots when present:
   - Node.js: `NVM_HOME` and `NVM_SYMLINK`.
   - PHP: `PHP_HOME`.
   - Java: `JAVA_HOME`.
   - Rust: `CARGO_HOME` and `%USERPROFILE%\\.cargo`.
3. Inspect a small set of conventional Windows locations:
   - `%ProgramFiles%\\nodejs`, `%ProgramFiles%\\PHP`, `%ProgramFiles%\\Git\\cmd\\git.exe`.
   - Direct child JDK directories under `%ProgramFiles%\\Java`.
   - `%ProgramData%\\ComposerSetup\\bin\\composer.phar`.
4. For version-manager roots that contain version directories, inspect direct child directories only. Do not recurse into arbitrary subdirectories.
5. Composer is discovered as a PHAR file. Read and execute it only through one unambiguous validated `php.exe` candidate with fixed arguments. Never execute `.bat` or `.cmd` wrappers.
6. Do not inspect the Windows registry, scan whole drives, query network catalogs, or transmit local inventory.

Approved hard limits:
- At most 64 current `PATH` entries.
- At most 32 direct child directories per version-manager root.
- At most 8 retained candidates per tool.
- A 3-second timeout per version command.
- At most 8 KiB retained from each of stdout and stderr; continue draining streams after the retained limit is reached.

Any additional source (registry, another version manager, recursive search, network service) requires a separate scope decision.

## Fixed version commands

| Tool | Executable / runner | Fixed arguments |
|---|---|---|
| Node.js | Validated `node.exe` | `--version` |
| PHP | Validated `php.exe` | `--version` |
| Java | Validated `java.exe` | `-version` |
| Rust compiler | Validated `rustc.exe` | `--version` |
| Cargo | Validated `cargo.exe` | `--version` |
| Git | Validated `git.exe` | `--version` |
| Composer | Validated `php.exe` + discovered `composer.phar` | `<composer.phar path> --no-plugins --no-scripts --version` |

No shell interpolation is used. Process exit status and parsed version are separate evidence. Unexpected output remains unknown; it must not be guessed.

## Scope

1. Implement the approved support matrix and reviewed bounded detection-source policy.
2. Define the typed detection result and stable Rust/frontend contract.
3. Implement read-only discovery and validate candidate executable paths.
4. Obtain version metadata with fixed commands, timeouts, and bounded output.
5. Display real inventory results in the Environment page.
6. Add automated tests, local Windows verification instructions, and acceptance records.

## Explicit non-goals

- Installing, downloading, upgrading, uninstalling, or switching runtimes.
- Changing Process, User, or Machine environment variables or PATH.
- Whole-disk or recursive directory scanning.
- Executing project scripts, shell wrappers, or commands constructed from detected values.
- Selecting an arbitrary installation when multiple candidates exist.
- Registry discovery, network lookup, telemetry, or remote inventory upload.
- Claiming support for Python or other ecosystems outside the approved matrix.

## Detection result contract

The contract must preserve these distinct outcomes:

- **Detected:** exactly one validated installation has usable metadata.
- **Not found:** approved sources were inspected successfully and no installation was found.
- **Ambiguous:** multiple installations exist; all retained candidates are shown and no default is selected.
- **Inspection error:** a source or candidate could not be inspected, a process failed/timed out, or version output could not be parsed.

Do not collapse missing, ambiguous, and error into an empty list or a generic false value. If a configured candidate limit is reached or a source cannot be inspected, mark the result incomplete and explain why.

## Safety requirements

- Read-only inspection only.
- Bounded paths and bounded work; no unbounded whole-disk search.
- Canonicalize candidate paths and require the expected executable/PHAR filename before metadata retrieval. This is path validation, not publisher-signature or binary-integrity verification.
- Execute only the validated absolute candidate path with fixed argument lists; no shell interpolation or wrapper execution. Version inspection still executes local binaries, so users should keep their PATH and configured tool roots trusted.
- Enforce timeout and bounded stdout/stderr; kill a timed-out child process.
- Treat process exit code and parsed version as separate evidence.
- Never change PATH or other machine configuration.
- Do not expose full environment variables or unrelated environment values in UI/logs.
- Keep Tauri capabilities minimal; do not add filesystem/process plugins or broad frontend permissions for this feature.

## Acceptance criteria

- [x] Initial supported tool/runtime matrix is documented and approved.
- [x] Detection-source policy and limits are reviewed and approved.
- [x] Detection result contract is typed and distinguishes detected, missing, ambiguous, and inspection-error states.
- [x] Multiple installations are retained; no arbitrary default is implied.
- [x] Discovery is read-only, bounded, deterministic, and limited to approved sources (implementation and tests reviewed).
- [x] Executable paths are validated before metadata retrieval.
- [x] Version retrieval handles malformed output, process errors, timeout, and access failures in automated coverage.
- [x] Environment UI shows real detected data and honest unavailable/ambiguous/error states (Windows screenshots reviewed).
- [x] Tests cover found, missing, duplicate/ambiguous, invalid path, malformed output, process failure, timeout, and relevant error cases.
- [x] Lint, typecheck, frontend tests/build, formatting, Cargo tests, and Clippy pass on the user's Windows machine.
- [ ] Final Windows read-only check confirms Process/User/Machine PATH values are unchanged; visual detection was verified, but before/after PATH evidence was not recorded.
- [x] Product docs, test strategy, and acceptance register are updated.
- [ ] Explicit user acceptance is recorded (implementation has already been merged with the user's merge authorization).

## Verification plan

### Automated
- Unit-test path validation, candidate deduplication, result-state mapping, version parsing, timeout/error mapping, output limits, and deterministic ordering.
- Test Tauri command serialization/contract where practical.
- Frontend tests cover detected, empty, ambiguous, and error UI states.
- Run the project-standard npm and Cargo checks.

### Manual Windows
- Record Windows version and relevant tool versions.
- Verify each supported tool when installed, missing, and duplicated where practical.
- Compare displayed executable paths/versions against independently run commands.
- Confirm no Process/User/Machine PATH or environment values changed.
- Capture concise, redacted evidence and record pass/fail.
- Note custom installations not found by the bounded sources; do not infer that they are absent from the machine.

## Parallel preparation

While M2 implementation owns the runtime detection contract:
- Workstream C (M5) may research candidate project metadata formats and prepare sanitized parser fixtures/test cases.
- Workstream B (M4) may prepare safety ADR options and failure/rollback test cases.
- Workstream D (M6) may audit UX states and prepare E2E scenarios.

These activities must not introduce competing runtime types, start environment mutation, or integrate against an unapproved M2 contract.

## Estimate

5–7 focused person-hours baseline, including discovery, implementation, tests, documentation, and review. Re-estimate if implementation reveals additional Windows-specific complexity.


## Implementation and verification record — 2026-11-19

**Delivery**
- PR [#12](https://github.com/imandodo01/devenv-manager/pull/12) was merged to `main` with squash merge.
- Merge commit: `f2d53e2e9c1027f95fdfee074be5a26eb32ed44b`.
- Superseded PR #11 was closed; implementation issue #11 was closed by the merged PR.
- Local repository was synchronized to `main`; the user's final `git status --short --branch` showed `## main...origin/main` with no working-tree changes.

**Automated checks run by the user on Windows**
| Check | Result |
|---|---|
| `npm run lint` | Passed |
| `npm run typecheck` | Passed |
| `npm test` | 3 files, 8 tests passed |
| `npm run build` | Passed |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | Passed after the formatting follow-up |
| `cargo test --manifest-path src-tauri/Cargo.toml` | 17 passed, 0 failed, 2 ignored |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | Passed |

**Windows application verification**
- `npm run tauri dev` launched the native desktop application successfully.
- Screenshots show all seven inventory entries and multiple installations retained for Java, Rust compiler, Cargo, Git, and Composer.
- Node.js 22.23.4 and PHP 8.1.10 were detected; Java 8 and 17 were shown; Rust compiler/Cargo each showed two candidates; Git showed four paths; Composer showed two PHP/PHAR combinations.
- Scan again completed and the Overview changed Environment inventory to Ready to use.
- Node.js was displayed as detected but with incomplete inspection notes because configured `NVM_HOME` and `NVM_SYMLINK` directories do not exist on this workstation. This is visible and honest diagnostic behavior, but should be reviewed later if the user does not intend to configure NVM.
- No evidence was supplied for independent executable/version comparison using `Get-Command`, or before/after Process/User/Machine PATH equality. Do not mark those checks passed.

**Remaining acceptance gate**
Run the read-only comparison in section 6 of [M2_LOCAL_VERIFICATION.md](../testing/M2_LOCAL_VERIFICATION.md), or explicitly defer it with rationale, then record user acceptance. No PATH or environment mutation is part of this step.
