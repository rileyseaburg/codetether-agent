use crate::{browser_js::eval_with_dom, js::JsValue};

#[test]
fn url_object_url_static_helpers_are_present() {
    let result = eval_with_dom(
        "<main></main>",
        "typeof URL.createObjectURL + ':' + typeof URL.revokeObjectURL;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("function:function".into()));
}

#[test]
fn url_create_object_url_returns_unique_blob_urls() {
    let result = eval_with_dom(
        "<main></main>",
        "let first=URL.createObjectURL(Blob(['a']));\
         let second=URL.createObjectURL(Blob(['b']));\
         first + '|' + second + '|' + (first !== second);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("blob:tetherscript://blob/1|blob:tetherscript://blob/2|true".into())
    );
}

#[test]
fn url_revoke_object_url_returns_undefined() {
    let result = eval_with_dom(
        "<main></main>",
        "let url=URL.createObjectURL(Blob(['x']));\
         '' + URL.revokeObjectURL(url);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("undefined".into()));
}
