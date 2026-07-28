use super::*;

#[test]
fn init_event_reinitializes_constructed_event() {
    let result = eval_with_dom(
        "<button id='go'></button>",
        "let e=Event('old',{bubbles:true,cancelable:true});\
         e.preventDefault();e.initEvent('ready',false,true);\
         e.type+':'+e.bubbles+':'+e.cancelable+':'+e.defaultPrevented+':'\
         +(typeof e.initCustomEvent);",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("ready:false:true:false:undefined".into())
    );
}

#[test]
fn init_custom_event_reinitializes_detail() {
    let result = eval_with_dom(
        "<button id='go'></button>",
        "let e=CustomEvent('old',{detail:'before'});\
         e.initCustomEvent('ready',true,false,'after');\
         e.type+':'+e.bubbles+':'+e.cancelable+':'+e.detail;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("ready:true:false:after".into())
    );
}
