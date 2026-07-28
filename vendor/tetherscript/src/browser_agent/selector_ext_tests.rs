use super::query::locate;
use super::{BrowserPage, Locator};

fn ids(html: &str, selector: &str) -> Vec<String> {
    let page = BrowserPage::from_html("mem://selector-ext", html);
    locate(&page.session.document, &Locator::css(selector))
        .into_iter()
        .map(|item| item.element.attrs.get("id").cloned().unwrap_or_default())
        .collect()
}

#[test]
fn visible_filter_skips_hidden_dom_states() {
    let html = "<button id='a'>Save</button><button id='b' hidden>Hide</button>\
        <button id='c' style='display:none'>Gone</button>\
        <button id='d' aria-hidden='true'>Mute</button>";

    assert_eq!(ids(html, "button:visible"), vec!["a"]);
}

#[test]
fn enabled_and_disabled_filters_match_control_state() {
    let html = "<button id='a'></button><button id='b' disabled></button>\
        <button id='c' aria-disabled='true'></button>";

    assert_eq!(ids(html, "button:enabled"), vec!["a"]);
    assert_eq!(ids(html, "button:disabled"), vec!["b", "c"]);
}

#[test]
fn checked_filter_matches_checked_elements() {
    let html = "<input id='a' type='checkbox' checked><input id='b' type='checkbox'>\
        <div id='c' role='checkbox' aria-checked='true'></div>";

    assert_eq!(ids(html, "input:checked"), vec!["a"]);
    assert_eq!(ids(html, "[role='checkbox']:checked"), vec!["c"]);
}

#[test]
fn has_text_and_nth_filters_constrain_css_matches() {
    let html = "<button id='a'>Save</button><button id='b'>Delete</button>\
        <button id='c'>Save draft</button>";

    assert_eq!(ids(html, "button:has-text('Save')"), vec!["a", "c"]);
    assert_eq!(ids(html, "button:has-text('Save'):nth(1)"), vec!["c"]);
}
