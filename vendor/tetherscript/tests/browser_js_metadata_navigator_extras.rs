use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn navigator_get_gamepads_returns_empty_array() {
    let value = eval(
        "let pads=navigator.getGamepads();\
         [typeof navigator.getGamepads,pads.length,pads.join(',')].join('|');",
    );

    assert_eq!(value, JsValue::String("function|0|".into()));
}

#[test]
fn navigator_credentials_return_synchronous_thenables() {
    let value = eval(
        "let seen='';\
         let a=navigator.credentials.get().catch(function(e){seen=seen+e+';';return 'get';});\
         let b=navigator.credentials.create({}).catch(function(e){seen=seen+e+';';return 'create';});\
         let c=navigator.credentials.store({}).then(null,function(e){seen=seen+e+';';return 'store';});\
         let cleared='pending'; let d=navigator.credentials.preventSilentAccess();\
         d.then(function(v){cleared=''+v;});\
         [typeof navigator.credentials,typeof navigator.credentials.get,\
         typeof navigator.credentials.create,typeof navigator.credentials.store,\
         typeof navigator.credentials.preventSilentAccess,a.__promise_value,b.__promise_value,\
         c.__promise_value,d.__promise_state,cleared,seen].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "object|function|function|function|function|get|create|store|fulfilled|undefined|\
             navigator.credentials.get: unsupported credential operation;\
             navigator.credentials.create: unsupported credential operation;\
             navigator.credentials.store: unsupported credential operation;"
                .into()
        )
    );
}
