# M5 — Dependency & Configuration Inspector

**Status:** Planned; implementation not started | **Dependencies:** M2 inventory and repository metadata model

## Scope
Inspect agreed project metadata, extract runtime/tool requirements, compare requirements with local inventory, and report satisfied/missing/incompatible/unknown.

## Candidate metadata (not yet committed support matrix)
- Node.js: `package.json`, `.nvmrc`
- PHP: `composer.json`
- Rust: `Cargo.toml`, `rust-toolchain.toml`
- Java: `pom.xml`, `build.gradle`

## Acceptance criteria
- Parsing is read-only and safe for malformed/untrusted files.
- Findings identify source file and extracted requirement.
- Unsupported formats are not reported as compatible.
- Missing, incompatible, satisfied, and indeterminate states are distinct.
