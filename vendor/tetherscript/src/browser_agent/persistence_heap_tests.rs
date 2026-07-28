use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn page_restore_does_not_clone_js_heap() {
    let html = "<button id='go'>Go</button>";
    let mut page = BrowserPage::from_html("https://example.test/js", html);
    page.eval_js("window.secret = 41; document.getElementById('go').addEventListener('click', function(){ window.secret = 99; });").unwrap();

    let snapshot = page.snapshot_state();
    let mut restored = BrowserPage::new(Default::default());
    restored.restore_state(snapshot).unwrap();
    restored.click(&Locator::css("#go")).unwrap();
    let value = restored.eval_js("window.secret").unwrap();

    assert_eq!(value, JsValue::Undefined);
}
