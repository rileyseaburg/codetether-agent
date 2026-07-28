use crate::browser_agent::BrowserPage;

#[test]
fn cross_origin_message_uses_page_security_policy() {
    let mut page = BrowserPage::from_html(
        "https://app.test/root",
        r#"<iframe name="child" src="https://embed.test/widget"></iframe>"#,
    );
    let tree = page.frame_tree();
    let child = tree.children_of(tree.root_id())[0].id();

    let blocked = page
        .post_frame_message(tree.root_id(), child, "one", "*")
        .unwrap();
    page.allow_origin("https://embed.test");
    let allowed = page
        .post_frame_message(tree.root_id(), child, "two", "*")
        .unwrap();

    assert!(!blocked.same_origin);
    assert!(!blocked.allowed_by_policy);
    assert!(!blocked.delivered);
    assert!(allowed.allowed_by_policy);
    assert!(allowed.delivered);
}
