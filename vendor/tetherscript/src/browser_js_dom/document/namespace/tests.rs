use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_namespace_elements_expose_html_and_svg_names() {
    let script = "let h=document.createElementNS('http://www.w3.org/1999/xhtml','section');\
        let s=document.createElementNS('http://www.w3.org/2000/svg','svg:rect');\
        h.namespaceURI+'|'+h.localName+'|'+(h.prefix===null)+'|'\
        +h.nodeName+'|'+h.tagName+'|'\
        +s.namespaceURI+'|'+s.localName+'|'+s.prefix+'|'\
        +s.nodeName+'|'+s.tagName;";
    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "http://www.w3.org/1999/xhtml|section|true|SECTION|SECTION|\
             http://www.w3.org/2000/svg|rect|svg|svg:rect|svg:rect"
                .into()
        )
    );
}

#[test]
fn document_namespace_attrs_expose_attr_fields() {
    let script = "let a=document.createAttribute('data-x');\
        let n=document.createAttributeNS('urn:test','p:item');\
        a.name+'|'+a.nodeName+'|'+a.nodeType+'|'+a.value+'|'+a.nodeValue\
        +'|'+a.specified+'|'+(a.namespaceURI===null)+'|'+a.localName+'|'\
        +(a.prefix===null)+'|'+n.name+'|'+n.nodeName+'|'+n.nodeType+'|'\
        +n.value+'|'+n.nodeValue+'|'+n.specified+'|'\
        +n.namespaceURI+'|'+n.localName+'|'+n.prefix;";
    let result = eval_with_dom("<main></main>", script).unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "data-x|data-x|2|||true|true|data-x|true|p:item|p:item|2|||true|urn:test|item|p".into()
        )
    );
}
