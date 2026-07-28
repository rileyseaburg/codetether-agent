use crate::js::JsValue;

use super::super::eval_with_dom;

#[test]
fn css_global_supports_known_and_unknown_conditions() {
    let result = eval_with_dom(
        "",
        "typeof CSS + ':' + (CSS === window.CSS) + ':' +\
         CSS.supports('display','flex') + ':' + CSS.supports('color: red') + ':' +\
         CSS.supports('(width: 1px)') + ':' + CSS.supports('(unsupported: value)') + ':' +\
         CSS.supports('unknown condition');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("object:true:true:true:true:false:false".into())
    );
}

#[test]
fn css_escape_handles_common_selector_identifiers() {
    let result = eval_with_dom(
        "",
        "CSS.escape('') + '|' + CSS.escape('a b') + '|' +\
         CSS.escape('1box') + '|' + CSS.escape('a.b#c');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("|a\\ b|\\31 box|a\\.b\\#c".into())
    );
}
