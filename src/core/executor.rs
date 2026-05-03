use crate::terminal::Terminal;
use crate::plan::parser::Step as PlanStep;

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
        let output = self.term.run_command(step.code.as_str())?;
        let status = if output.contains("ERR:") {
            StepStatus::Error
        } else {
            StepStatus::Done
        };
        Ok(ExecResult { status, output })
    }
}
