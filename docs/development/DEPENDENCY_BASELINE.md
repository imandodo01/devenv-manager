# M0 Dependency Baseline

**Status:** Initial baseline; lockfiles and local build verification pending.

## Toolchain

| Tool | Baseline | Reason |
|---|---:|---|
| Node.js | 22.23.4 | Meets Vite 8 and Vitest 5 requirements |
| npm | 10.9.9 | Lockfile and package installation workflow |
| Rust | Stable MSVC toolchain | Native Windows desktop backend |
| Tauri | 2.12.5 (JS API and CLI); Rust crate 2.x | Current stable Tauri 2 line |
| React / React DOM | 19.3.1 | Current stable React line |
| TypeScript | 6.0.3 | Strict typed frontend |
| Vite | 8.3.8 | Frontend dev server and production build |
| Tailwind CSS | 4.3.5 | Vite plugin integration |
| Vitest | 5.1.0 | Frontend test runner |
| ESLint | 10.15.0 | Frontend linting |
| typescript-eslint | 8.73.0 | TypeScript lint rules |
| Prettier | 3.10.1 | Consistent formatting |

Versions are pinned in `package.json`. `Cargo.toml` uses the Tauri 2 semver line; `Cargo.lock` must be generated and committed after dependency resolution.

## Compatibility decision

The project baseline is Node.js 22.12 or newer. Vitest 5 requires Node.js 22.12 or newer, while Vite 8 requires Node.js 20.19 or newer. Node.js 22.23.4 is already available on the maintainer workstation and is selected to satisfy both.

## Scope decisions

- The app uses the official Tauri 2 architecture with React, TypeScript, Vite, and Tailwind CSS.
- Tailwind CSS 4 is integrated through its official Vite plugin.
- No runtime detection, PATH mutation, repository inspection, or environment-management behavior is included in M0.
- Tauri bundling remains disabled. A provisional development-only Windows icon is included at `src-tauri/icons/icon.ico` because Tauri's Windows resource generation requires an ICO even when installer bundling is disabled. This icon is not final branding; the final cross-platform icon set, installer format, signing, and release channel remain open decisions.
- The Tauri capability grants only `core:default`; no filesystem, shell, process, or other plugin permissions are introduced.
