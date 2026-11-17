# M4 — Runtime Switching

**Status:** Planned; implementation not started | **Dependencies:** M2 inventory, M3 metadata, approved environment-change design

## Scope
Select global runtime; validate executable and scope; preview proposed changes; handle permissions/approval; apply only approved changes; verify and record outcome.

## Mandatory safety acceptance
- No Machine PATH change without explicit approval.
- Preserve unrelated PATH entries.
- Process/User/Machine scope is explicit.
- Reject invalid/missing executable paths.
- Report permission and partial failures.
- Verify using an appropriate new process and explain process inheritance.
- Document and test rollback/recovery.

## Test preparation
Design non-destructive tests first. Real Windows changes require a controlled test plan and user confirmation before execution.
