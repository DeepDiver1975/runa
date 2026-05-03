use runa::plan::parse_plan_from_markdown;

#[test]
fn parses_code_blocks_into_steps() {
    let md = "# Plan\n\n```bash\necho hi\n```\n";
    let plan = parse_plan_from_markdown(md).expect("parse failed");
    assert_eq!(plan.steps.len(), 1);
    assert_eq!(plan.steps[0].code.trim(), "echo hi");
}
