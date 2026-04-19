# Init Rust project — Design Spec

Summary

Initialize the repository as a Rust workspace-like crate `runa` with the structure documented in CLAUDE.md. Provide minimal, well-documented stubs so contributors can iterate on features without merge conflicts.

Goals
- Provide a small, compilable scaffold
- Match CLAUDE.md file layout: src/{app,core,ui,terminal,plan,util}
- Add CI that verifies format, lint, build, and tests
- Commit spec and scaffolding to repository

Architecture & Components

- App: Application entry, AppState orchestration
- Core: Core domain models (AppState, Workspace, Step, StepStatus)
- UI: Rendering components (stateless widgets)
- Terminal: PTY and terminal integration
- Plan: Markdown plan parsing, Step extraction
- Util: Small helpers and shared utilities

Data Flow
Input → Plan parsing → Steps → Execution (Terminal) → Output buffer → UI

Error handling
- Modules should return Result where appropriate. For the scaffold, keep simple `init()` helpers that print status.

Testing & Verification
- Ensure code builds with `cargo build`
- Add unit tests later under `tests/` and `src/*` as needed
- CI workflow provided to catch formatting and lint issues

Files added
- Cargo.toml
- src/main.rs
- src/{app,core,ui,terminal,plan,util}/mod.rs (stubs)
- docs/superpowers/specs/2026-04-20-init-rust-project-design.md (this file)
- .github/workflows/ci.yml
- README.md, .gitignore

Approval
Spec written. Please review the spec file at: $SPEC_PATH. Reply when ready to proceed to the implementation plan.
