use super::super::*;

#[test]
fn performance_observer_receives_user_timing_records() {
    let result = eval_with_dom(
        "<main></main>",
        "let obs=PerformanceObserver(function(list){ let e=list.getEntries()[0];\
         console.log(e.entryType+':'+e.name); });\
         obs.observe({entryTypes:['mark','measure']}); performance.mark('ready'); 'sync';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert_eq!(result.console, vec!["mark:ready".to_string()]);
}
