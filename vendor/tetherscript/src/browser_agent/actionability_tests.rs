use super::{BrowserPage, Locator};

#[test]
fn click_scrolls_element_into_view() {
    let html = "<div style='height:40px'>Top</div><button id='go' style='height:2px'>Go</button>";
    let mut page = BrowserPage::from_html("mem://scroll", html);
    page.viewport_height = 10;

    page.click(&Locator::css("#go")).unwrap();

    assert!(page.session.scroll.y > 0);
}

#[test]
fn fill_rejects_non_editable_element() {
    let mut page = BrowserPage::from_html("mem://fill", "<div id='note'>Text</div>");

    let err = page.fill(&Locator::css("#note"), "nope").unwrap_err();

    assert!(err.contains("editable"));
    assert!(err.contains("element is not fillable"));
}

#[test]
fn disabled_controls_reject_click_and_fill() {
    let html = "<button id='go' disabled>Go</button><input id='q' disabled>";
    let mut page = BrowserPage::from_html("mem://disabled", html);

    let click_err = page.click(&Locator::css("#go")).unwrap_err();
    let fill_err = page.fill(&Locator::css("#q"), "text").unwrap_err();

    assert!(click_err.contains("enabled"));
    assert!(click_err.contains("disabled"));
    assert!(fill_err.contains("enabled"));
    assert!(fill_err.contains("disabled"));
}
