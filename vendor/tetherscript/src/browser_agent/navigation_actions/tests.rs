//! Navigation action tests.

use crate::browser_agent::{BrowserPage, Locator, PageLoadState};

#[test]
fn click_anchor_navigates_and_updates_metadata() {
    let mut page = BrowserPage::from_html(
        "https://example.test/app/index.html",
        "<a id='next' href='/next?from=app'>Next</a>",
    );
    let before = page.navigation().id;

    page.click(&Locator::css("#next")).unwrap();

    assert_eq!(page.session.url, "https://example.test/next?from=app");
    assert_eq!(page.navigation().id, before + 1);
    assert_eq!(page.navigation().action, "anchor_click");
    assert_eq!(page.load_state(), PageLoadState::Load);
}

#[test]
fn download_anchor_records_without_navigation() {
    let mut page = BrowserPage::from_html(
        "https://example.test/app",
        "<a id='dl' href='/file.bin' download='file.bin'>Save</a>",
    );
    let before = page.navigation().clone();

    page.click(&Locator::css("#dl")).unwrap();

    assert_eq!(page.navigation().id, before.id);
    assert_eq!(page.navigation().url, before.url);
    assert_eq!(page.navigation().action, before.action);
    assert_eq!(page.session.url, "https://example.test/app");
    assert_eq!(page.downloads().len(), 1);
}

#[test]
fn get_form_submit_button_navigates_with_query() {
    let html = "<form action='/search' method='get'>\
        <input name='q' value='rust browser'>\
        <input type='checkbox' name='ok' checked>\
        <button id='go' name='commit' value='yes'>Go</button>\
    </form>";
    let mut page = BrowserPage::from_html("https://example.test/app", html);

    page.click(&Locator::css("#go")).unwrap();

    assert_eq!(
        page.session.url,
        "https://example.test/search?q=rust+browser&ok=on&commit=yes"
    );
    assert_eq!(page.navigation().action, "form_submit");
}
