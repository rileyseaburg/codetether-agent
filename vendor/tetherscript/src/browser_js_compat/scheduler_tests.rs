use super::super::super::super::*;

fn run(script: &str) -> BrowserJsResult {
    eval_with_dom("<p id='out'></p>", script).unwrap()
}

#[test]
fn scheduler_is_window_and_bare_global() {
    let result = run("typeof scheduler + ':' + (scheduler === window.scheduler);");
    assert_eq!(result.value, JsValue::String("object:true".into()));
}

#[test]
fn post_task_runs_callback_and_resolves_return_value() {
    let result = run("scheduler.postTask(function(){ return 7; },\
         {priority:'user-visible'}).then(function(v){\
         document.getElementById('out').textContent='v'+v; });'sync';");
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("v7"));
}

#[test]
fn post_task_without_callback_resolves_undefined() {
    let result = run("scheduler.postTask().then(function(v){\
         document.getElementById('out').textContent=typeof v; });'sync';");
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("undefined"));
}

#[test]
fn yield_returns_resolved_thenable() {
    let result = run("scheduler.yield().then(function(v){\
         document.getElementById('out').textContent=typeof v; });'sync';");
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("undefined"));
}
