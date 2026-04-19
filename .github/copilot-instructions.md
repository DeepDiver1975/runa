# .github/copilot-instructions.md — Runa

Purpose
- Help Copilot-like assistants navigate and operate in this repository (Runa — a keyboard-first plan execution TUI).

Quick repository facts
- Primary project described in repository docs (CLAUDE.md): a Rust-based TUI that parses Markdown plans into executable "steps" and runs them in an integrated PTY.
- Current working tree contains docs; source code layout expected under `Cargo.toml` and `src/` (see CLAUDE.md).

Build / test / lint (detected)
- No explicit build/test/lint scripts or tool configs were detected in the repository snapshot.

Inferred / common commands (project appears Rust-based — verify locally)
- Build: cargo build
- Run: cargo run
- Test full suite: cargo test
- Run a single unit test: cargo test <test_name>  (e.g. cargo test parse_markdown)
- Lint: cargo clippy
- Format check: cargo fmt -- --check
Note: These are inferred from CLAUDE.md (which references Cargo.toml). Confirm by inspecting repository for Cargo.toml and CI configs.

High-level architecture (summary)
- State-driven TUI: AppState is the single source of truth; UI components are stateless and read from AppState.
- Workspace contains the active Plan and Steps; a Step = {title, description, code, status}.
- Plan parsing: Markdown files parsed (pulldown-cmark) → code blocks → Step list.
- Execution/Terminal: integrated PTY (portable-pty) + ANSI parsing (vte); outputs buffered and shown in OutputPanel.
- UI: TopBar, Sidebar, PlanPanel (StepList), OutputPanel (TerminalView + LinkList), StatusBar.
- Event flow: Input → Command → State update → Render. Key commands: j/k navigation, Ctrl+Enter run step, R run-all, Alt+L link mode.

Key repository conventions
- Components are stateless; all mutable state lives on AppState/Workspace/TerminalState.
- Plan files are authoritative sources of executable steps: code fences in Markdown map to Step.code.
- Step lifecycle: Pending → Running → Done | Error. UI should reflect these statuses.
- Terminal integration assumes a PTY-backed execution model; outputs are parsed for links and stored in a scrollable buffer.
- Follow the component tree and event/command mapping patterns described in CLAUDE.md when adding features.

Where to look first
- CLAUDE.md — architecture, design decisions, keyboard mappings.
- repo root for Cargo.toml and src/ (not present in this snapshot).
- Any existing CI files (.github/workflows/) for project-specific commands.

AI-assistant / other config files found
- CLAUDE.md — important project doc; included in this guidance.
- No other AI-assistant config files (AGENTS.md, .cursorrules, .windsurfrules, CONVENTIONS.md, etc.) were detected in the snapshot.

Notes for Copilot sessions
- Prefer preserving the single-source-of-truth AppState pattern when suggesting changes.
- For changes touching execution or terminal code, verify PTY and ANSI handling with manual runs.
- When adding features that affect plans or steps, update CLAUDE.md to keep design and expectations aligned.
- GitHub Actions pinning convention: always pin action references to an explicit commit SHA and append the release/tag as a trailing comment. Example:
  uses: actions/checkout@de0fac2e4500dabe0009e67214ff5f5447ce83dd # v6.0.2
  Do NOT leave duplicate comment lines; keep the SHA and single version comment on the same line.

Concrete local commands (explicit)
- Build (debug): cargo build
- Build (release): cargo build --release
- Run: cargo run
- Run a single test by name: cargo test <test_name> -- --nocapture
  - For exact name matching: cargo test '^test_name$' -- --nocapture
- Run a single integration test file: cargo test --test my_integration_test
- Run with all features: cargo test --all-features
- Lint (Clippy): cargo clippy -- -D warnings
- Format check: cargo fmt -- --check

Example GitHub Actions workflow (Rust CI)
Create .github/workflows/ci.yml with this content to run fmt, clippy, build and tests on push and PRs:

```yaml
name: CI
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Cache cargo registry & target
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Format check
        run: cargo fmt -- --check

      - name: Lint (clippy)
        run: cargo clippy --workspace --all-targets -- -D warnings

      - name: Build (release)
        run: cargo build --release

      - name: Run tests
        run: cargo test --all --no-fail-fast
```

Notes
- The workflow is a recommended starting point; adjust `actions-rs/toolchain` usage or caching rules to match repository needs.
- If the project uses additional CI steps (e.g., packaging, publishing, Playwright tests), provide details and Copilot can add them.

If you want more/alternative CI (e.g., GitLab CI, CircleCI) or different Rust toolchain policies (MSRV pinning, matrix builds), say which and Copilot will extend this file.
