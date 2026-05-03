use runa::plan::parser::parse_plan;

#[test]
fn parse_empty_plan_should_return_empty_vec() {
    let md = "# Nothing here";
    let steps = parse_plan(md).expect("parse");
    assert!(steps.is_empty());
}

#[test]
fn parser_extracts_heading_as_title() {
    let md = "# Step One\n\n```bash\necho hi\n```";
    let steps = parse_plan(md).unwrap();
    assert_eq!(steps[0].title, "Step One");
}
