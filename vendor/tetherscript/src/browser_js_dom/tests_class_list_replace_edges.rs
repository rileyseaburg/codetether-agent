use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn class_list_replace_returns_and_syncs_for_edge_cases() {
    let result = eval_with_dom(
        "<div id='x' class='a b c'></div>",
        "let el=document.getElementById('x');let c=el.classList;\
         [c.replace('b','d'),c.replace('x','q'),c.replace('a','d'),\
         c.replace('d','d'),c.value,el.getAttribute('class')].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("true|false|true|true|d c|d c".into())
    );
}
