# Test Strategy

Derive tests from acceptance criteria and risk. Test pure logic without Windows dependencies where possible. Use Rust unit tests for backend logic and frontend unit/component tests once M0 establishes the stack. Add integration tests for Tauri command contracts where practical. Reserve real Windows environment behavior for manual verification.

## Layers
1. Static checks: formatting, lint, typecheck, static analysis.
2. Unit tests: parsers, version comparison, path validation, state transitions.
3. Integration tests: frontend/backend contracts and data flow.
4. Build checks: npm build, Cargo test/build, Windows bundle build.
5. Manual Windows verification: executable discovery, PATH behavior, permissions, switching, process inheritance, rollback, and absence of unintended system changes.
6. User acceptance: criteria checked with evidence.

For each PR, list exact commands, outcomes, environment, and checks not run. Distinguish agent-run automated checks from user-run Windows checks. A successful build alone is not complete verification.
