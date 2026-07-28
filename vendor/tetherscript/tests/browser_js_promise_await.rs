use tetherscript::browser_js;
use tetherscript::js::JsValue;

#[test]
fn await_drains_pending_microtask_promise_in_browser() {
    let result = browser_js::eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let resolve;let p=new P(function(r){resolve=r;});\
         queueMicrotask(function(){resolve({data:'late'});});\
         async function f(){let r=await p;\
         document.getElementById('out').textContent=r.data;return r.data;}f();",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("late".into()));
    assert!(result.html.contains("late"));
}

#[test]
fn then_adopts_pending_promise_returned_by_handler() {
    let result = browser_js::eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let resolve;let inner=new P(function(r){resolve=r;});\
         P.resolve('start').then(function(){return inner;}).then(function(v){\
         document.getElementById('out').textContent=v;});\
         queueMicrotask(function(){resolve('adopted');});'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("adopted"));
}
