use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_runs_first_step() {
    let mut cmd = Command::cargo_bin("runa").unwrap();
    cmd.arg("--plan").arg("tests/fixtures/simple_plan.md");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hello from step"));
}
