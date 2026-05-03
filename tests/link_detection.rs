use runa::terminal::detect_links;

#[test]
fn finds_url_in_output() {
    let s = "see http://example.com for details";
    let links = detect_links(s);
    assert_eq!(links.len(), 1);
    assert!(links[0].url.contains("example.com"));
}
