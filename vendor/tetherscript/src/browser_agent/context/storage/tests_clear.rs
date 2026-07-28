use crate::browser_agent::{BrowserContext, BrowserPage};
use crate::js::JsValue;

#[test]
fn clear_storage_state_clears_shared_and_page_session_storage() {
    let mut context = BrowserContext::new();
    let page = context.new_page(BrowserPage::from_html("https://app.test/a", ""));
    context.page_mut(page).unwrap().eval_js(seed()).unwrap();

    context.clear_storage_state();
    let value = context.page_mut(page).unwrap().eval_js(cleared()).unwrap();

    assert_eq!(value, JsValue::Bool(true));
}

fn seed() -> &'static str {
    "localStorage.setItem('token','one'); sessionStorage.setItem('tab','a'); document.cookie='sid=abc';"
}

fn cleared() -> &'static str {
    "localStorage.getItem('token') === null && sessionStorage.getItem('tab') === null && document.cookie === ''"
}
