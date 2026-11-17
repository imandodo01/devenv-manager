# Contributing

## Before starting
- Read repository `AGENTS.md`, relevant Product Specification, scope, ADRs, and shared AI Engineering Standard.
- Inspect branch and Git state.
- Confirm issue acceptance criteria and dependencies.
- Keep exploratory ideas separate from approved scope.

## Branch and commit
Use focused branches such as `feature/<ticket>`, `fix/<ticket>`, and `docs/<topic>`. Use Conventional Commits unless project convention exists: `feat(scope):`, `fix(scope):`, `test(scope):`, `docs(scope):`, `chore(scope):`.

## Quality
Prefer the smallest maintainable change. Add meaningful automated tests and update docs for setup/configuration/behavior changes. Avoid unrelated formatting/dependency changes. Never commit secrets or unnecessary local machine identifiers.

## Verification report
Separate automated checks, static inspection, manual Windows checks, and checks not run. Never claim Windows validation unless it was actually performed in a verified Windows environment or by the user.
