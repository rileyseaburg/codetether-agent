use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn pointer_capture_records_owner_and_dispatches_events() {
    let html = "<button id='a'>A</button><button id='b'>B</button><script>window.log='';\
    document.getElementById('a').addEventListener('gotpointercapture',function(){window.log=window.log+'got>';});\
    document.getElementById('a').addEventListener('lostpointercapture',function(){window.log=window.log+'lost>';});</script>";
    let mut page = BrowserPage::from_html("mem://capture", html);
    page.run_scripts().unwrap();

    page.set_pointer_capture(&Locator::css("#a")).unwrap();
    assert!(page.has_pointer_capture(&Locator::css("#a")).unwrap());
    assert!(!page.has_pointer_capture(&Locator::css("#b")).unwrap());
    assert!(page.release_pointer_capture(&Locator::css("#a")).unwrap());
    assert!(!page.has_pointer_capture(&Locator::css("#a")).unwrap());
    assert_eq!(
        page.eval_js("window.log").unwrap(),
        JsValue::String("got>lost>".into())
    );
}

#[test]
fn pointer_move_honors_active_capture() {
    let html = "<button id='a'>A</button><button id='b'>B</button><script>window.log='';\
    document.getElementById('a').addEventListener('pointermove',function(){window.log=window.log+'a>';});\
    document.getElementById('b').addEventListener('pointermove',function(){window.log=window.log+'b>';});</script>";
    let mut page = BrowserPage::from_html("mem://capture-move", html);
    page.run_scripts().unwrap();

    page.set_pointer_capture(&Locator::css("#a")).unwrap();
    page.pointer_move(&Locator::css("#b")).unwrap();
    page.release_pointer_capture(&Locator::css("#a")).unwrap();
    page.pointer_move(&Locator::css("#b")).unwrap();

    assert_eq!(
        page.eval_js("window.log").unwrap(),
        JsValue::String("a>b>".into())
    );
}
