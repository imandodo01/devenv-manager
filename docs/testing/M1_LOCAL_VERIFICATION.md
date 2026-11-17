# M1 — Local Windows Verification

## Before testing

```powershell
git fetch origin
git switch feature/m1-application-shell
git pull --ff-only origin feature/m1-application-shell
git status --short --branch
node --version
npm --version
```

Expected: clean working tree on `feature/m1-application-shell`, using the repository Node.js/npm baseline.

## Automated checks

```powershell
npm ci
npm run lint
npm run typecheck
npm test
npm run build
```

Expected: all commands exit successfully; Vitest covers shell rendering and navigation state.

## Native Windows review

```powershell
npm run tauri dev
```

Verify:
- Overview is the initial page.
- Environment, Repositories, Requirements, and Worklist navigation each opens the matching placeholder page.
- Active navigation state is clear.
- The Overview does not display invented inventory values.
- The shell remains usable at the normal window size and when resized narrower.
- No runtime scan, filesystem access, PATH change, or other machine configuration operation occurs.

Close the app normally after review.

## Evidence to report

- Results of each automated command.
- Confirmation that the native app launched.
- Navigation and narrow-window review result.
- Any layout defects or deviations.

Do not include secrets or unrelated workstation details.
