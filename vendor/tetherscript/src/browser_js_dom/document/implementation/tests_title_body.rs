use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_implementation_created_document_title_body_and_root_work() {
    let script = "let d=document.implementation.createHTMLDocument('Old');\
        d.title='New';let s=d.createElement('section');s.id='added';\
        d.body.appendChild(s);d.documentElement.nodeName+'|'\
        +d.head.parentNode.nodeName+'|'+d.body.children.length+'|'\
        +d.querySelector('#added').nodeName+'|'+d.title+'|'\
        +d.querySelector('title').textContent+'|'\
        +(document.querySelector('section')===null);";
    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("html|html|1|section|New|New|true".into())
    );
}
