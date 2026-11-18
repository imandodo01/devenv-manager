# M2 — Local Windows Verification

**Branch:** `feature/m2-environment-inventory-implementation`  
**Purpose:** Verify the read-only Environment inventory on the target Windows machine. Do not use this feature to change PATH or install/remove tools.

## 1. Prepare

From the repository root, first confirm there are no unrelated working-tree changes. Then fetch and check out the PR branch:

```powershell
git status --short --branch
git fetch origin
git switch --track origin/feature/m2-environment-inventory-implementation
```

If the local branch already exists, use `git switch feature/m2-environment-inventory-implementation` followed by `git pull --ff-only origin feature/m2-environment-inventory-implementation`.

Then activate the project's Node version and record the toolchain:

```powershell
nodeuse 22
node --version
npm --version
rustc --version
cargo --version
```

Confirm the working tree has no unrelated changes before switching branches. Record the Windows version and the versions/paths of the supported tools that are installed.

## 2. Automated checks

```powershell
npm run lint
npm run typecheck
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
```

## 3. Run the desktop application

```powershell
npm run tauri dev
```

Open **Environment** from the sidebar. The page should show seven tool entries: Node.js, PHP, Java, Rust compiler, Cargo, Git, and Composer.

## 4. Verify real detections

For every installed tool, compare the displayed version and executable path with the independently resolved executable and version:

```powershell
Get-Command node.exe, php.exe, java.exe, rustc.exe, cargo.exe, git.exe -All -ErrorAction SilentlyContinue |
  Select-Object Name, Source
```

Run the matching fixed version command for each installed executable. For Composer, verify the displayed PHAR path and PHP runner; compare against:

```powershell
$composerPhar = "C:\path\to\composer.phar"
php.exe $composerPhar --no-plugins --no-scripts --version
```

For custom installations not listed by `Get-Command`, compare the exact executable path shown in DEM directly. Do not paste full PATH or unrelated environment-variable values into evidence.

## 5. Verify result states

- **Detected:** exactly one candidate is found and its version can be read.
- **Not found:** no candidate exists in the approved sources.
- **Multiple installations:** two or more candidates exist; no default is selected.
- **Inspection incomplete:** a command fails, times out, produces unrecognized output, or an approved source cannot be fully inspected.
- Use **Scan again** and confirm the UI refreshes without freezing.

If practical, temporarily test an environment with a missing tool and one with two installed versions. Do not rename, delete, move, or uninstall real tools just to manufacture a test case.

## 6. Confirm read-only behavior

Capture before/after values in the same PowerShell session:

```powershell
$beforePath = $env:Path
$beforeUserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$beforeMachinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
```

After scanning, run:

```powershell
[pscustomobject]@{
  ProcessPathUnchanged = ($env:Path -ceq $beforePath)
  UserPathUnchanged = ([Environment]::GetEnvironmentVariable("Path", "User") -ceq $beforeUserPath)
  MachinePathUnchanged = ([Environment]::GetEnvironmentVariable("Path", "Machine") -ceq $beforeMachinePath)
}
```

All three values must be `True`. Also confirm no installation, file, or environment configuration was changed.

## 7. Report

Record pass/fail for each supported tool, unexpected output, duplicate installations, and any custom install locations not discovered. “Not found” means not found in the approved bounded sources; it does not prove absence from the whole computer. Redact usernames or sensitive path segments before sharing screenshots/logs.
