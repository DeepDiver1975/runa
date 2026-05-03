use crate::plan::parser::Step as PlanStep;
use crate::terminal::{CommandOutput, Terminal};

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    Pending,
    Running,
    Done,
    Error,
}

pub struct ExecResult {
    pub status: StepStatus,
    pub output: String,
}

pub struct Executor {
    term: Terminal,
}

impl Executor {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Executor {
            term: Terminal::new()?,
        })
    }

    pub fn run(&mut self, step: &PlanStep) -> Result<ExecResult, std::io::Error> {
        let out: CommandOutput = self.term.run_command(step.code.as_str())?;
        let merged = if !out.stderr.is_empty() {
            format!("{}\nERR:\n{}", out.stdout, out.stderr)
        } else {
            out.stdout.clone()
        };
        let status = if out.status.success() {
            StepStatus::Done
        } else {
            StepStatus::Error
        };
        Ok(ExecResult {
            status,
            output: merged,
        })
    }
}
