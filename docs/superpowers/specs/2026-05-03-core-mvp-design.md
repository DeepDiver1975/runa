# Core MVP design — Runa

Summary
- Goal: Implement the minimal core loop so a user can load a Markdown plan, select a step, run it in an integrated PTY, and view output. Keep interfaces small and testable.

Scope
- PTY integration + output buffer (portable-pty + ANSI parsing)
- Markdown plan parser → Vec<Step>
- Workspace/State model (Plan, Steps, active_step, TerminalState)
- CLI entry to load plan and run one step
- Unit tests for parser; integration test for single-step run

Architecture & Components
- PlanParser: parse markdown (pulldown-cmark) → Step {title, description, code}
- Terminal/PTY module: spawn command, stream stdout/stderr into TerminalState buffer; expose write/close
- Workspace: holds current_plan, steps, active_step, output
- Executor: accepts Step, runs its code in a PTY, updates Step.status
- UI/CLI shim: minimal CLI to load plan and run a step (for now headless; prints buffer)

Data flow
Plan file -> PlanParser -> Workspace.steps -> Executor -> PTY -> TerminalState.buffer -> CLI prints

Error handling
- Parser: return Result<Vec<Step>, ParseError>
- PTY/Executor: surface exit code and stderr; mark Step.status = Error on non-zero exit
- Timeouts: Executor supports configurable timeout per step (default none)

Testing
- Unit: PlanParser extracts code blocks and titles
- Integration: spawn a step that echoes and verify buffer contents and status

Success criteria
- cargo test passes for new tests
- cargo run -- load.md && run-step prints expected output and marks step Done/Error

Files to add
- src/terminal/mod.rs, src/plan/parser.rs, src/core/workspace.rs, tests/parser.rs, tests/integration_run.rs


