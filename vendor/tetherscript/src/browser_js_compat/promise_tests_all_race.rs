use super::super::*;

#[test]
fn promise_all_unwraps_mixed_values_and_thenables() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let out='';\
         P.all([1,P.resolve(2),Blob(['x']).text()]).then(function(v){\
         out=v.join('-');document.getElementById('out').textContent=out;});out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("1-2-x"));
}

#[test]
fn promise_all_rejects_when_any_input_is_rejected() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let P=window.Promise;let out='';\
         P.all([1,P.reject('no'),P.resolve(3)]).catch(function(e){\
         out=e;document.getElementById('out').textContent=out;});out;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("".into()));
    assert!(result.html.contains("no"));
}

#[test]
fn promise_race_settles_to_first_item() {
    let result = eval_with_dom(
        "<p id='a'></p><p id='b'></p>",
        "let P=window.Promise;let a='';let b='';\
         P.race(['first',P.resolve('later')]).then(function(v){\
         a=v;document.getElementById('a').textContent=a;});\
         P.race([P.reject('bad'),P.resolve('ok')]).catch(function(e){\
         b=e;document.getElementById('b').textContent=b;});a+':'+b;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String(":".into()));
    assert!(result.html.contains("first"));
    assert!(result.html.contains("bad"));
}
