use super::super::*;

#[test]
fn promise_constructor_resolve_path_runs_handler() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let out='sync';\
         new P(function(resolve,reject){resolve('ok');}).then(function(v){out=v;document.getElementById('out').textContent=out;});out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("ok"));
}

#[test]
fn promise_constructor_reject_path_runs_handler() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let out='sync';\
         P(function(resolve,reject){reject('no');}).catch(function(e){out=e;document.getElementById('out').textContent=out;});out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("sync".into()));
    assert!(result.html.contains("no"));
}

#[test]
fn promise_pending_handlers_run_after_later_settlement() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let resolve;let p=new P(function(r){resolve=r;});\
         p.then(function(v){document.getElementById('out').textContent='then:'+v;});\
         setTimeout(function(){resolve('late');},0);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("then:late"));
}

#[test]
fn promise_pending_catch_runs_after_later_rejection() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let reject;let p=new P(function(r,j){reject=j;});\
         p.catch(function(e){document.getElementById('out').textContent='catch:'+e;});\
         setTimeout(function(){reject('late');},0);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("catch:late"));
}
