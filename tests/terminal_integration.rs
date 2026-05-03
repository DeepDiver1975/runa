use runa::terminal::Terminal;

#[test]
fn runs_command_and_captures_output() {
    let mut term = Terminal::new().expect("terminal");
    let output = term.run_command("echo hello").expect("run");
    assert!(output.contains("hello"));
}
