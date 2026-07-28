use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_implementation_create_html_document_normalizes_tree() {
    let script = "let d=document.implementation.createHTMLDocument('Hello');\
        d.documentElement.nodeName+'|'+d.head.nodeName+'|'+d.body.nodeName+'|'\
        +d.title+'|'+d.querySelector('title').textContent+'|'\
        +document.querySelector('main').id;";
    let result = eval_with_dom("<main id='live'></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String("html|head|body|Hello|Hello|live".into())
    );
}
