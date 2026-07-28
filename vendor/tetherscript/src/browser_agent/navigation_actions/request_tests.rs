//! Request-submit navigation tests.

use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn request_submit_get_form_navigates_with_query() {
    let mut page = BrowserPage::from_html(
        "https://example.test/app",
        "<form id='f' action='/send'><input name='q' value='rust'></form>",
    );

    page.request_submit(&Locator::css("#f")).unwrap();

    assert_eq!(page.session.url, "https://example.test/send?q=rust");
    assert_eq!(page.navigation().action, "request_submit");
}
