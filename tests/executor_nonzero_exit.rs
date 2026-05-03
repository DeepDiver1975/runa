use runa::core::executor::Executor;
use runa::plan::parser::parse_plan;

#[test]
fn executor_nonzero_exit_marks_error() {
    // Use shell to exit non-zero
    let md = "```bash\nsh -c 'exit 2'\n```";
    let steps = parse_plan(md).unwrap();
    let mut exec = Executor::new().unwrap();
    let res = exec.run(&steps[0]).unwrap();
    assert_eq!(res.status, runa::core::executor::StepStatus::Error);
}
