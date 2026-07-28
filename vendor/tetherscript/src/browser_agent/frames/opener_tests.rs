use crate::browser_agent::{BrowserContext, BrowserPage};

#[test]
fn frame_window_relation_reports_parent_and_top() {
    let page = BrowserPage::from_html(
        "https://app.test",
        r#"<iframe name="outer"><iframe name="inner"></iframe></iframe>"#,
    );
    let tree = page.frame_tree();
    let root = tree.root_id();
    let outer = tree.children_of(root)[0].id();
    let inner = tree.children_of(outer)[0].id();

    let relation = page.frame_window_relation(inner).unwrap();

    assert_eq!(relation.frame_id, inner);
    assert_eq!(relation.parent, Some(outer));
    assert_eq!(relation.top, root);
    assert_eq!(relation.opener, None);
}

#[test]
fn context_open_page_with_opener_sets_top_window_opener() {
    let mut context = BrowserContext::new();
    let opener = context.new_page(BrowserPage::from_html("https://app.test", ""));
    let opened = context
        .open_page_with_opener(opener, BrowserPage::from_html("https://popup.test", ""))
        .unwrap();
    let page = context.page(opened).unwrap();
    let root = page.frame_tree().root_id();

    let relation = page.frame_window_relation(root).unwrap();

    assert_eq!(relation.opener.unwrap().page_index, opener);
    assert_eq!(page.opener().unwrap().page_index, opener);
}
