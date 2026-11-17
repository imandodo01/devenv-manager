# Local Windows Verification Guide

This is the common structure; every PR must add exact ticket-specific commands and expected results.

## Before testing
- Confirm branch and PR.
- Pull latest branch commit.
- Record Windows version and relevant tool versions.
- Avoid production credentials and sensitive data.
- Keep a known-good terminal open when testing environment changes.

## M0 baseline checks (confirm scripts after bootstrap)
- `node --version`
- `npm --version`
- `rustc --version`
- `cargo --version`
- Run the repository-documented dependency installation command.
- Run `npm run` to inspect actual available scripts.
- Run documented lint, typecheck, test, and build scripts.
- `cargo test`
- Run documented Tauri Windows build command.

Do not assume script names or prerequisites until M0 files and versions are inspected.

## Environment-changing feature checks
Capture before-state of relevant values (redact sensitive data); confirm Process/User/Machine scope; inspect proposed diff; verify explicit approval and permission behavior; inspect result in a new process; check existing-process inheritance; test failure and rollback; confirm unrelated PATH entries remain unchanged. Use controlled test conditions.

## Evidence and acceptance
Attach concise logs/screenshots with unnecessary personal/system details redacted. Record pass/fail and deviations. Local testing is accepted only when the user confirms steps and expected results. Remote builds are not a substitute.
