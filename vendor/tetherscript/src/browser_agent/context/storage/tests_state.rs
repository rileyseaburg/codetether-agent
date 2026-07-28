use crate::browser_agent::{BrowserContext, BrowserPage};
use crate::js::JsValue;

#[test]
fn storage_state_restores_shared_state_without_session_storage() {
    let mut context = BrowserContext::new();
    let page = context.new_page(BrowserPage::from_html("https://app.test/a", ""));
    context.page_mut(page).unwrap().eval_js(seed()).unwrap();

    let snapshot = context.storage_state();
    let mut restored = BrowserContext::new();
    restored.restore_storage_state(snapshot);
    let page = restored.new_page(BrowserPage::from_html("https://app.test/b", ""));
    let value = restored.page_mut(page).unwrap().eval_js(read()).unwrap();

    assert_eq!(value, JsValue::String("one:null:sid=abc".into()));
}

fn seed() -> &'static str {
    "localStorage.setItem('token','one'); sessionStorage.setItem('tab','a'); document.cookie='sid=abc';"
}

fn read() -> &'static str {
    "localStorage.getItem('token') + ':' + sessionStorage.getItem('tab') + ':' + document.cookie"
}
