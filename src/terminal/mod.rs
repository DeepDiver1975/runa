use std::process::{Command, Stdio};
use std::io::Read;

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Terminal {})
    }

    pub fn run_command(&mut self, cmd: &str) -> Result<String, std::io::Error> {
        // Run through shell for simplicity
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", cmd])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
        } else {
            Command::new("sh")
                .args(["-c", cmd])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
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
            if !buf.is_empty() {
                out.push_str("\nERR:\n");
                out.push_str(&buf);
            }
        }
        let _ = child.wait();
        Ok(out)
    }
}
