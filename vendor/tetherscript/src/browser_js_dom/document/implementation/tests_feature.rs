use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_implementation_has_feature_returns_true() {
    let result = eval_with_dom(
        "<main></main>",
        "document.implementation.hasFeature() &&\
         document.implementation.hasFeature('HTML','1.0');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}
