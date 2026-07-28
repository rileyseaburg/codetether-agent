use super::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn clipboard_text_can_be_written_read_and_cleared() {
    let mut page = BrowserPage::from_html("https://example.test", "");

    page.write_clipboard("hello");
    assert_eq!(page.read_clipboard(), "hello");
    page.clear_clipboard();

    assert_eq!(page.read_clipboard(), "");
}

#[test]
fn copy_text_stores_element_text() {
    let mut page = BrowserPage::from_html(
        "https://example.test",
        "<button id='copy'>Copy <span>me</span></button>",
    );

    let report = page.copy_text(&Locator::css("#copy")).unwrap();

    assert_eq!(report.action, "copy_text");
    assert_eq!(page.read_clipboard(), "Copy me");
}

#[test]
fn paste_replaces_input_value_and_dispatches_input() {
    let mut page = BrowserPage::from_html("https://example.test", "<input id='q' value='old'>");
    page.eval_js(
        "let q=document.getElementById('q'); q.addEventListener('input', function(){ this.setAttribute('data-input','yes'); });",
    )
    .unwrap();
    page.write_clipboard("new text");

    page.paste(&Locator::css("#q")).unwrap();
    let value = page.eval_js("document.getElementById('q').value").unwrap();

    assert_eq!(value, JsValue::String("new text".into()));
    assert!(page.session.html.contains("data-input=\"yes\""));
}

#[test]
fn clipboard_is_page_local() {
    let mut first = BrowserPage::from_html("https://example.test/a", "");
    let second = BrowserPage::from_html("https://example.test/b", "");

    first.write_clipboard("secret");

    assert_eq!(first.read_clipboard(), "secret");
    assert_eq!(second.read_clipboard(), "");
}
