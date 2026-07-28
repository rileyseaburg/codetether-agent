use super::super::*;

#[test]
fn finally_waits_for_pending_callback_rejection() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let resolve;let reject_cleanup;\
         let p=new P(function(r){resolve=r;});\
         let cleanup=new P(function(r,j){reject_cleanup=j;});\
         p.finally(function(){return cleanup;}).catch(function(e){\
         document.getElementById('out').textContent=e;});\
         setTimeout(function(){resolve('ok');},0);\
         setTimeout(function(){reject_cleanup('cleanup-failed');},1);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("cleanup-failed"));
}

#[test]
fn finally_waits_for_pending_callback_before_original_rejection() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let reject;let resolve_cleanup;\
         let p=new P(function(r,j){reject=j;});\
         let cleanup=new P(function(r){resolve_cleanup=r;});\
         p.finally(function(){return cleanup;}).catch(function(e){\
         document.getElementById('out').textContent=e;});\
         setTimeout(function(){reject('original');},0);\
         setTimeout(function(){resolve_cleanup('done');},1);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("original"));
}
