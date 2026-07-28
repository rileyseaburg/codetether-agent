use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn visual_viewport_handler_properties_receive_event_target() {
    let script = "let v=window.visualViewport;let seen='';\
        v.onresize=function(e){seen=e.type+':' + (e.target===v) + ':' + (this===v);};\
        let ok=v.dispatchEvent({type:'resize'});\
        seen+':'+ok+':'+typeof v.onscroll;";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("resize:true:true:true:object".into())
    );
}
