# Iced UI Design for Runa — 2026-04-20

## Summary

Design for a native desktop UI using the Rust `iced` framework with the Tokio async runtime. Implements the screenshot's three-column layout (Sidebar | Plan | Output) with keyboard-first interactions, step execution via integrated PTY, and structured MVU architecture.

## Scope

- Target: Desktop app (cross-platform) using iced
- Core features: File tree sidebar, Plan panel (steps), Terminal output panel, keyboard navigation, run/skip steps, run-all
- Out of scope: Web-based UI, plugin system, AI chat UI

## Architecture

- Pattern: MVU (Model-View-Update) using `iced::Application` as the entry point. Messages drive state updates; side-effects executed via `Command::perform` (Tokio async tasks).
- Modules: app, ui (components), plan (parsing), terminal (PTY backend), filetree, util

## Components

- TopBar: title, current path, quick actions
- Sidebar: FileTree + Session list + controls
- PlanPanel: PlanHeader, StepList, StepItem
- OutputPanel: TerminalView (scrolling buffer) + LinkList
- StatusBar: short hints, mode indicators
- CommandPalette / Modal: command entry and search

## Data Flow

User action -> Message -> Update(Model) -> (optional) Effect/Command -> Async task completes -> TerminalOutput/Result Message -> Update(Model) -> View re-render

Execution example: RunStep -> spawn PTY (async) -> stream output -> TerminalOutput messages -> update TerminalSession.buffer and Step.status

## State structs (Rust sketches)

pub struct AppState {
    pub workspace: Workspace,
    pub ui: UIState,
}

pub struct Workspace {
    pub current_plan: Option<Plan>,
    pub steps: Vec<Step>,
    pub active_step: usize,
    pub terminal: TerminalState,
    pub file_tree: FileTree,
}

pub struct Plan {
    pub path: std::path::PathBuf,
    pub title: String,
    pub steps: Vec<Step>,
}

pub struct Step {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub status: StepStatus,
    pub meta: Option<StepMeta>,
}

pub enum StepStatus { Pending, Running, Done, Error }

pub struct StepMeta { pub language: String, pub shortcuts: Option<Shortcuts> }

pub struct UIState {
    pub focused_panel: Panel,
    pub link_mode: bool,
    pub command_palette_open: bool,
    pub selected_node: Option<std::path::PathBuf>,
}

pub struct TerminalState {
    pub sessions: std::collections::HashMap<SessionId, TerminalSession>,
    pub active_session: Option<SessionId>,
}

pub struct TerminalSession {
    pub id: SessionId,
    pub buffer: VecDeque<String>,
    pub running_cmd: Option<CommandInfo>,
    pub links: Vec<Link>,
}

pub struct FileTree { pub root: PathBuf, pub nodes: Vec<FileNode>, pub expanded: HashSet<PathBuf> }

## Message enum (examples)

pub enum Message {
    NextStep,
    PrevStep,
    RunStep(usize),
    SkipStep(usize),
    RunAll,
    TerminalOutput(SessionId, String),
    TerminalExited(SessionId, ExitStatus),
    OpenFile(PathBuf),
    ToggleLinkMode,
    // UI events
}

## Terminal backend notes

- Use the portable-pty crate as the primary PTY implementation (recommended) for cross-platform PTY support. Provide a small adapter to make it tokio-friendly where necessary.
- Run PTY readers on Tokio tasks; forward output via iced::Command::perform to produce TerminalOutput messages handled by the MVU update loop.
- Expose an async TerminalBackend trait with spawn, write, resize, and kill operations. Provide a MockTerminalBackend for unit and integration tests so UI logic can be verified without real processes.
- Keep PTY handling isolated behind the trait so UI code and tests remain decoupled from OS PTY behavior.

## Types & aliases (clarifications)

- type SessionId = uuid::Uuid;
- type ExitStatus = std::process::ExitStatus;

pub struct CommandInfo { pub cmd: String, pub start_time: std::time::Instant }

pub struct Link { pub range: (usize, usize), pub url: String }

pub struct FileNode { pub path: PathBuf, pub kind: FileKind, pub children_count: usize }

pub enum FileKind { File, Dir }

pub struct Shortcuts { pub run: Option<String>, pub skip: Option<String> }

These types are referenced above and help remove ambiguity.

## Testing

- Unit: plan parsing, step lifecycle transitions, filetree operations
- Integration: mock PTY to simulate output and verify UI state transitions
- UI: component snapshot tests where applicable

## Implementation plan (phases)

1. Scaffold iced app (AppState, Message enum) + tokio runtime support
2. Implement plan parser (pulldown-cmark) + tests
3. FileTree component + sidebar wiring
4. PlanPanel UI (list, step item, keyboard navigation)
5. Terminal backend (PTY abstraction) + TerminalView rendering
6. Hook RunStep -> PTY -> update Step.status and TerminalSession
7. Link detection + LinkList and opening behavior
8. Polish: theming, accessibility, tests, packaging

## Acceptance criteria

- App launches and shows Sidebar | Plan | Output layout
- Loading a markdown plan parses steps into the PlanPanel
- Running a step streams output into OutputPanel and updates Step.status
- Keyboard navigation (j/k, Ctrl+Enter) works for steps

## Next steps / TODOs

- Commit this spec to docs/superpowers/specs/2026-04-20-iced-ui-design.md
- Start implementation: create iced scaffold and parser (write-plan via writing-plans skill after user approves)

---

*Spec authored during brainstorming session. Awaiting user review before writing implementation plan.*
