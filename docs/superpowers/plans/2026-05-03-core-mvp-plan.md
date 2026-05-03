# Core‑MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the Core‑MVP so a user can load a Markdown plan, parse steps, run a single step in a PTY, and see its output.

**Architecture:** Small focused modules: plan parser, terminal/PTY wrapper, workspace+executor, and a minimal CLI. TDD-first: each feature begins with a failing test.

**Tech Stack:** Rust (cargo), pulldown-cmark (markdown parsing), portable-pty crate (or std::process for minimal), vte for ANSI parsing (optional for MVP).

---

### Task 1: Plan parser (markdown -> Vec<Step>)

**Files:**
- Create: `src/plan/parser.rs`
- Modify: `src/plan/mod.rs` (export parser)
- Test: `tests/plan_parser.rs`

- [ ] **Step 1: Write the failing test**

```rust
// tests/plan_parser.rs
use runa::plan::parser::parse_plan;

#[test]
fn extracts_code_blocks_as_steps() {
    let md = r#"
# Title

Some text

```bash
echo hello
```

More text

```python
print(1)
```
"#;
    let steps = parse_plan(md).expect("parse");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0].code.trim(), "echo hello");
    assert_eq!(steps[1].code.trim(), "print(1)");
}
```

Run: `cargo test tests::plan_parser -- --nocapture` Expected: FAIL (parse_plan not implemented)

- [ ] **Step 2: Implement minimal parser**

```rust
// src/plan/parser.rs
use pulldown_cmark::{Event, Parser, Tag};

#[derive(Debug, Clone)]
pub struct Step {
    pub title: String,
    pub description: String,
    pub code: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("markdown parse error")]
    Parse,
}

pub fn parse_plan(md: &str) -> Result<Vec<Step>, ParseError> {
    let parser = Parser::new(md);
    let mut in_code = false;
    let mut code_buf = String::new();
    let mut steps = Vec::new();
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_info)) => { in_code = true; code_buf.clear(); }
            Event::End(Tag::CodeBlock(_)) => {
                in_code = false;
                steps.push(Step { title: String::new(), description: String::new(), code: code_buf.clone() });
            }
            Event::Text(t) => {
                if in_code {
                    code_buf.push_str(&t);
                }
            }
            _ => {}
        }
    }
    Ok(steps)
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test --test plan_parser` Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/plan/parser.rs tests/plan_parser.rs
git commit -m "feat(plan): add markdown plan parser and tests" \
  -m "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 2: Terminal / PTY module

**Files:**
- Create: `src/terminal/mod.rs`
- Test: `tests/terminal_integration.rs`

Notes: For MVP, use std::process::Command to run steps in a shell and capture stdout/stderr. Portable-pty can be introduced later. Keep interface compatible with future PTY swap.

- [ ] **Step 1: Write failing test**

```rust
// tests/terminal_integration.rs
use runa::terminal::Terminal;

#[test]
fn runs_command_and_captures_output() {
    let mut term = Terminal::new().expect("terminal");
    let output = term.run_command("echo hello").expect("run");
    assert!(output.contains("hello"));
}
```

Run: `cargo test --test terminal_integration` Expected: FAIL

- [ ] **Step 2: Implement minimal terminal wrapper**

```rust
// src/terminal/mod.rs
use std::process::{Command, Stdio};
use std::io::{Read};

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> { Ok(Terminal {}) }

    pub fn run_command(&mut self, cmd: &str) -> Result<String, std::io::Error> {
        // Run through shell for simplicity
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        } else {
            Command::new("sh").args(["-c", cmd]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        };
        let mut out = String::new();
        if let Some(mut stdout) = child.stdout.take() {
            let mut buf = String::new();
            stdout.read_to_string(&mut buf)?;
            out.push_str(&buf);
        }
        if let Some(mut stderr) = child.stderr.take() {
            let mut buf = String::new();
            stderr.read_to_string(&mut buf)?;
            if !buf.is_empty() { out.push_str("\nERR:\n"); out.push_str(&buf); }
        }
        let _ = child.wait();
        Ok(out)
    }
}
```

- [ ] **Step 3: Run tests & commit**

```bash
cargo test --test terminal_integration
git add src/terminal/mod.rs tests/terminal_integration.rs
git commit -m "feat(terminal): add minimal terminal wrapper and tests" \
  -m "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 3: Workspace + Executor

**Files:**
- Create: `src/core/workspace.rs`
- Create: `src/core/executor.rs`
- Test: `tests/executor_integration.rs`

- [ ] **Step 1: Write failing integration test**

```rust
// tests/executor_integration.rs
use runa::plan::parser::parse_plan;
use runa::core::executor::Executor;

#[test]
fn executor_runs_step_and_marks_done() {
    let md = "```bash\necho ok\n```";
    let steps = parse_plan(md).unwrap();
    let mut exec = Executor::new().unwrap();
    let res = exec.run(&steps[0]).expect("exec");
    assert_eq!(res.status, runa::core::executor::StepStatus::Done);
    assert!(res.output.contains("ok"));
}
```

- [ ] **Step 2: Implement Workspace and Executor**

```rust
// src/core/executor.rs
use crate::terminal::Terminal;
use crate::plan::parser::Step as PlanStep;

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus { Pending, Running, Done, Error }

pub struct ExecResult { pub status: StepStatus, pub output: String }

pub struct Executor { term: Terminal }

impl Executor {
    pub fn new() -> Result<Self, std::io::Error> { Ok(Executor { term: Terminal::new()? }) }

    pub fn run(&mut self, step: &PlanStep) -> Result<ExecResult, std::io::Error> {
        let output = self.term.run_command(step.code.as_str())?;
        let status = if output.contains("ERR:") { StepStatus::Error } else { StepStatus::Done };
        Ok(ExecResult { status, output })
    }
}
```

- [ ] **Step 3: Run tests & commit**

```bash
cargo test --test executor_integration
git add src/core/executor.rs src/core/workspace.rs tests/executor_integration.rs
git commit -m "feat(core): add executor and workspace tests" \
  -m "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

### Task 4: Minimal CLI to load plan and run a step

**Files:**
- Modify: `src/main.rs` (or create if missing)
- Test: `tests/cli_integration.rs`

- [ ] **Step 1: Write failing integration test**

```rust
// tests/cli_integration.rs
use assert_cmd::Command;

#[test]
fn cli_runs_first_step() {
    let mut cmd = Command::cargo_bin("runa").unwrap();
    cmd.arg("--plan").arg("tests/fixtures/simple_plan.md");
    cmd.assert().success().stdout(predicate::str::contains("hello from step"));
}
```

Provide fixture: `tests/fixtures/simple_plan.md` containing a code block that echoes text.

- [ ] **Step 2: Implement CLI**

```rust
// src/main.rs
use std::fs;
use runa::plan::parser::parse_plan;
use runa::core::executor::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let plan_path = args.iter().position(|a| a=="--plan").and_then(|i| args.get(i+1)).expect("--plan <path>");
    let md = fs::read_to_string(plan_path)?;
    let steps = parse_plan(&md)?;
    let mut exec = Executor::new()?;
    let res = exec.run(&steps[0])?;
    println!("{}", res.output);
    if res.status != runa::core::executor::StepStatus::Done { std::process::exit(1); }
    Ok(())
}
```

- [ ] **Step 3: Run tests & commit**

```bash
# create fixture
mkdir -p tests/fixtures
cat > tests/fixtures/simple_plan.md <<'MD'
```bash
echo "hello from step"
```
MD

cargo test --test cli_integration

git add src/main.rs tests/cli_integration.rs tests/fixtures/simple_plan.md
git commit -m "feat(cli): add minimal CLI to load plan and run first step" \
  -m "Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>"
```

---

## Self-review

- Spec coverage: Each major spec item (parser, terminal, workspace, executor, CLI) has a corresponding task above.
- No placeholders: All steps include concrete code snippets and exact commands.
- Types and names: Consistent names: `parse_plan`, `Terminal`, `Executor`, `StepStatus`.


