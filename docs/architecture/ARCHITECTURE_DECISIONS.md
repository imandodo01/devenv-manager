# Architecture Decision Records

## ADR-0001 — Windows desktop with Tauri 2
- **Status:** Accepted direction
- **Decision:** Tauri 2, React, TypeScript, Rust, and Tailwind CSS.
- **Consequences:** Rust/Tauri commands are the controlled boundary for local OS interaction; Windows behavior must be verified on Windows.
- **Open:** Exact versions, command structure, permissions, and packaging await M0 audit.

## ADR-0002 — Global runtime switching differs from project-scoped activation
- **Status:** Accepted product direction; implementation design pending
- **Decision:** Global runtime selection is a core product concept.
- **Consequences:** Design must account for environment scope, process inheritance, permissions, verification, and rollback. Existing processes may retain inherited values; newly launched processes generally observe updated values.
- **Safety:** Never change Machine PATH automatically; preview and explicit approval are required.

## ADR-0003 — DEM is an environment information center, not a project IDE
- **Status:** Accepted product boundary
- **Decision:** Support repository inventory and requirement inspection without becoming a full project manager.
- **Consequences:** Repository status/worklist is user-controlled and distinct from automatic task creation.
