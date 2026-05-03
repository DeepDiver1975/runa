use runa::terminal::{MockTerminal, TerminalBackend};

#[tokio::test]
async fn mock_streams_output() {
    let mut m = MockTerminal::new();
    let id = m.spawn("echo hello").await.unwrap();
    // read a line produced by the mock
    let out = m.next_output_opt(id).await.unwrap();
    assert!(out.is_some());
    assert!(out.unwrap().contains("hello"));
}
