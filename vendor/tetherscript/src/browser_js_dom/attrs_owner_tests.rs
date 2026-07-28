use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn attribute_node_exposes_owner_element() {
    let result = eval_with_dom(
        "<div id='x'></div>",
        "let attr=document.getElementById('x').attributes.getNamedItem('id');\
         attr.nodeType+':'+attr.specified+':'+attr.ownerElement.id\
         +':'+attr.ownerElement.nodeName;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("2:true:x:div".into()));
}
