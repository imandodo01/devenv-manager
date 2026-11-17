# DevEnv Manager — Product Specification

**Status:** Draft baseline | **Target:** Windows desktop | **Reviewed:** 2026-11-17

## Purpose
DEM is a local developer-environment information center. It helps developers understand installed tools/runtimes, their versions and locations, repository requirements, environment gaps, and the globally selected runtime.

## Direction currently agreed
- Windows desktop using Tauri 2, React, TypeScript, Rust, and Tailwind CSS.
- Global runtime selection is a core concept: it changes relevant global Windows environment configuration, not merely one terminal or repository.
- Existing processes may retain inherited environment values; newly launched processes generally observe updated values. The product must explain this distinction.
- Inventory installed runtimes/tools; discover or register repositories; inspect repository requirements; compare requirements with local capabilities.
- Repository records distinguish Existing, Planned, Pending Setup, and Archived. Planned does not automatically mean a task.
- DEM informs and helps users identify/provision missing capabilities without silently taking broad control of repositories or the machine.

## Initial capability areas
1. Global Runtime Manager — inspect installed runtimes and global selection; controlled switching.
2. Environment Inventory — detected tools, versions, executable locations, relevant environment information.
3. Repository Inventory — discover/register local repositories and maintain user-controlled status.
4. Requirement Inspector — inspect recognized project metadata and compare requirements with local capabilities.
5. Repository Worklist — user-managed planned/pending setup records, not automatic task generation.

These are capability areas, not a promise that every ecosystem/action ships in the first release.

## Safety requirements
- Never modify Machine PATH automatically.
- Separate inspection from modifying operations.
- Preview target, scope, permissions, and expected effect before changes.
- Require explicit approval for system-level or destructive changes.
- Validate executable paths and verify results; exit code zero alone is insufficient.
- Keep traceable records of configuration changes and verification.

## Non-goals
Not an IDE/code editor, Git client, general package manager, Docker/container orchestrator, or general project-management system.

## Technology direction
Tauri 2; React + TypeScript; Rust; Tailwind CSS; npm and Cargo; Windows desktop initially. Exact versions, supported Windows versions, packaging/signing, and runtime support matrix require confirmation.

## Open decisions — do not assume
- First-release runtime/ecosystem support and detection strategy.
- Repository discovery roots and registration behavior.
- Repository status persistence and worklist semantics.
- Whether DEM installs runtimes or links to trusted providers.
- Supported environment scopes (Process/User/Machine), permission model, rollback, and audit retention.
- Supported Windows versions, installer/update strategy, signing, and release channel.
- Telemetry/privacy policy; default should avoid transmitting local inventory.
- Final information architecture and accessibility requirements.

## Acceptance principle
Compilation alone is not completion. Features require acceptance criteria, automated checks, documented local verification, and user acceptance where applicable.

## Provenance
This draft records project direction already discussed. Details and open questions remain provisional until reconciled with any authoritative existing Product Specification and reviewed by the user. Do not silently convert assumptions into requirements.
