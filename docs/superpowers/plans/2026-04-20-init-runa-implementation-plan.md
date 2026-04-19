# Init Runa Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a minimal, testable Rust scaffold matching CLAUDE.md layout and verify build, formatting, linting, and tests in CI.

**Architecture:** Single-crate Rust application `runa` with focused modules: app, core, ui, terminal, plan, util. Each module owns a small surface area and exposes an `init()` to demonstrate integration. Tests will live under `tests/` and module unit tests.

**Tech Stack:** Rust (stable), cargo, GitHub Actions (CI), clippy, rustfmt

---

### Task 1: Ensure crate builds and prints expected startup message

**Files:**
- Create: `tests/startup.rs`
- Modify: `Cargo.toml` (add dev-dependencies)

- [ ] **Step 1: Add integration test to assert startup output**

```rust
// tests/startup.rs
use assert_cmd::Command;

#[test]
fn prints_startup_message() {
    let mut cmd = Command::cargo_bin("runa").unwrap();
    cmd.assert().success().stdout(predicates::str::contains("Runa scaffold initialized."));
}
```

- [ ] **Step 2: Add dev-dependencies for test harness**

Modify `Cargo.toml` (add under [dev-dependencies]):

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "2"
```

- [ ] **Step 3: Run the test**

Run:

```
cargo test --test startup -- --nocapture
```

Expected: PASS once dev-deps added.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml tests/startup.rs
git commit -m "test: add startup integration test\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>" --no-gpg-sign
```

### Task 2: Enforce rustfmt and clippy in CI

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Ensure CI steps fail on issues**

Update the workflow to remove `|| true` from fmt and clippy steps so they fail the job when errors present.

- [ ] **Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: enforce rustfmt and clippy checks\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>" --no-gpg-sign
```

### Task 3: Add core models and unit tests

**Files:**
- Create: `src/core/models.rs`
- Modify: `src/core/mod.rs`
- Create: `src/core/tests.rs`

- [ ] **Step 1: Implement Step and StepStatus in core/models.rs**

```rust
// src/core/models.rs
#[derive(Debug, PartialEq, Eq)]
pub enum StepStatus { Pending, Running, Done, Error }

#[derive(Debug)]
pub struct Step {
    pub title: String,
    pub description: String,
    pub code: String,
    pub status: StepStatus,
}

impl Step {
    pub fn new(title: &str, code: &str) -> Self {
        Self { title: title.to_string(), description: String::new(), code: code.to_string(), status: StepStatus::Pending }
    }
}
```

- [ ] **Step 2: Wire mod.rs to export models**

```rust
// src/core/mod.rs
pub mod models;

pub fn init() {
    println!("init: core");
}
```

- [ ] **Step 3: Add unit tests**

```rust
// src/core/tests.rs
#[cfg(test)]
mod tests {
    use crate::core::models::{Step, StepStatus};

    #[test]
    fn step_new_has_pending_status() {
        let s = Step::new("t", "echo hi");
        assert_eq!(s.status, StepStatus::Pending);
    }
}
```

- [ ] **Step 4: Run cargo test and commit**

```bash
cargo test
git add src/core
git commit -m "test(core): add Step model and unit tests\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>" --no-gpg-sign
```

### Task 4: Add simple plan parsing with test

**Files:**
- Modify: `src/plan/mod.rs`
- Create: `src/plan/tests.rs`

- [ ] **Step 1: Add a simple markdown code-block extractor**

```rust
// src/plan/mod.rs
pub fn extract_code_blocks(md: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut in_code = false;
    let mut buf = String::new();
    for line in md.lines() {
        if line.trim_start().starts_with("```") {
            if in_code { blocks.push(buf.clone()); buf.clear(); }
            in_code = !in_code;
            continue;
        }
        if in_code { buf.push_str(line); buf.push('\n'); }
    }
    blocks
}

pub fn init() { println!("init: plan"); }
```

- [ ] **Step 2: Add unit test**

```rust
// src/plan/tests.rs
#[cfg(test)]
mod tests {
    use crate::plan::extract_code_blocks;

    #[test]
    fn extracts_single_block() {
        let md = "# Title\n\n```bash\necho hi\n```";
        let blocks = extract_code_blocks(md);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].trim(), "echo hi");
    }
}
```

- [ ] **Step 3: Run cargo test and commit**

```bash
cargo test
git add src/plan
git commit -m "feat(plan): add markdown code-block extractor and tests\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>" --no-gpg-sign
```

### Task 5: Developer docs

**Files:**
- Modify: `README.md`
- Create: `docs/developer/README.md`

- [ ] **Step 1: Add commands and testing notes to README**

Append to README.md:

```
## Development

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`
```

- [ ] **Step 2: Commit**

```bash
git add README.md docs/developer/README.md
git commit -m "docs: add developer notes\n\nCo-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>" --no-gpg-sign
```

---

## Self-review

1. Spec coverage: The plan implements scaffold, core models, plan parsing, tests, and CI enforcement requested in the spec file.
2. Placeholder scan: No placeholders used.
3. Type consistency: Names chosen match the scaffold (Step, StepStatus, extract_code_blocks).

Plan saved to: $PLAN_PATH

Execution options: Subagent-Driven (recommended) or Inline Execution. Which approach?
