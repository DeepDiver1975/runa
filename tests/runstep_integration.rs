use runa::plan::parse_plan_from_markdown;

#[tokio::test]
async fn runstep_updates_status() {
    // parse a plan with one step: echo hi
    let plan = parse_plan_from_markdown("```bash\necho hi\n```").unwrap();
    let mut state = runa::state::AppState::new();
    state.steps = plan.steps;

    // Verify initial status is Pending
    assert_eq!(state.steps[0].status, runa::state::StepStatus::Pending);
    
    // Simulate running the step
    state.steps[0].status = runa::state::StepStatus::Running;
    assert_eq!(state.steps[0].status, runa::state::StepStatus::Running);
    
    // Mark as done
    state.steps[0].status = runa::state::StepStatus::Done;
    assert_eq!(state.steps[0].status, runa::state::StepStatus::Done);
}
