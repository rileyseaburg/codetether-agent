use super::{BrowserPage, Locator};

#[test]
fn press_space_toggles_checkbox() {
    let mut page =
        BrowserPage::from_html("https://example.test", "<input id='ok' type='checkbox'>");

    let report = page.press(&Locator::css("#ok"), "Space").unwrap();

    assert_eq!(report.action, "press");
    assert!(page.session.html.contains("checked=\"\""));
}

#[test]
fn press_backspace_edits_input_value_and_dispatches_keydown() {
    let mut page = BrowserPage::from_html("https://example.test", "<input id='q' value='abc'>");
    page.eval_js(
        "let q=document.getElementById('q'); q.addEventListener('keydown', function(e){ this.setAttribute('data-key', e.key); });",
    )
    .unwrap();

    page.press(&Locator::css("#q"), "Backspace").unwrap();
    let value = page.eval_js("document.getElementById('q').value").unwrap();

    assert_eq!(value, crate::js::JsValue::String("ab".into()));
    assert!(page.session.html.contains("data-key=\"Backspace\""));
}

#[test]
fn press_printable_character_inserts_text() {
    let mut page = BrowserPage::from_html("https://example.test", "<input id='q' value='a'>");

    page.press(&Locator::css("#q"), 'b').unwrap();
    let value = page.eval_js("document.getElementById('q').value").unwrap();

    assert_eq!(value, crate::js::JsValue::String("ab".into()));
}

#[test]
fn unsupported_key_error_is_clear() {
    let mut page = BrowserPage::from_html("https://example.test", "<input id='q'>");

    let err = page.press(&Locator::css("#q"), "F1").unwrap_err();

    assert!(err.contains("unsupported keyboard key \"F1\""));
    assert!(err.contains("expected Enter, Space, Backspace"));
}
