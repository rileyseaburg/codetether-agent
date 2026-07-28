use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn window_control_methods_have_global_browser_shape() {
    let result = eval_with_dom(
        "<main></main>",
        "[typeof window.open,typeof open,typeof window.close,typeof close,\
         typeof print,typeof stop,typeof focus,typeof blur,window.closed,\
         window.__printCount,window.__stopped,window.__focused].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "function|function|function|function|function|function|function|function|false|0|false|true"
                .into()
        )
    );
}

#[test]
fn window_control_methods_mutate_deterministic_probes() {
    let result = eval_with_dom(
        "<main></main>",
        "print();print();stop();blur();let before=[window.__printCount,\
         window.__stopped,window.__focused,window.closed].join('|');\
         focus();window.close();before+'|'+window.__focused+'|'+window.closed;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("2|true|false|false|true|true".into())
    );
}
