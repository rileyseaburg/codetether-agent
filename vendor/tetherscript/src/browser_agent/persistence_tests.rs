use crate::browser_agent::{BrowserContext, BrowserPage, DeviceScale};
use crate::js::JsValue;

#[test]
fn page_snapshot_restore_preserves_dom_and_storage() {
    let html = "<main id='app'>App</main>";
    let mut page = BrowserPage::from_html("https://example.test/app", html);
    page.set_viewport_size(320, 200).unwrap();
    page.device_scale = DeviceScale::new(2.0, false).unwrap();
    page.eval_js("document.getElementById('app').setAttribute('data-ready','yes'); localStorage.setItem('token','one'); sessionStorage.setItem('tab','a'); document.cookie='sid=abc';").unwrap();

    let snapshot = page.snapshot_state();
    let mut restored = BrowserPage::new(Default::default());
    restored.restore_state(snapshot).unwrap();

    assert_eq!(restored.session.url, "https://example.test/app");
    assert!(restored.session.html.contains("data-ready=\"yes\""));
    assert_eq!(restored.session.local_storage_item("token"), Some("one"));
    assert_eq!(restored.session.session_storage_item("tab"), Some("a"));
    assert!(restored
        .session
        .cookie_header("https://example.test/app")
        .contains("sid=abc"));
    assert_eq!(restored.viewport().width, 320);
    assert_eq!(restored.viewport().device_scale_factor, 2.0);
}

#[test]
fn context_snapshot_restore_preserves_shared_state() {
    let mut context = BrowserContext::new();
    let first = context.new_page(BrowserPage::from_html("https://example.test/a", ""));
    let second = context.new_page(BrowserPage::from_html("https://example.test/b", ""));
    context
        .page_mut(first)
        .unwrap()
        .eval_js("localStorage.setItem('token','one'); document.cookie='sid=abc';")
        .unwrap();

    let snapshot = context.snapshot_state();
    let mut restored = BrowserContext::new();
    restored.restore_state(snapshot).unwrap();
    let value = restored
        .page_mut(second)
        .unwrap()
        .eval_js("localStorage.getItem('token') + ':' + document.cookie")
        .unwrap();

    assert_eq!(restored.len(), 2);
    assert_eq!(value, JsValue::String("one:sid=abc".into()));
}
