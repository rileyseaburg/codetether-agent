use crate::browser_agent::{BrowserPage, Locator};
use crate::browser_session::ScrollState;
use crate::js::JsValue;

#[test]
fn wheel_dispatches_fields_and_scrolls_unless_prevented() {
    let html = "<div id='w' style='width:20px;height:10px'>W</div><script>\
    let w=document.getElementById('w');w.addEventListener('wheel',function(e){\
    window.seen=e.deltaX+','+e.deltaY+','+e.deltaMode+','+e.clientX;});</script>";
    let mut page = BrowserPage::from_html("mem://wheel", html);
    page.run_scripts().unwrap();

    page.wheel(&Locator::css("#w"), 3, 40).unwrap();

    assert_eq!(page.session.scroll, ScrollState { x: 3, y: 40 });
    assert_eq!(
        page.eval_js("window.seen").unwrap(),
        JsValue::String("3,40,0,10".into())
    );
    page.eval_js(
        "document.getElementById('w').addEventListener('wheel',function(e){e.preventDefault();});",
    )
    .unwrap();
    page.wheel(&Locator::css("#w"), 0, 20).unwrap();
    assert_eq!(page.session.scroll, ScrollState { x: 3, y: 40 });
}
