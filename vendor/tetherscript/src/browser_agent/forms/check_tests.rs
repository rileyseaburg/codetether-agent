use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn check_and_uncheck_checkbox_dispatch_events() {
    let html = "<input id='ok' type='checkbox'><script>let log='';let n=document.getElementById('ok');n.addEventListener('click',function(){log=log+'c'});n.addEventListener('input',function(){log=log+'i'});n.addEventListener('change',function(){log=log+'h'});</script>";
    let mut page = BrowserPage::from_html("mem://check", html);
    page.run_scripts().unwrap();

    page.check(&Locator::css("#ok")).unwrap();
    assert!(page.session.html.contains("checked=\"\""));
    page.uncheck(&Locator::css("#ok")).unwrap();

    assert!(!page.session.html.contains("checked=\"\""));
    assert_eq!(
        page.eval_js("log").unwrap(),
        JsValue::String("cihcih".into())
    );
}

#[test]
fn check_radio_updates_group() {
    let html =
        "<input id='a' type='radio' name='pick' checked><input id='b' type='radio' name='pick'>";
    let mut page = BrowserPage::from_html("mem://radio", html);

    page.check(&Locator::css("#b")).unwrap();
    let value = page
        .eval_js("document.getElementById('a').checked+':'+document.getElementById('b').checked")
        .unwrap();

    assert_eq!(value, JsValue::String("false:true".into()));
}

#[test]
fn check_rejects_wrong_element() {
    let mut page = BrowserPage::from_html("mem://bad-check", "<button id='go'>Go</button>");

    let err = page.check(&Locator::css("#go")).unwrap_err();

    assert!(err.contains("cannot perform form action"));
    assert!(err.contains("not an input"));
}
