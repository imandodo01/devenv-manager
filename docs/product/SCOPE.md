# Product Scope and Boundaries

## In scope — initial direction
- Windows developer-environment inventory.
- Runtime/tool detection with version and path information.
- Global runtime selection and controlled switching, subject to safety design.
- Repository discovery or registration.
- Reading recognized project metadata for runtime/tool requirements.
- Comparing repository requirements with local capabilities.
- User-managed repository status and setup worklist.
- Diagnostics, verification guidance, and traceable change records.

## Out of scope
- Editing source code or replacing an IDE.
- Git operations or PR management as product features.
- General-purpose package management.
- Container orchestration or VM management.
- Automatic project setup/repository mutation without separately approved scope.
- Silent Machine PATH or system configuration changes.

## Scope control
Ideas remain exploratory until explicitly accepted. Before adding capability, document the user problem, outcome, safety implications, dependencies, and first-release impact.
