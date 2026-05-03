use runa::filetree::FileTree;

#[test]
fn expand_and_collapse_node() {
    let mut ft = FileTree::new(".");
    let root = ft.root.clone();
    ft.toggle_expand(&root);
    assert!(ft.is_expanded(&root));
    ft.toggle_expand(&root);
    assert!(!ft.is_expanded(&root));
}
