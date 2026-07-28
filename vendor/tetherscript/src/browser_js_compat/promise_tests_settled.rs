use super::super::*;

#[test]
fn promise_resolve_then_runs_at_microtask_checkpoint() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let seen='sync';\
         P.resolve(4).then(function(v){seen='r'+v;document.getElementById('out').textContent=seen;});seen;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("r4"));
}

#[test]
fn promise_reject_catch_runs_at_microtask_checkpoint() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let seen='sync';\
         P.reject('bad').catch(function(e){seen=e;document.getElementById('out').textContent=seen;});seen;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("bad"));
}

#[test]
fn promise_finally_preserves_original_settlement() {
    let result = eval_with_dom(
        "<p id='a'></p><p id='b'></p>",
        "let P=window.Promise;let a='';let b='';\
         P.resolve('ok').finally(function(){a='f';}).then(function(v){a=a+v;document.getElementById('a').textContent=a;});\
         P.reject('no').finally(function(){b='f';}).catch(function(e){b=b+e;document.getElementById('b').textContent=b;});'sync';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("fok"));
    assert!(result.html.contains("fno"));
}

#[test]
fn await_unwraps_window_promise_fulfillment() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;async function f(){let r=await P.resolve({data:'ok'});\
         document.getElementById('out').textContent=r.data;return r.data;}f();",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("ok".into()));
    assert!(result.html.contains("ok"));
}
