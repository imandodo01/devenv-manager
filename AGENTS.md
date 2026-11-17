# DEM Agent Instructions

These instructions specialize the shared AI Engineering Standard for this repository.

## Before work
1. Read the current `https://github.com/imandodo01/ai-engineering-standard` `AGENTS.md` and relevant policies.
2. Inspect repository instructions, docs, structure, branch, and Git state.
3. Read the relevant Product Specification and ADRs.
4. Confirm ticket scope and acceptance criteria. Exploratory ideas are not approved scope by default.

## Project constraints
- Initial target: Windows desktop; stack direction: Tauri 2, React, TypeScript, Rust, Tailwind CSS, npm, Cargo.
- M0 is foundation preparation only; do not implement DEM product features during M0.
- DEM is a developer-environment information and management center, not an IDE, Git client, package manager, Docker manager, or general project manager.
- Separate read-only inspection from environment modification.
- Never change Machine PATH automatically. Preview and obtain explicit approval before system-level changes.
- Do not perform destructive operations or modify user/system configuration without explicit approval.
- Remote checks do not prove local Windows behavior.

## Workflow
- Use focused ticket branches and PRs for independently testable changes.
- Keep PRs small; add automated tests and explicit Windows local verification steps.
- Wait for user verification and acceptance before treating a ticket as accepted/merged.
- Record decisions, test outcomes, evidence, and unresolved issues as they occur.
- Preserve unrelated work; never claim checks that were not run.

## Completion report
Report summary, changed areas, exact checks/results, checks not run, user verification still required, Git/PR details, and remaining risks.
