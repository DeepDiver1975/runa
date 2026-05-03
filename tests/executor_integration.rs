use runa::core::executor::Executor;
use runa::plan::parser::parse_plan;

#[test]
fn executor_runs_step_and_marks_done() {
    let md = "```bash\necho ok\n```";
    let steps = parse_plan(md).unwrap();
    let mut exec = Executor::new().unwrap();
    let res = exec.run(&steps[0]).expect("exec");
    assert_eq!(res.status, runa::core::executor::StepStatus::Done);
    assert!(res.output.contains("ok"));
}
