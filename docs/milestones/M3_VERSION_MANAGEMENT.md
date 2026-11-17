# M3 — Runtime Information & Version Management

**Status:** Planned; implementation not started | **Dependency:** M2 detection model

## Scope
Present installed runtime versions and metadata; provide information about available versions from approved sources; explain selection options and installation status.

## Open boundary
Whether DEM installs runtimes itself or only provides trusted guidance/links is undecided. Do not implement installation until explicitly resolved.

## Acceptance criteria
- Version metadata identifies source and freshness.
- Installed and externally available versions are distinguished.
- No installation/system modification occurs without explicit approval and a defined safety flow.
