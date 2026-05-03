use runa::state::AppState;

#[test]
fn next_prev_step_wraps() {
    let mut s = AppState::new();
    // inject two steps
    s.steps.push(runa::state::Step::dummy("s1"));
    s.steps.push(runa::state::Step::dummy("s2"));
    s.active_step = 0;
    s.next_step();
    assert_eq!(s.active_step, 1);
    s.next_step();
    assert_eq!(s.active_step, 0);
}
