# Development Workflow

## Operating model
- GitHub is source of truth for code, issues, branches, PRs, and review decisions.
- Development, documentation, review, and automated checks can proceed from any suitable workstation.
- Windows-specific local verification and user acceptance take place on the user's home machine.
- Remote checks do not replace local Windows acceptance.

## Ticket lifecycle
1. Select one ticket and confirm scope/dependencies.
2. Prepare acceptance criteria and verification plan before implementation.
3. Implement on a ticket-specific branch.
4. Add/update automated tests and docs.
5. Run relevant available checks and report exact results.
6. Open a focused PR with local verification instructions.
7. User tests the branch on the home Windows machine.
8. User reports evidence, failures, and feedback.
9. Fix and re-verify as needed.
10. User approves/merges; record acceptance and close ticket.

## Parallel preparation
While a current PR is being tested, prepare subsequent technical investigation, dependency analysis, interface proposals, acceptance criteria, automated tests, and manual verification instructions. Mark assumptions and interface dependencies; do not implement against unmerged unstable contracts.

## PR requirements
Link the issue; state scope/non-goals; summarize implementation and automated tests; provide exact local verification commands and expected results; identify Windows-only checks/evidence; list known limitations and checks not run. Do not merge before user acceptance.

Track Development, Test Preparation, and Acceptance separately.
