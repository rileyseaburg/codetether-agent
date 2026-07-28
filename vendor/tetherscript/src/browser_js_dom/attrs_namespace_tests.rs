use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

const XLINK: &str = "http://www.w3.org/1999/xlink";

#[test]
fn set_and_get_xlink_attribute_by_local_name() {
    let script = format!(
        "let parsed=document.querySelector('use');\
         let u=document.createElementNS('http://www.w3.org/2000/svg','use');\
         u.setAttributeNS('{XLINK}','xlink:href','#icon');\
         parsed.getAttributeNS('{XLINK}','href')+':'+u.getAttributeNS('{XLINK}','href');"
    );
    let result = eval_with_dom("<svg><use xlink:href='#old'></use></svg>", &script).unwrap();
    assert_eq!(result.value, JsValue::String("#old:#icon".into()));
}

#[test]
fn null_namespace_uses_local_attribute_name() {
    let result = eval_with_dom(
        "<div id='x'></div>",
        "let el=document.getElementById('x');\
         el.setAttributeNS(null,'data-a','1');\
         el.setAttributeNS('','data-b','2');\
         el.getAttributeNS(null,'data-a')+':'+el.getAttributeNS('', 'data-b');",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("1:2".into()));
}

#[test]
fn has_and_remove_follow_namespace_and_local_name() {
    let script = format!(
        "let el=document.createElementNS('http://www.w3.org/2000/svg','use');\
         el.setAttributeNS('{XLINK}','xlink:href','#a');\
         let before=el.hasAttributeNS('{XLINK}','href')+':'+el.hasAttributeNS(null,'href');\
         el.removeAttributeNS('{XLINK}','href');\
         before+':'+el.hasAttributeNS('{XLINK}','href')+':'+el.getAttribute('xlink:href');"
    );
    let result = eval_with_dom("<main></main>", &script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String("true:false:false:null".into())
    );
}
