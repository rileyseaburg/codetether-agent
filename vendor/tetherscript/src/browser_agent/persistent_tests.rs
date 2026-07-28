use super::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn page_actions_keep_script_registered_event_listeners() {
    let html = "<button id='go'>Go</button><script>let count=0; let b=document.getElementById('go'); b.addEventListener('click', function(){ count=count+1; window.hit=count; this.setAttribute('data-clicked','yes'); });</script>";
    let mut page = BrowserPage::from_html("https://example.test", html);

    page.run_scripts().unwrap();
    page.click(&Locator::css("#go")).unwrap();
    let value = page.eval_js("window.hit").unwrap();

    assert_eq!(value, JsValue::Number(1.0));
    assert!(page.session.html.contains("data-clicked=\"yes\""));
}

#[test]
fn persistent_page_runtime_keeps_storage_cookie_and_dom_side_effects() {
    let html = "<input id='q'><button id='save'>Save</button><script>document.getElementById('save').addEventListener('click', function(){ let q=document.getElementById('q'); localStorage.setItem('q', q.value); document.cookie='sid=abc'; q.setAttribute('data-saved','yes'); });</script>";
    let mut page = BrowserPage::from_html("https://example.test/form", html);

    page.run_scripts().unwrap();
    page.fill(&Locator::css("#q"), "hello").unwrap();
    page.click(&Locator::css("#save")).unwrap();
    let value = page
        .eval_js("localStorage.getItem('q') + ':' + document.cookie")
        .unwrap();

    assert_eq!(value, JsValue::String("hello:sid=abc".into()));
    assert_eq!(page.session.local_storage_item("q"), Some("hello"));
    assert!(page
        .session
        .cookie_header("https://example.test/form")
        .contains("sid=abc"));
    assert!(page.session.html.contains("data-saved=\"yes\""));
}
