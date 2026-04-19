# 🚀 Runa — Product, Architecture & Implementation Guide

---

# 🧠 Product Definition

## Core Idea

**Runa is a keyboard-first execution workspace that turns structured plans (Markdown) into directly runnable development workflows.**

---

## Problem

Modern development workflows are fragmented:

* Terminal → execution
* Editor → reading plans
* Browser → links / results

This creates constant context switching:

> read → copy → switch → paste → run → repeat

---

## Solution

Runa compresses this loop into:

> **read → run → observe → repeat**

All inside one environment.

---

## Core Principles

1. **Execution-first**

   * Everything revolves around running steps

2. **Keyboard-first**

   * No mouse required

3. **Minimal surface area**

   * No unnecessary features

4. **Fast feedback loop**

   * Immediate execution + visible output

---

# 🔥 Core Features (v0.1)

## 1. Markdown Plan Execution

* Load `.md` file
* Detect code blocks
* Treat each as executable step

---

## 2. Step Navigation

* `j / k` → navigate steps
* active step highlighted

---

## 3. Step Execution

* `Ctrl + Enter` → run step
* executes in integrated terminal

---

## 4. Terminal Output

* live output stream
* stored in buffer
* visible in output panel

---

## 5. Link Detection

* parse output for:

  * URLs
  * file paths

* keyboard navigation:

  * `Alt + L` → enter link mode
  * `Enter` → open

---

## 6. Run All

* sequential execution of all steps

---

# ❌ Non-Goals (Critical)

Do NOT implement:

* full code editor
* plugin system
* browser embedding
* AI chat UI
* complex layouts
* theming system

---

# 🧱 Architecture Overview

## Core Rule

> **State is the single source of truth. UI is pure rendering.**

---

# 📁 Project Structure

```
runa/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app/
│   ├── core/
│   ├── ui/
│   ├── terminal/
│   ├── plan/
│   └── util/
```

---

# 🧠 Core State

## AppState

```rust
pub struct AppState {
    pub workspace: Workspace,
    pub ui: UIState,
}
```

---

## Workspace

```rust
pub struct Workspace {
    pub current_plan: Plan,
    pub steps: Vec<Step>,
    pub active_step: usize,
    pub output: TerminalState,
}
```

---

## Step

```rust
pub struct Step {
    pub title: String,
    pub description: String,
    pub code: String,
    pub status: StepStatus,
}
```

---

## StepStatus

```rust
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Error,
}
```

---

## UIState

```rust
pub struct UIState {
    pub focused_panel: Panel,
    pub link_mode: bool,
}
```

---

## Panel

```rust
pub enum Panel {
    Plan,
    Output,
    Sidebar,
}
```

---

# 🔁 Data Flow

```
Input → Command → State Update → Render
```

---

# 🎮 Event System

## Event

```rust
pub enum Event {
    KeyPress(Key),
}
```

---

## Command

```rust
pub enum Command {
    NextStep,
    PrevStep,
    RunStep,
    RunAll,
    EnterLinkMode,
}
```

---

## Mapping

```rust
fn map_event(event: Event) -> Option<Command>
```

---

## Update

```rust
fn update(state: &mut AppState, cmd: Command)
```

---

# 🧩 UI Component System

## Rule

> Components are stateless and read from AppState

---

## Component Trait

```rust
pub trait Component {
    fn render(&self, state: &AppState);
}
```

---

## Component Tree

```
App
├── TopBar
├── Sidebar
├── PlanPanel
│   ├── PlanHeader
│   └── StepList
│       └── StepItem
├── OutputPanel
│   ├── TerminalView
│   └── LinkList
└── StatusBar
```

---

# 📄 Plan Panel

## Responsibilities

* display markdown as structured steps
* highlight active step
* provide execution controls

---

## Step States

* Active → highlighted
* Done → success indicator
* Pending → neutral

---

# 🖥️ Output Panel

## Responsibilities

* show terminal output
* show detected links
* allow keyboard navigation

---

# ⚙️ Terminal System

## Components

* PTY (`portable-pty`)
* ANSI parser (`vte`)
* screen buffer

---

## Responsibilities

* spawn shell
* read output stream
* write commands
* update buffer

---

# 📜 Plan Parsing

## Input

Markdown file

---

## Process

* parse with `pulldown-cmark`
* extract code blocks
* map to `Step`

---

## Output

```rust
Vec<Step>
```

---

# 🎹 Keyboard Controls

## Navigation

* `j / k` → step navigation
* `Enter` → focus

---

## Execution

* `Ctrl + Enter` → run step
* `R` → run all

---

## Output

* `Alt + L` → link mode
* `Enter` → open link

---

## Global

* `Ctrl + K` → command palette
* `Esc` → exit modes

---

# 🎨 Design System

## Philosophy

> 90% neutral, 10% signal

---

## Colors

* background: dark
* accent: purple
* success: green
* error: red

---

## Typography

* UI: Inter
* Code: JetBrains Mono

---

# 📏 Layout

```
TopBar
Sidebar | Plan | Output
StatusBar
```

---

# 🚀 Milestones

---

## 🟢 Phase 1 — Core Loop (MVP)

* load markdown
* render steps
* execute commands
* show output

---

## 🟡 Phase 2 — Usability

* link detection
* better terminal buffer
* run-all logic

---

## 🔵 Phase 3 — Interaction

* link navigation
* improved keyboard flow

---

## 🟣 Phase 4 — Polish

* rendering improvements
* performance
* visual refinement

---

# ⚠️ Risks

* terminal complexity
* scope creep
* overengineering UI
* drifting into IDE territory

---

# 🧭 Guiding Principle

> **If it does not improve the execution loop, it does not belong in Runa**

---

# 🏁 Definition of Success

A user can:

1. open a markdown plan
2. navigate steps
3. execute each step
4. interact with output
5. never leave Runa

---

# 🔥 Final Positioning

Runa is not:

* a terminal
* an IDE
* a workspace

Runa is:

> **the fastest path from plan → running system**

---

# 🚀 Next Step for Development

Implement in order:

1. PTY + output buffer
2. Markdown → Step parsing
3. Step rendering
4. Execution binding
5. Keyboard navigation

---

Build fast. Keep it minimal. Stay focused.

---
