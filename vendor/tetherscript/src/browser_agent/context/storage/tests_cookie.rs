use crate::browser_agent::{BrowserContext, BrowserPage};
use crate::js::JsValue;

#[test]
fn storage_state_restores_cookie_path_and_origin_scope() {
    let mut context = BrowserContext::new();
    let page = context.new_page(BrowserPage::from_html("https://app.test/app/login", ""));
    context
        .page_mut(page)
        .unwrap()
        .eval_js("document.cookie='sid=abc; Path=/app';")
        .unwrap();

    let snapshot = context.storage_state();
    let mut restored = BrowserContext::new();
    restored.restore_storage_state(snapshot);

    assert_cookie(&mut restored, "https://app.test/app/page", "sid=abc");
    assert_cookie(&mut restored, "https://app.test/other", "");
    assert_cookie(&mut restored, "https://other.test/app/page", "");
}

fn assert_cookie(context: &mut BrowserContext, url: &str, expected: &str) {
    let page = context.new_page(BrowserPage::from_html(url, ""));
    let value = context
        .page_mut(page)
        .unwrap()
        .eval_js("document.cookie")
        .unwrap();
    assert_eq!(value, JsValue::String(expected.into()));
}
