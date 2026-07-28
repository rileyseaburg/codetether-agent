use super::super::*;

#[test]
fn picker_functions_are_window_and_bare_globals() {
    let result = eval_with_dom(
        "<main></main>",
        "typeof showOpenFilePicker+'|'+(showOpenFilePicker===window.showOpenFilePicker)+'|'\
         +typeof showSaveFilePicker+'|'+typeof showDirectoryPicker;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|true|function|function".into())
    );
}

#[test]
fn picker_calls_return_async_rejected_thenables() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let seen='';let p=showOpenFilePicker();\
         let next=p.catch(function(e){seen=e.name+':'+e.message;return 'handled';});\
         setTimeout(function(){document.getElementById('out').textContent=\
         [seen,next.__promise_state,next.__promise_value].join('|');},0);\
         [p.__promise_state,seen,next.__promise_state].join('|');",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("rejected||pending".into()));
    assert!(result
        .html
        .contains("NotAllowedError:file picker unsupported|fulfilled|handled"));
}

#[test]
fn picker_then_without_rejection_handler_rejects_async() {
    let result = eval_with_dom(
        "<p id='out'></p>",
        "let p=showSaveFilePicker();let next=p.then(null);\
         setTimeout(function(){document.getElementById('out').textContent=\
         next.__promise_state+'|'+next.__promise_reason.name;},0);\
         next.__promise_state;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("pending".into()));
    assert!(result.html.contains("rejected|NotAllowedError"));
}
