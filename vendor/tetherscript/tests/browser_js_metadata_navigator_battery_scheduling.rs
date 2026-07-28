use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn navigator_get_battery_returns_sync_fulfilled_thenable() {
    let value = eval(
        "let seen='pending'; let p=navigator.getBattery();\
         let q=p.then(function(b){\
         seen=[b===p.__promise_value,b.charging,b.chargingTime,\
         b.dischargingTime>1000000000,b.level,b.onchargingchange===null,\
         b.onchargingtimechange===null,b.ondischargingtimechange===null,\
         b.onlevelchange===null,typeof b.addEventListener,\
         typeof b.removeEventListener,b.dispatchEvent({type:'levelchange'})].join(':');\
         return b.level;});\
         [typeof navigator.getBattery,p.__promise_state,q.__promise_state,\
         q.__promise_value,seen].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "function|fulfilled|fulfilled|1|true:true:0:true:1:true:true:true:true:function:function:true"
                .into()
        )
    );
}

#[test]
fn navigator_scheduling_is_input_pending_is_deterministic() {
    let value = eval(
        "let s=navigator.scheduling;\
         [typeof s,s===navigator.scheduling,typeof s.isInputPending,\
         s.isInputPending(),s.isInputPending({includeContinuous:true})].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String("object|true|function|false|false".into())
    );
}
