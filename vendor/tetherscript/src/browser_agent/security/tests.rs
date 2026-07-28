use crate::browser_agent::{BrowserContext, BrowserPage, Origin};

#[test]
fn origin_parsing_normalizes_scheme_host_and_default_port() {
    let origin = Origin::parse("HTTPS://Example.test:443/path#frag");
    let same = Origin::parse("https://example.test/other");
    let other = Origin::parse("https://example.test:8443/other");

    assert_eq!(origin.serialized(), "https://example.test");
    assert!(origin.is_same_origin(&same));
    assert!(!origin.is_same_origin(&other));
}

#[test]
fn page_allows_same_origin_and_blocks_cross_origin_by_default() {
    let page = BrowserPage::from_html("https://app.test/path#view", "");
    let same = page.request_security_metadata("/api");
    let cross = page.request_security_metadata("https://api.test/data");

    assert!(same.same_origin);
    assert!(same.allowed_by_policy);
    assert_eq!(cross.referrer.as_deref(), Some("https://app.test/path"));
    assert!(!cross.allowed_by_policy);
}

#[test]
fn page_allowed_origin_unblocks_cross_origin_metadata() {
    let mut page = BrowserPage::from_html("https://app.test/path", "");

    page.allow_origin("https://api.test/data");

    assert!(page.is_request_allowed("https://api.test/v1"));
    assert_eq!(page.current_origin().serialized(), "https://app.test");
}

#[test]
fn context_allowed_origins_remain_isolated() {
    let mut open = BrowserContext::new();
    let mut closed = BrowserContext::new();
    open.allow_origin("https://api.test");
    let open_page = open.new_page(BrowserPage::from_html("https://app.test", ""));
    let closed_page = closed.new_page(BrowserPage::from_html("https://app.test", ""));

    assert!(open
        .page(open_page)
        .unwrap()
        .is_request_allowed("https://api.test/x"));
    assert!(!closed
        .page(closed_page)
        .unwrap()
        .is_request_allowed("https://api.test/x"));
}
