use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn touch_tap_exposes_touch_lists() {
    let html = "<div id='t' style='width:20px;height:10px'>T</div><script>\
    let t=document.getElementById('t');t.addEventListener('touchstart',function(e){\
    window.touch=e.touches.length+','+e.changedTouches[0].clientX+','+e.changedTouches[0].force;});\
    t.addEventListener('touchend',function(e){window.end=e.touches.length+','+e.changedTouches.length;});</script>";
    let mut page = BrowserPage::from_html("mem://touch", html);
    page.run_scripts().unwrap();

    page.touch_tap(&Locator::css("#t")).unwrap();

    assert_eq!(
        page.eval_js("window.touch").unwrap(),
        JsValue::String("1,10,0.5".into())
    );
    assert_eq!(
        page.eval_js("window.end").unwrap(),
        JsValue::String("0,1".into())
    );
}
