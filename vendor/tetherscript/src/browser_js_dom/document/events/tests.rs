use super::*;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<button id='go'></button>", script)
        .unwrap()
        .value
}

#[test]
fn document_events_common_names() {
    let script = "let n=['Event','Events','HTMLEvents','MouseEvent','MouseEvents','CustomEvent'];let ok=true;let i=0;while(i<n.length){ok=ok&&(typeof document.createEvent(n[i]).initEvent=='function');i=i+1;}ok;";
    assert_eq!(eval(script), JsValue::Bool(true));
}

#[test]
fn document_events_defaults() {
    let script = "let e=document.createEvent('Event');[e.type,e.bubbles,e.cancelable,e.defaultPrevented,e.target===null,e.currentTarget===null,e.eventPhase,typeof e.timeStamp,e.composedPath().length,typeof e.preventDefault,typeof e.stopPropagation,typeof e.stopImmediatePropagation,typeof e.initEvent].join(':');";
    assert_eq!(
        eval(script).display(),
        ":false:false:false:true:true:0:number:0:function:function:function:function"
    );
}

#[test]
fn document_events_init_event_resets_cancelation() {
    let script = "let e=document.createEvent('Events');e.initEvent('old',false,true);e.preventDefault();let before=e.defaultPrevented;e.initEvent('ready',true,false);e.type+':'+e.bubbles+':'+e.cancelable+':'+before+':'+e.defaultPrevented;";
    assert_eq!(eval(script).display(), "ready:true:false:true:false");
}

#[test]
fn document_events_dispatch_uses_initialized_type() {
    let script = "let b=document.getElementById('go');let e=document.createEvent('HTMLEvents');let seen='';b.addEventListener('legacy',function(x){seen=x.type+':'+x.target.id+':'+x.currentTarget.id;});e.initEvent('legacy',false,false);let ok=b.dispatchEvent(e);seen+':'+ok;";
    assert_eq!(eval(script).display(), "legacy:go:go:true");
}

#[test]
fn document_events_cancelable_prevent_default() {
    let script = "let b=document.getElementById('go');let e=document.createEvent('MouseEvents');e.initEvent('click',true,true);b.addEventListener('click',function(x){x.preventDefault();});let ok=b.dispatchEvent(e);ok+':'+e.cancelable;";
    assert_eq!(eval(script).display(), "false:true");
}
