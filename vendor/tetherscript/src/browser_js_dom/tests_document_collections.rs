use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_collections_are_indexed() {
    let result = eval_with_dom(
        "<form id='f'></form><img id='i'><script id='s'></script>",
        "document.readyState+'|'+document.forms.length+'|'\
         +document.forms[0].id+'|'+(document.forms.item(1)===null)\
         +'|'+document.images[0].id+'|'+document.scripts[0].id;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("complete|1|f|true|i|s".into())
    );
}

#[test]
fn document_links_collect_anchor_and_area_href_nodes() {
    let result = eval_with_dom(
        "<a id='a' href='/a'>A</a><a id='skip'>S</a><map><area id='m' href='/m'></map>",
        "let seen='';document.links.forEach(function(n,i,c){\
         seen=seen+i+n.id+(c===document.links);});\
         document.links.length+':'+document.links.item(1).id+':'+seen;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("2:m:0atrue1mtrue".into()));
}

#[test]
fn document_anchors_collect_named_anchor_nodes() {
    let result = eval_with_dom(
        "<a id='a' name='top'></a><a id='skip' href='/x'></a><a id='b' name='bottom'></a>",
        "document.anchors.length+':'+document.anchors[0].id+':'\
         +document.anchors.item(1).getAttribute('name')+':'\
         +(document.anchors.item(2)===null);",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("2:a:bottom:true".into()));
}

#[test]
fn document_scrolling_element_matches_document_element() {
    let result = eval_with_dom(
        "<html><body><main></main></body></html>",
        "document.scrollingElement===document.documentElement &&\
         document.scrollingElement.tagName;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("HTML".into()));
}
