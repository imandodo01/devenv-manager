# M0 — Foundation Setup

**Owner:** User | **Development:** In progress — foundation scaffold prepared | **Acceptance:** Pending local Windows verification

## Scope

Foundation only; no DEM product features.

## Tasks

- [x] Confirm Node.js/npm, Rust/Cargo, and Windows build prerequisites.
- [x] Prepare Tauri 2 + React + TypeScript + Vite foundation.
- [x] Integrate the Rust backend entry point and minimal capability configuration.
- [x] Integrate Tailwind CSS 4 through the Vite plugin.
- [x] Prepare npm scripts for lint, typecheck, test, build, and formatting.
- [x] Prepare Cargo test and formatting workflow.
- [ ] Generate and commit npm and Cargo lockfiles after local dependency resolution.
- [ ] Verify frontend lint, typecheck, test, and build commands on Windows.
- [ ] Verify Cargo formatting, test, and Clippy commands on Windows.
- [ ] Launch the native app and verify the React screen renders.
- [ ] Build the Windows application executable.
- [ ] Record local evidence and user acceptance.

## Exit criteria

- Tauri app launches; React renders.
- Rust backend is integrated.
- Frontend lint/typecheck/test/build scripts work.
- Cargo tests run.
- Windows application build succeeds.
- Source and lockfiles are committed and pushed.

## Workstation prerequisite evidence

The maintainer workstation was checked before scaffold work:
- Node.js 20.20.2 and npm 10.8.2 were directly executable; Node 22.23.4 and npm 10.9.9 are installed and selected as the project baseline for Vitest 5.
- Rust 1.99.1, Cargo 1.99.1, and `stable-x86_64-pc-windows-msvc` were detected.
- Visual Studio 2019 Build Tools, MSVC 14.29.30154, Windows SDK 10.0.19041.0, and WebView2 Runtime 157.0.4341.46 were detected.
- A native C compile/link/execute smoke test completed with exit code 0 and printed `DEM M0 toolchain OK`.

## Out of scope

Runtime detection, PATH changes, runtime installation/switching, repository inspection, and all other DEM product features.

## Verification status

Workstation prerequisites are verified. Repository dependency installation, frontend checks, Cargo dependency resolution, Tauri app launch, and Windows app build remain pending local execution.
