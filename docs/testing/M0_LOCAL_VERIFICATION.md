# M0 — Local Windows Verification

**Purpose:** Verify the Tauri + React + TypeScript + Rust foundation on the target Windows workstation.

## 1. Sync the branch

Run from the existing repository:

```powershell
git fetch origin
git switch feature/m0-foundation
git pull --ff-only origin feature/m0-foundation
git status --short --branch
```

Expected branch: `feature/m0-foundation`. Review any local changes before proceeding; do not discard unrelated work.

## 2. Select the project Node.js version

```powershell
nodeuse 22
node --version
npm --version
```

Expected: Node.js 22.23.4 or newer within the Node 22 line, and npm 10.9.0 or newer. The project declares Node.js >=22.12.0 and npm >=10.9.0.

## 3. Install JavaScript dependencies

```powershell
npm install
```

This first install creates `package-lock.json`. Keep the generated lockfile; it must be committed with the M0 source. Subsequent clean installs should use `npm ci`.

## 4. Frontend checks

Run each command and record its result:

```powershell
npm run lint
npm run typecheck
npm test
npm run build
```

Expected:
- ESLint exits successfully.
- TypeScript reports no errors.
- Vitest reports one passing foundation smoke test without the previous config-loader warning.
- Vite produces the frontend bundle in `dist/`.

## 5. Rust checks and lockfile

```powershell
cargo generate-lockfile --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
```

Expected: formatting, test, and Clippy commands exit successfully. The first Cargo dependency resolution creates `src-tauri/Cargo.lock`; commit this lockfile.

## 6. Run and build the desktop app

A provisional development-only Windows icon is included at `src-tauri/icons/icon.ico` for Tauri's Windows resource generation. It is not final product branding. Installer bundling remains disabled.

```powershell
npm run tauri dev
```

Expected: a native DevEnv Manager window opens and displays the foundation screen. Close the app normally.

Then run:

```powershell
npm run tauri build
```

Expected: the Windows application executable is built successfully. Installer bundling is intentionally disabled for M0 pending a separate packaging decision.

## 7. Review and publish lockfiles

```powershell
git status --short
git diff --check
git diff -- package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock
```

Confirm that no secrets, machine-specific paths, or unrelated files are included. Commit the two generated lockfiles and push the branch.

## 8. Evidence to report

- Node.js and npm versions.
- Results of all frontend and Rust checks.
- Confirmation that `npm run tauri dev` opened the application.
- Result of `npm run tauri build`.
- Screenshot of the application window.
- Commit SHA containing `package-lock.json` and `src-tauri/Cargo.lock`.

Do not include personal identifiers, secrets, or unrelated machine details in evidence.
