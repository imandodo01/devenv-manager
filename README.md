# DevEnv Manager (DEM)

Windows desktop developer-environment information and management center.

**Current milestone:** M0 — Foundation Setup. Product capabilities are not implemented yet.

## Technology

- Tauri 2
- React 19 + TypeScript 6
- Vite 8
- Tailwind CSS 4
- Rust (stable MSVC toolchain)
- npm

See [Dependency Baseline](docs/development/DEPENDENCY_BASELINE.md) for the initial version baseline and rationale.

## Prerequisites

- Windows 10 version 1803 or later / Windows 11 with WebView2 Runtime
- Microsoft C++ Build Tools with the Desktop development with C++ workload
- Rust stable with the `x86_64-pc-windows-msvc` target
- Node.js >=22.12.0 and npm >=10.9.0

## Setup

```powershell
npm install
```

The first install creates `package-lock.json`. Commit it. Use `npm ci` for subsequent clean dependency installs.

## Development

```powershell
npm run dev
```

Starts the frontend in the browser.

```powershell
npm run tauri dev
```

Starts the native desktop application.

## Quality checks

```powershell
npm run lint
npm run typecheck
npm test
npm run build
npm run format:check
```

Rust checks:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
```

## Windows build

```powershell
npm run tauri build
```

M0 builds the native application executable. Installer bundling remains disabled until the project makes an explicit packaging, icon, and signing decision.

## Scope boundaries

M0 establishes the application foundation only. Runtime detection, PATH changes, runtime installation/switching, repository inventory, and project requirement inspection are out of scope.

## Documentation

- [Product specification](docs/product/PRODUCT_SPECIFICATION.md)
- [Scope](docs/product/SCOPE.md)
- [Roadmap](docs/product/ROADMAP.md)
- [Architecture overview](docs/architecture/SYSTEM_OVERVIEW.md)
- [Architecture decisions](docs/architecture/ARCHITECTURE_DECISIONS.md)
- [Development workflow](docs/development/DEVELOPMENT_WORKFLOW.md)
- [Dependency baseline](docs/development/DEPENDENCY_BASELINE.md)
- [Test strategy](docs/testing/TEST_STRATEGY.md)
- [Local verification](docs/testing/LOCAL_VERIFICATION.md)
- [M0 local verification](docs/testing/M0_LOCAL_VERIFICATION.md)
- [Acceptance register](docs/testing/ACCEPTANCE_TESTS.md)
