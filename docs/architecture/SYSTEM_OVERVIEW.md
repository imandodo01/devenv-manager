# System Overview

## Product boundary
DEM is a Windows desktop application. The frontend presents environment/repository information; the Rust/Tauri layer provides a narrow boundary for local OS inspection and operations.

## Conceptual areas
- UI shell and navigation.
- Environment/runtime inventory and detection adapters.
- Global runtime selection and environment-change workflow.
- Repository inventory and user-controlled status.
- Project requirement parsers.
- Diagnostics and verification records.

This is a conceptual decomposition, not a confirmed implementation architecture. Inspect the foundation before creating modules or abstractions.

## Trust boundary
Treat filesystem contents, project manifests, environment variables, process output, and external metadata as untrusted input. Validate paths/arguments, avoid shell-string construction when structured process arguments are available, and keep privileged operations narrow.

## OS interaction
Separate read-only detection from configuration-changing commands. Changes require preview, explicit approval, validation, and post-change verification. Machine-level changes require particular care and may need elevation.
