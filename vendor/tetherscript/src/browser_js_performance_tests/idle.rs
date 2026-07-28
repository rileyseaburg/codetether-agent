use super::super::*;

#[test]
fn idle_callbacks_run_after_timers_and_can_be_cancelled() {
    let result = eval_with_dom(
        "<main></main>",
        "let order=''; let cancelled=requestIdleCallback(function(){ order=order+'bad'; });\
         cancelIdleCallback(cancelled);\
         requestIdleCallback(function(d){ console.log(order+'I:'+d.didTimeout+':'\
         +d.timeRemaining()+':'+d.timeout); }, {timeout:0});\
         setTimeout(function(){ order=order+'T'; }, 0); 'sync';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["TI:true:50:0".to_string()]);
}
