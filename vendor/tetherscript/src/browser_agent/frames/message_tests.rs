use crate::browser_agent::BrowserPage;

fn page_with_child(src: &str) -> BrowserPage {
    BrowserPage::from_html(
        "https://app.test/root",
        format!(r#"<iframe name="child" src="{src}"></iframe>"#),
    )
}

#[test]
fn frame_message_records_delivered_same_origin_metadata() {
    let mut page = page_with_child("https://app.test/child");
    let tree = page.frame_tree();
    let root = tree.root_id();
    let child = tree.children_of(root)[0].id();

    let message = page
        .post_frame_message(root, child, "ready", "https://app.test")
        .unwrap();

    assert_eq!(message.sequence, 0);
    assert_eq!(message.source, root);
    assert_eq!(message.target, child);
    assert_eq!(message.data, "ready");
    assert!(message.same_origin);
    assert!(message.allowed_by_policy);
    assert!(message.delivered);
    assert_eq!(page.frame_messages_for(child), vec![message]);
}

#[test]
fn frame_message_records_target_origin_mismatch() {
    let mut page = page_with_child("https://app.test/child");
    let tree = page.frame_tree();
    let child = tree.children_of(tree.root_id())[0].id();

    let message = page
        .post_frame_message(tree.root_id(), child, "blocked", "https://other.test")
        .unwrap();

    assert!(message.allowed_by_policy);
    assert!(!message.delivered);
    assert_eq!(page.frame_messages().len(), 1);
}
