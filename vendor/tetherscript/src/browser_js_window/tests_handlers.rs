use super::super::eval_with_dom;
use crate::js::JsValue;

#[test]
fn window_lifecycle_handler_properties_have_global_shape() {
    let result = eval_with_dom(
        "<main></main>",
        "[typeof window.onpagehide,typeof onpagehide,typeof window.onpageshow,\
         typeof window.onvisibilitychange,typeof window.ononline,typeof window.onoffline,\
         typeof window.onresize,typeof window.onscroll,typeof scrollTo,typeof window.scrollBy].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "object|object|object|object|object|object|object|object|function|function".into()
        )
    );
}

#[test]
fn top_window_aliases_have_browser_shape() {
    let result = eval_with_dom(
        "<main></main>",
        "[globalThis===window,self===window,top===window,parent===window,\
         frames===window,window.opener===null,window.closed,window.name,\
         window.length,length].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("true|true|true|true|true|true|false||0|0".into())
    );
}
