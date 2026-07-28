use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn mouse_and_pointer_events_expose_richer_fields() {
    let html = "<button id='b' style='width:20px;height:10px'>B</button><script>\
    window.log='';let b=document.getElementById('b');\
    b.addEventListener('mousedown',function(e){window.log=e.clientX+','+e.clientY+','+e.buttons+','+e.detail;});\
    b.addEventListener('pointermove',function(e){window.ptr=e.pointerType+','+e.pressure+','+e.isPrimary+','+e.buttons;});</script>";
    let mut page = BrowserPage::from_html("mem://pointer-fields", html);
    page.run_scripts().unwrap();

    page.mouse_down(&Locator::css("#b")).unwrap();
    page.pointer_move(&Locator::css("#b")).unwrap();

    assert_eq!(
        page.eval_js("window.log").unwrap(),
        JsValue::String("10,5,1,1".into())
    );
    assert_eq!(
        page.eval_js("window.ptr").unwrap(),
        JsValue::String("mouse,0,true,0".into())
    );
}
