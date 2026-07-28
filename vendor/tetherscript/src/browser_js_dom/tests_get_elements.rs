use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn get_elements_by_tag_and_wildcard_are_ordered() {
    let result = eval_with_dom(
        "<main id='m'><span id='a'></span><span id='b'></span></main>",
        "let spans=document.getElementsByTagName('span');\
         let all=document.getElementsByTagName('*');\
         spans.length+':'+spans[0].id+spans.item(1).id+':'\
         +all.length+':'+all.item(0).tagName+':'+all.item(2).id+':'\
         +(all.item(3)===null);",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("2:ab:3:MAIN:b:true".into()));
}

#[test]
fn get_elements_by_class_requires_all_tokens() {
    let result = eval_with_dom(
        "<p id='a' class='one two'></p><p id='b' class='one'></p>\
         <p id='c' class='two one three'></p>",
        "let both=document.getElementsByClassName('one two');\
         both.length+':'+both[0].id+both.item(1).id+':'\
         +document.getElementsByClassName(' ').length;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("2:ac:0".into()));
}

#[test]
fn get_elements_by_name_matches_name_attribute() {
    let result = eval_with_dom(
        "<input id='a' name='q'><button id='b' name='q'></button>\
         <input id='c' name='other'>",
        "let named=document.getElementsByName('q');\
         named.length+':'+named[0].id+named.item(1).id+':'\
         +(named.item(2)===null);",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("2:ab:true".into()));
}
