use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn fresh_attributes_read_reflects_attribute_mutation() {
    let result = eval_with_dom(
        "<div id='x'></div>",
        "let el=document.getElementById('x');let before=el.attributes.length;\
         el.setAttribute('data-a','1');\
         let added=document.getElementById('x').attributes;\
         el.removeAttribute('id');\
         let removed=document.querySelector('div').attributes;\
         before+':'+added.length+':'+added.getNamedItem('data-a').value\
         +':'+removed.length+':'+removed.getNamedItem('id');",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("1:2:1:1:null".into()));
}
