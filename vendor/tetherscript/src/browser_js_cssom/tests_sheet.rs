use crate::js::JsValue;

use super::super::eval_with_dom;

#[test]
fn constructable_stylesheet_rules_can_be_mutated() {
    let result = eval_with_dom(
        "",
        "let sheet = new CSSStyleSheet();\
         let index = sheet.insertRule('#x { color: red; width: 1px }');\
         let before = sheet.cssRules[index].style.color + ':' + sheet.cssRules.length;\
         sheet.deleteRule(index);\
         before + ':' + sheet.cssRules.length;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("red:1:0".into()));
}
