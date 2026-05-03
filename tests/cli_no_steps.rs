use assert_cmd::Command;
use std::fs;

#[test]
fn cli_no_code_blocks_returns_error() {
    let fixture = "tests/fixtures/empty_plan.md";
    fs::write(fixture, "# empty\nno code here\n").unwrap();
    let mut cmd = Command::cargo_bin("runa").unwrap();
    cmd.arg("--plan").arg(fixture);
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("no steps found"));
}
