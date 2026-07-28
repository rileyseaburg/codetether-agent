use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn select_option_updates_value_and_dispatches_events() {
    let html = "<select id='s'><option value='a'>A</option><option value='b'>B</option></select><script>let log='';let s=document.getElementById('s');s.addEventListener('input',function(){log=log+'i'});s.addEventListener('change',function(){log=log+'c'});</script>";
    let mut page = BrowserPage::from_html("mem://select", html);
    page.run_scripts().unwrap();

    page.select_option(&Locator::css("#s"), "b").unwrap();
    let value = page
        .eval_js("document.getElementById('s').value+':'+log")
        .unwrap();

    assert_eq!(value, JsValue::String("b:ic".into()));
}

#[test]
fn select_option_rejects_wrong_element_or_missing_value() {
    let mut page = BrowserPage::from_html(
        "mem://bad-select",
        "<input id='q'><select id='s'><option value='a'>A</option></select>",
    );

    let not_select = page.select_option(&Locator::css("#q"), "a").unwrap_err();
    let missing = page.select_option(&Locator::css("#s"), "z").unwrap_err();

    assert!(not_select.contains("element is not a select"));
    assert!(missing.contains("no option value"));
}
