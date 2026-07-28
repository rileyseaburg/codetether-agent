use crate::js::JsValue;

use super::eval_with_dom;

#[test]
fn style_sheets_expose_rules_and_declarations() {
    let result = eval_with_dom(
        "<style>#box{color:red;width:2px}</style><main id='box'></main>",
        "let sheet=document.styleSheets[0];\
         let rule=sheet.cssRules[0];\
         document.styleSheets.length + ':' + sheet.cssRules.length + ':' +\
         rule.selectorText + ':' + rule.style.getPropertyValue('color') + ':' +\
         rule.cssText;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("1:1:#box:red:#box { color: red; width: 2px }".into())
    );
}

#[test]
fn insert_and_delete_rule_update_later_computed_style() {
    let result = eval_with_dom(
        "<style>#box { color: red }</style><main id='box'></main>",
        "let box=document.getElementById('box');\
         let sheet=document.styleSheets[0];\
         let before=getComputedStyle(box).getPropertyValue('color');\
         let idx=sheet.insertRule('#box { color: blue; width: 7px }', sheet.cssRules.length);\
         let after=getComputedStyle(box).getPropertyValue('color');\
         let width=getComputedStyle(box).getPropertyValue('width');\
         let added=sheet.cssRules[idx].selectorText + ':' + sheet.cssRules[idx].style.width;\
         sheet.deleteRule(idx);\
         before + ':' + added + ':' + after + ':' + width + ':' +\
         sheet.cssRules.length + ':' + getComputedStyle(box).getPropertyValue('color');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("red:#box:7px:blue:7px:1:red".into())
    );
}
