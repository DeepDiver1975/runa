use runa::terminal::Terminal;

#[test]
fn terminal_handles_large_output_without_deadlock() {
    let mut term = Terminal::new().expect("terminal");
    let cmd = "for i in $(seq 1 1000); do echo line$i; done";
    let out = term.run_command(cmd).expect("run");
    assert!(out.stdout.contains("line1000"));
}
