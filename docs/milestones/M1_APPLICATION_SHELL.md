# M1 — Application Shell

**Development:** Complete | **Test Preparation:** Complete | **Acceptance:** Passed — 2026-11-17 | **Dependency:** M0 accepted

## Goal

Replace the M0 foundation placeholder with the navigable desktop shell that will host subsequent DEM capabilities.

## Scope

- Persistent primary navigation and a page header.
- Overview dashboard with honest unavailable/empty states.
- Navigation destinations for Overview, Environment, Repositories, Requirements, and Worklist.
- Small reusable presentation primitives for page headings, status labels, and summary panels.
- Desktop-first layout with a compact-width fallback.
- Frontend-only navigation; no router dependency.

## Explicit non-goals

- Runtime/tool detection or environment scanning.
- Repository discovery, filesystem access, or manifest parsing.
- Tauri commands, shell/process execution, PATH changes, installation, or runtime switching.
- Persistence, settings, telemetry, or external network calls.
- Fake inventory counts, versions, health states, or repository records.

## Acceptance criteria

- [x] App renders a consistent shell with navigation and page content.
- [x] Each primary navigation item changes the active page and page heading.
- [x] Active navigation state is visually and semantically identifiable.
- [x] Overview distinguishes unavailable data from actual zero values.
- [x] Placeholder destinations explain future purpose without implying features exist.
- [x] Layout remains usable at the configured desktop size and narrower widths.
- [x] Shared UI primitives are reused without unnecessary framework dependencies.
- [x] Lint, typecheck, tests, and build pass; tests cover navigation state and shell rendering.
- [x] Windows local verification is completed and recorded.
- [x] No machine configuration, filesystem, or process interaction is introduced.

## Verification

Automated checks executed on the user's Windows workstation on 2026-11-17:

- `npm run lint` — passed.
- `npm run typecheck` — passed.
- `npm test` — passed, 1 test file and 5 tests.
- `npm run build` — passed; Vite production build completed.

Manual Windows verification reported by the user on 2026-11-17:

- Native application launched and displayed correctly.
- Overview appeared as the initial page.
- Environment, Repositories, Requirements, and Worklist navigation worked.
- Active navigation state changed correctly.
- No fabricated inventory data was displayed.
- Layout remained usable at normal and narrower window sizes.
- No PATH or other environment configuration changes were observed.

## Decisions and assumptions

- The initial navigation reflects the five capability areas in the Product Specification.
- A dark developer-tool visual language continues the M0 palette.
- Navigation state is in-memory only and resets to Overview on relaunch.
- Placeholder pages are intentionally informational; their data and workflows arrive in later milestones.
