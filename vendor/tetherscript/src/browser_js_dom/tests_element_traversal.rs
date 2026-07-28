use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn element_traversal_skips_text_nodes() {
    let result = eval_with_dom(
        "<div id='app'>A<span id='a'>A</span>B<em id='b'>B</em>C</div>",
        "let app=document.getElementById('app');let a=document.getElementById('a');\
         let b=document.getElementById('b');app.firstElementChild.id+'|'\
         +app.lastElementChild.id+'|'+a.nextElementSibling.id+'|'\
         +b.previousElementSibling.id+'|'+(a.previousElementSibling===null)\
         +'|'+(b.nextElementSibling===null);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("a|b|b|a|true|true".into()));
}

#[test]
fn element_remove_detaches_from_parent() {
    let result = eval_with_dom(
        "<main id='app'><span>A</span><b id='b'>B</b></main>",
        "let app=document.getElementById('app');let b=document.getElementById('b');\
         b.remove();document.getElementById('app').textContent+':'\
         +document.getElementById('b')+':'+app.contains(b);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("A:null:false".into()));
    assert!(!result.html.contains("<b id=\"b\">B</b>"));
}
