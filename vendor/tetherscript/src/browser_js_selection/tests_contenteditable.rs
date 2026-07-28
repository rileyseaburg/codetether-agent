use super::super::{eval_with_dom, JsValue};

#[test]
fn content_editable_property_reflects_attribute() {
    let result = eval_with_dom(
        "<main><p id='a'></p><p id='b' contenteditable></p>\
         <p id='c' contenteditable='FALSE'></p><p id='d' contenteditable='spellcheck'></p></main>",
        "let a=document.getElementById('a');let b=document.getElementById('b');\
         let c=document.getElementById('c');let d=document.getElementById('d');\
         a.contentEditable+':'+a.isContentEditable+'|'+b.contentEditable+':'\
         +b.isContentEditable+'|'+c.contentEditable+':'+c.isContentEditable+'|'\
         +d.contentEditable+':'+d.isContentEditable;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("inherit:false|true:true|false:false|spellcheck:false".into())
    );
}

#[test]
fn content_editable_setter_normalizes_attribute() {
    let result = eval_with_dom(
        "<section id='e'></section>",
        "let e=document.getElementById('e');e.contentEditable='true';\
         let a=e.contentEditable+':'+e.getAttribute('contenteditable')+':'\
         +e.isContentEditable;e.contentEditable='inherit';\
         a+'|'+e.contentEditable+':'+e.hasAttribute('contenteditable')+':'\
         +e.isContentEditable;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("true:true:true|inherit:false:false".into())
    );
}

#[test]
fn is_content_editable_uses_nearest_explicit_ancestor_state() {
    let result = eval_with_dom(
        "<div id='root' contenteditable='true'><p id='child'></p>\
         <p id='off' contenteditable='false'><b id='leaf'></b></p></div>",
        "let child=document.getElementById('child');let off=document.getElementById('off');\
         let leaf=document.getElementById('leaf');\
         child.isContentEditable+':'+off.isContentEditable+':'+leaf.isContentEditable;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("true:false:false".into()));
}
