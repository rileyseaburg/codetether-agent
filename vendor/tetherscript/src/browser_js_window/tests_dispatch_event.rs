use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn window_dispatch_event_preserves_object_fields_and_cancellation() {
    let result = eval_with_dom(
        "<main></main>",
        "let seen='';let e=CustomEvent('ready',{cancelable:true,detail:{value:7}});\
         window.addEventListener('ready',function(ev){\
         seen=ev.type+':'+ev.detail.value+':'+typeof ev.target+':'+typeof ev.currentTarget;\
         ev.preventDefault();});let ok=window.dispatchEvent(e);seen+':'+ok;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("ready:7:undefined:undefined:false".into())
    );
}
