use super::super::*;

#[test]
fn dom_exception_defaults_to_error_and_empty_message() {
    let result = eval_with_dom(
        "<main></main>",
        "let e=DOMException(); e.name+'|'+e.message+'|'+e.code+'|'+typeof window.DOMException;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("Error||0|function".into()));
}

#[test]
fn dom_exception_accepts_call_and_new_arguments() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=DOMException('blocked','SecurityError');let b=new DOMException('stop','AbortError');\
         a.name+'|'+a.message+'|'+b.name+'|'+b.message;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("SecurityError|blocked|AbortError|stop".into())
    );
}

#[test]
fn dom_exception_to_string_matches_name_message_shape() {
    let result = eval_with_dom(
        "<main></main>",
        "DOMException('blocked','SecurityError').toString()+'|'+DOMException().toString();",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("SecurityError: blocked|Error".into())
    );
}

#[test]
fn dom_exception_uses_legacy_codes_for_known_names() {
    let result = eval_with_dom(
        "<main></main>",
        "DOMException('x','SecurityError').code+'|'+DOMException('x','AbortError').code+'|'+DOMException('x','OtherError').code;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("18|20|0".into()));
}
