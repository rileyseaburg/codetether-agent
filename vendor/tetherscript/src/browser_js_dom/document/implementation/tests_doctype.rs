use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_implementation_doctype_fields_are_deterministic() {
    let script = "let t=document.doctype;t.name+'|'+t.nodeType+'|'\
        +t.nodeName+'|'+t.publicId+'|'+t.systemId+'|'\
        +(t===document.documentType);";
    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(result.value, JsValue::String("html|10|html|||true".into()));
}
