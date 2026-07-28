use super::super::*;

#[test]
fn promise_jobs_share_queue_microtask_order() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let order='S';\
         P.resolve(0).then(function(){order=order+'P';document.getElementById('out').textContent=order;});\
         queueMicrotask(function(){order=order+'M';document.getElementById('out').textContent=order;});\
         'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("SPM"));
}

#[test]
fn queued_microtasks_keep_order_before_promise_jobs() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let order='S';\
         queueMicrotask(function(){order=order+'M';document.getElementById('out').textContent=order;});\
         P.resolve(0).then(function(){order=order+'P';document.getElementById('out').textContent=order;});\
         'sync';",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("SMP"));
}
