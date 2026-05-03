use std::process::{Command, ExitStatus};

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Terminal {})
    }

    /// Runs a command through the system shell and returns exit status, stdout and stderr separately.
    pub fn run_command(&mut self, cmd: &str) -> Result<CommandOutput, std::io::Error> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", cmd]).output()?
        } else {
            Command::new("sh").args(["-c", cmd]).output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        Ok(CommandOutput {
            status: output.status,
            stdout,
            stderr,
        })
    }
}
