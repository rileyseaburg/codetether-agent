use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<main></main>", script).unwrap().value
}

#[test]
fn navigator_wake_lock_request_resolves_screen_sentinel() {
    let value = eval(
        "let seen='';let p=navigator.wakeLock.request('screen');\
         p.then(function(s){let r=s.release();seen=[s.type,s.released,\
         s.onrelease===null,typeof s.addEventListener,typeof s.removeEventListener,\
         ''+s.addEventListener('release',function(){}),''+s.removeEventListener('release',\
         function(){}),s.dispatchEvent({type:'release'}),r.__promise_state,\
         ''+r.__promise_value].join('|');});\
         [typeof navigator.wakeLock,typeof navigator.wakeLock.request,\
         p.__promise_state,seen].join(';');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "object;function;fulfilled;screen|true|true|function|function|undefined|\
             undefined|true|fulfilled|undefined"
                .into()
        )
    );
}

#[test]
fn navigator_wake_lock_rejects_unsupported_types() {
    let value = eval(
        "let seen='';let p=navigator.wakeLock.request('system');\
         let next=p.catch(function(e){seen=e;return 'handled';});\
         [p.__promise_state,p.__promise_reason,seen,next.__promise_state,\
         next.__promise_value].join('|');",
    );

    assert_eq!(
        value,
        JsValue::String(
            "rejected|navigator.wakeLock.request: unsupported type 'system'|\
             navigator.wakeLock.request: unsupported type 'system'|fulfilled|handled"
                .into()
        )
    );
}
