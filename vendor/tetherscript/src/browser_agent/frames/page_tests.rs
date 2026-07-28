use crate::browser_agent::BrowserPage;

#[test]
fn page_iframes_expose_child_metadata() {
    let page = BrowserPage::from_html(
        "https://root.test",
        r#"<iframe name="login" src="/login"></iframe><iframe></iframe>"#,
    );

    let tree = page.frame_tree();
    let children = tree.children_of(tree.root_id());

    assert_eq!(children.len(), 2);
    assert_eq!(children[0].name(), "login");
    assert_eq!(children[0].url(), "/login");
    assert_eq!(children[1].name(), "");
    assert_eq!(children[1].url(), "about:blank");
}

#[test]
fn repeated_scans_keep_stable_iframe_ids() {
    let html = r#"<main><iframe name="one"></iframe><iframe name="two"></iframe></main>"#;
    let mut page = BrowserPage::from_html("mem://root", html);
    let first: Vec<_> = page.frames().iter().map(|frame| frame.id()).collect();

    page.load_html(html);
    let second: Vec<_> = page.frames().iter().map(|frame| frame.id()).collect();

    assert_eq!(first, second);
}

#[test]
fn nested_iframe_markup_produces_nested_frame_entries() {
    let page = BrowserPage::from_html(
        "mem://root",
        r#"<iframe name="outer" src="outer"><iframe name="inner" src="inner"></iframe></iframe>"#,
    );

    let tree = page.frame_tree();
    let outer = tree.children_of(tree.root_id())[0];
    let inner = tree.children_of(outer.id())[0];

    assert_eq!(outer.name(), "outer");
    assert_eq!(inner.name(), "inner");
    assert_eq!(tree.parent_of(inner.id()).unwrap().id(), outer.id());
}
