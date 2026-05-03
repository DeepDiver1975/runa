use std::process::Command;

#[test]
fn app_binary_builds() {
    // ensure project builds (CI will run more thorough checks)
    let status = Command::new("cargo").args(&["build"]).status().expect("cargo build failed");
    assert!(status.success());
}
