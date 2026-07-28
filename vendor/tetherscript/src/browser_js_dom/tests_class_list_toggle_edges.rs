use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn class_list_toggle_force_reflects_attribute() {
    let result = eval_with_dom(
        "<div id='x' class='a b c'></div>",
        "let el=document.getElementById('x');let c=el.classList;\
         [c.toggle('d',true),c.toggle('a',true),c.toggle('b',false),\
         c.toggle('z',false),c.value,el.getAttribute('class')].join('|');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("true|true|false|false|a c d|a c d".into())
    );
}
