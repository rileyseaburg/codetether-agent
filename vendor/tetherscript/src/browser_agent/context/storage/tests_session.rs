use crate::browser_agent::BrowserPage;

#[test]
fn session_storage_survives_same_origin_navigation_per_page() {
    let mut page = BrowserPage::from_html("https://app.test/a", "");
    page.set_session_storage_item("tab", "one");

    page.goto_html("https://app.test/b", "");
    assert_eq!(page.session_storage_item("tab").as_deref(), Some("one"));
    page.goto_html("https://other.test/", "");
    assert_eq!(page.session_storage_item("tab"), None);
    page.goto_html("https://app.test/c", "");
    assert_eq!(page.session_storage_item("tab").as_deref(), Some("one"));
}
