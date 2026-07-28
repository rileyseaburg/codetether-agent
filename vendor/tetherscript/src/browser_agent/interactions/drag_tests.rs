use crate::browser_agent::{BrowserPage, Locator};
use crate::js::JsValue;

#[test]
fn drag_to_dispatches_drag_events_with_data_transfer() {
    let html = "<div id='src'>Seed</div><input id='dst'><script>window.log='';\
    let s=document.getElementById('src');let d=document.getElementById('dst');\
    function add(e){window.log=window.log+e.type+':'+e.dataTransfer.getData('text/plain')+'>'; }\
    s.addEventListener('dragstart',function(e){e.dataTransfer.setData('text/plain','Payload');add(e);});\
    d.addEventListener('dragenter',add);d.addEventListener('dragover',add);\
    d.addEventListener('drop',add);d.addEventListener('input',function(){window.log=window.log+'input:'+this.value+'>';});</script>";
    let mut page = BrowserPage::from_html("mem://drag", html);
    page.run_scripts().unwrap();

    let report = page
        .drag_to(&Locator::css("#src"), &Locator::css("#dst"))
        .unwrap();

    assert_eq!(report.action, "drag_to");
    assert_eq!(
        page.eval_js("document.getElementById('dst').value")
            .unwrap(),
        JsValue::String("Payload".into())
    );
    assert_eq!(
        page.eval_js("window.log").unwrap(),
        JsValue::String(
            "dragstart:Payload>dragenter:Payload>dragover:Payload>drop:Payload>input:Payload>"
                .into()
        )
    );
}

#[test]
fn drag_pointer_capture_reroutes_move_and_up() {
    let html = "<div id='src'>A</div><div id='dst'>B</div><script>window.log='';\
    let s=document.getElementById('src');let d=document.getElementById('dst');\
    s.addEventListener('pointerdown',function(e){window.log=window.log+'down>';e.setPointerCapture(1);});\
    s.addEventListener('pointermove',function(e){window.log=window.log+'move:'+e.hasPointerCapture()+'>';});\
    s.addEventListener('pointerup',function(){window.log=window.log+'up>';});\
    d.addEventListener('pointermove',function(){window.log=window.log+'target-move>';});</script>";
    let mut page = BrowserPage::from_html("mem://drag-capture", html);
    page.run_scripts().unwrap();

    page.drag_to(&Locator::css("#src"), &Locator::css("#dst"))
        .unwrap();

    assert_eq!(
        page.eval_js("window.log").unwrap(),
        JsValue::String("down>move:true>up>".into())
    );
}
