use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn class_list_value_and_to_string_reflect_attribute() {
    let result = eval_with_dom(
        "<div id='x' class='a b c'></div>",
        "let el=document.getElementById('x');let c=el.classList;\
         let before=c.value+'|'+c.toString();c.value='x y x';\
         before+'|'+c.length+'|'+c[0]+','+c[1]+','+(c[2]===undefined)\
         +'|'+c.toString()+'|'+el.getAttribute('class');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("a b c|a b c|2|x,y,true|x y|x y".into())
    );
}
