# M1 — Application Shell

**Development:** In progress | **Test Preparation:** Ready | **Acceptance:** Pending | **Dependency:** M0 accepted

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

- [ ] App renders a consistent shell with navigation and page content.
- [ ] Each primary navigation item changes the active page and page heading.
- [ ] Active navigation state is visually and semantically identifiable.
- [ ] Overview distinguishes unavailable data from actual zero values.
- [ ] Placeholder destinations explain future purpose without implying features exist.
- [ ] Layout remains usable at the configured desktop size and narrower widths.
- [ ] Shared UI primitives are reused without unnecessary framework dependencies.
- [ ] Lint, typecheck, tests, and build pass; tests cover navigation state and shell rendering.
- [ ] Windows local verification is completed and recorded.
- [ ] No machine configuration, filesystem, or process interaction is introduced.

## Verification

Automated: `npm run lint`, `npm run typecheck`, `npm test`, `npm run build`.

Manual Windows: run `npm run tauri dev`; inspect Overview; navigate to all destinations; verify active state; inspect normal and narrower widths; confirm no environment or system configuration changes occur.

## Decisions and assumptions

- The initial navigation reflects the five capability areas in the Product Specification.
- A dark developer-tool visual language continues the M0 palette.
- Navigation state is in-memory only and resets to Overview on relaunch.
- Placeholder pages are intentionally informational; their data and workflows arrive in later milestones.
