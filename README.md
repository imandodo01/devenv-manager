# DevEnv Manager (DEM)

Windows desktop developer-environment information and management center.

**Status:** Foundation preparation. Product documents are working baselines, not evidence of implemented functionality.

## Current state
- M0 foundation: pending local preparation and verification.
- Feature implementation: not started.
- Product specification: draft; see [Product Specification](docs/product/PRODUCT_SPECIFICATION.md).

## Documentation
- [Product specification](docs/product/PRODUCT_SPECIFICATION.md)
- [Scope](docs/product/SCOPE.md)
- [Roadmap](docs/product/ROADMAP.md)
- [Architecture overview](docs/architecture/SYSTEM_OVERVIEW.md)
- [Architecture decisions](docs/architecture/ARCHITECTURE_DECISIONS.md)
- [Development workflow](docs/development/DEVELOPMENT_WORKFLOW.md)
- [Contributing](docs/development/CONTRIBUTING.md)
- [Test strategy](docs/testing/TEST_STRATEGY.md)
- [Local verification](docs/testing/LOCAL_VERIFICATION.md)
- [Acceptance register](docs/testing/ACCEPTANCE_TESTS.md)
- [Milestones](docs/milestones/)

## Technology direction
Tauri 2, React, TypeScript, Rust, Tailwind CSS, npm, and Cargo. Windows is the initial target. Exact versions and commands must be confirmed during M0.

## Working model
Development, documentation, review, and automated checks can proceed remotely. Windows-specific verification and user acceptance take place on the user's home machine. Automated checks do not replace local acceptance.
