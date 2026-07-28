use super::FrameTree;

#[test]
fn adds_nested_frames_and_traverses_children() {
    let mut tree = FrameTree::new("https://root.test");
    let child = tree
        .add_child(tree.root_id(), "https://child.test", "child")
        .unwrap();
    let grand = tree
        .add_child(child, "https://grand.test", "grand")
        .unwrap();

    assert_eq!(tree.children_of(tree.root_id())[0].id(), child);
    assert_eq!(tree.children_of(child)[0].id(), grand);
    assert_eq!(tree.frame(grand).unwrap().name(), "grand");
}

#[test]
fn resolves_parent_and_top_frames() {
    let mut tree = FrameTree::new("https://root.test");
    let child = tree
        .add_child(tree.root_id(), "https://child.test", "")
        .unwrap();
    let grand = tree.add_child(child, "https://grand.test", "").unwrap();

    assert_eq!(tree.parent_of(child).unwrap().id(), tree.root_id());
    assert_eq!(tree.parent_of(grand).unwrap().id(), child);
    assert_eq!(tree.top_for(grand).unwrap().id(), tree.root_id());
}

#[test]
fn keeps_ids_stable_when_metadata_changes() {
    let mut tree = FrameTree::new("https://root.test");
    let child = tree
        .add_child(tree.root_id(), "about:blank", "old")
        .unwrap();

    tree.set_url(child, "https://next.test").unwrap();
    tree.set_name(child, "new").unwrap();

    let frame = tree.frame(child).unwrap();
    assert_eq!(frame.id(), child);
    assert_eq!(frame.url(), "https://next.test");
    assert_eq!(frame.name(), "new");
}
