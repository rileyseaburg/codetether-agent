use super::{BrowserContext, BrowserPage};
use crate::js::JsValue;

#[test]
fn pages_in_one_context_share_cookie_and_local_storage() {
    let mut context = BrowserContext::new();
    let first = context.new_page(BrowserPage::from_html("https://example.test/a", ""));
    let second = context.new_page(BrowserPage::from_html("https://example.test/b", ""));
    let seed = "localStorage.setItem('token','one'); document.cookie='sid=abc';";

    context.page_mut(first).unwrap().eval_js(seed).unwrap();
    let value = context
        .page_mut(second)
        .unwrap()
        .eval_js("localStorage.getItem('token') + ':' + document.cookie")
        .unwrap();

    assert_eq!(value, JsValue::String("one:sid=abc".into()));
}

#[test]
fn context_pages_keep_session_storage_isolated() {
    let mut context = BrowserContext::new();
    let first = context.new_page(BrowserPage::from_html("https://example.test/a", ""));
    let second = context.new_page(BrowserPage::from_html("https://example.test/b", ""));

    context
        .page_mut(first)
        .unwrap()
        .eval_js("sessionStorage.setItem('tab','one');")
        .unwrap();
    let value = context
        .page_mut(second)
        .unwrap()
        .eval_js("sessionStorage.getItem('tab')")
        .unwrap();

    assert_eq!(value, JsValue::Null);
}

#[test]
fn separate_contexts_do_not_share_state() {
    let mut left = BrowserContext::new();
    let mut right = BrowserContext::new();
    let left_page = left.new_page(BrowserPage::from_html("https://example.test/a", ""));
    let right_page = right.new_page(BrowserPage::from_html("https://example.test/b", ""));
    let seed = "localStorage.setItem('token','one'); document.cookie='sid=abc';";

    left.page_mut(left_page).unwrap().eval_js(seed).unwrap();
    let value = right
        .page_mut(right_page)
        .unwrap()
        .eval_js("localStorage.getItem('token') === null && document.cookie === ''")
        .unwrap();

    assert_eq!(value, JsValue::Bool(true));
}
