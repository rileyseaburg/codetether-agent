use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn screen_orientation_dispatches_change_listeners_and_handler() {
    let script = "let o=screen.orientation;let seen='';\
        function f(e){seen=seen+'L'+e.type+':' + (e.target===o) + ':' + (this===o)+';';}\
        o.addEventListener('change',f);\
        o.onchange=function(e){seen=seen+'H'+e.type+':' + (e.currentTarget===o);};\
        let ok=o.dispatchEvent({type:'change'});o.removeEventListener('change',f);\
        o.dispatchEvent({type:'change'});seen+':'+ok;";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("Lchange:true:true;Hchange:trueHchange:true:true".into())
    );
}

#[test]
fn screen_orientation_lock_returns_rejected_thenable() {
    let script = "let o=screen.orientation;let p=o.lock('portrait-primary');let seen='';\
        let next=p.catch(function(e){seen=e;return 'handled';});\
        [typeof o.lock,p.__promise_state,p.__promise_reason,seen,\
        next.__promise_state,next.__promise_value,o.type,o.angle].join('|');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "function|rejected|screen.orientation.lock: unsupported|screen.orientation.lock: \
             unsupported|fulfilled|handled|landscape-primary|0"
                .into()
        )
    );
}

#[test]
fn screen_orientation_unlock_is_stable_undefined_noop() {
    let script = "let o=screen.orientation;let seen='';\
        o.addEventListener('change',function(){seen='event';});\
        let before=o.type+':'+o.angle;let ret=o.unlock();\
        let after=o.type+':'+o.angle;\
        [typeof o.unlock,ret===undefined,before,after,seen].join('|');";
    let result = eval_with_dom("", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("function|true|landscape-primary:0|landscape-primary:0|".into())
    );
}
