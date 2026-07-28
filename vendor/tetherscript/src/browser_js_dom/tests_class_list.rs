use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn class_list_preserves_order_and_syncs_attribute() {
    let result = eval_with_dom(
        "<div id='x' class='one two'></div>",
        "let el=document.getElementById('x');let c=el.classList;\
         c.add('three','two','four');c.remove('two');\
         c.value+'|'+el.getAttribute('class');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("one three four|one three four".into())
    );
}

#[test]
fn class_list_exposes_length_item_and_value() {
    let result = eval_with_dom(
        "<div id='x' class='alpha beta gamma'></div>",
        "let c=document.getElementById('x').classList;\
         c.length+':'+c.item(0)+':'+c.item(2)+':'+c.item(3)+':'+c.value;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("3:alpha:gamma:null:alpha beta gamma".into())
    );
}

#[test]
fn class_list_replace_returns_whether_old_token_existed() {
    let result = eval_with_dom(
        "<div id='x' class='a b c'></div>",
        "let el=document.getElementById('x');let c=el.classList;\
         let a=c.replace('b','d');let b=c.replace('z','q');\
         a+':'+b+':'+c.value+':'+el.getAttribute('class');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("true:false:a d c:a d c".into())
    );
}

#[test]
fn class_list_add_and_remove_accept_multiple_tokens() {
    let result = eval_with_dom(
        "<div id='x' class='root'></div>",
        "let c=document.getElementById('x').classList;\
         c.add('one','two','one');c.remove('root','two');\
         c.length+':'+c.item(0)+':'+c.value;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("1:one:one".into()));
}
