# M0 — Foundation Setup

**Owner:** User | **Development:** Complete | **Acceptance:** Accepted by user — 2026-11-17

## Scope

Foundation only; no DEM product features.

## Completed tasks

- [x] Confirm Node.js/npm, Rust/Cargo, and Windows build prerequisites.
- [x] Prepare Tauri 2 + React + TypeScript + Vite foundation.
- [x] Integrate the Rust backend entry point and minimal capability configuration.
- [x] Integrate Tailwind CSS 4 through the Vite plugin.
- [x] Prepare npm scripts for lint, typecheck, test, build, and formatting.
- [x] Prepare Cargo test and formatting workflow.
- [x] Generate and commit npm and Cargo lockfiles after local dependency resolution.
- [x] Verify frontend lint, typecheck, test, and build commands on Windows.
- [x] Verify Cargo formatting, test, and Clippy commands on Windows.
- [x] Launch the native app and confirm the React screen renders.
- [x] Build the Windows application executable.
- [x] Record local verification results and user acceptance.

## Exit criteria

- [x] Tauri app launches; React renders.
- [x] Rust backend is integrated.
- [x] Frontend lint/typecheck/test/build scripts work.
- [x] Cargo tests run.
- [x] Windows application executable builds successfully.
- [x] Source and lockfiles are committed and pushed.

## Workstation prerequisite evidence

The maintainer workstation was checked before scaffold work:
- Node.js 20.20.2 and npm 10.8.2 were directly executable; Node 22.23.4 and npm 10.9.9 are installed and selected as the project baseline for Vitest 5.
- Rust 1.99.1, Cargo 1.99.1, and `stable-x86_64-pc-windows-msvc` were detected.
- Visual Studio 2019 Build Tools, MSVC 14.29.30154, Windows SDK 10.0.19041.0, and WebView2 Runtime 157.0.4341.46 were detected.
- A native C compile/link/execute smoke test completed with exit code 0 and printed `DEM M0 toolchain OK`.

## Local Windows verification

Executed by the user on the Windows workstation. The supplied terminal log records:

| Check | Result |
|---|---|
| `npm run lint` | Passed |
| `npm run typecheck` | Passed |
| `npm test` | Passed — 1 test |
| `npm run build` | Passed |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | Passed |
| `cargo test --manifest-path src-tauri/Cargo.toml` | Passed — 1 test |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | Passed |
| `npm run tauri dev` | User confirmed native app window opened and rendered; closed normally |
| `npm run tauri build` | Passed — Windows executable generated |

Build output: `src-tauri/target/release/devenv-manager.exe`.

The user subsequently committed `package-lock.json` and `src-tauri/Cargo.lock` in commit `f61cbd2` (`chore(m0): commit dependency lockfiles`) and pushed the branch. The local working tree was confirmed clean and synchronized with origin.

## Verification limitations

- No GitHub Actions workflow run was present for the PR head commit; verification above is local Windows evidence.
- No screenshot was attached to the verification log. The user explicitly confirmed that the native window appeared and rendered.
- Installer bundling remains disabled by design; this milestone validates the native executable build, not an installer.

## Out of scope

Runtime detection, PATH changes, runtime installation/switching, repository inspection, and all other DEM product features.

## Acceptance

M0 exit criteria are met based on the recorded local checks and the user's explicit confirmation of the native application launch. Accepted on 2026-11-17.
