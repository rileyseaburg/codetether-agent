use super::super::*;

#[test]
fn promise_all_waits_for_pending_inputs_and_preserves_order() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let ra;let rb;\
         let a=new P(function(r){ra=r;});let b=new P(function(r){rb=r;});\
         P.all([a,'x',b]).then(function(v){document.getElementById('out').textContent=v.join('-');});\
         setTimeout(function(){rb('b');},0);setTimeout(function(){ra('a');},1);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("a-x-b"));
}

#[test]
fn promise_all_rejects_when_pending_input_rejects() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let reject;let p=new P(function(r,j){reject=j;});\
         P.all([1,p]).catch(function(e){document.getElementById('out').textContent=e;});\
         setTimeout(function(){reject('late-no');},0);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("late-no"));
}

#[test]
fn promise_race_can_settle_from_pending_input() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let resolve;let p=new P(function(r){resolve=r;});\
         P.race([p]).then(function(v){document.getElementById('out').textContent=v;});\
         setTimeout(function(){resolve('late-win');},0);'sync';",
    )
    .unwrap();

    assert!(result.html.contains("late-win"));
}
