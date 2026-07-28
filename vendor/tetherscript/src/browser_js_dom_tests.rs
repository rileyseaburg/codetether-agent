use super::*;

#[path = "browser_js_dom/tests_adjacent_element.rs"]
mod tests_adjacent_element;
#[path = "browser_js_dom/tests_attrs_style.rs"]
mod tests_attrs_style;
#[path = "browser_js_dom/tests_class_list.rs"]
mod tests_class_list;
#[path = "browser_js_dom/tests_class_list_iteration.rs"]
mod tests_class_list_iteration;
#[path = "browser_js_dom/tests_collections.rs"]
mod tests_collections;
#[path = "browser_js_dom/tests_dialog_popover.rs"]
mod tests_dialog_popover;
#[path = "browser_js_dom/tests_document_collections.rs"]
mod tests_document_collections;
#[path = "browser_js_dom/tests_document_title.rs"]
mod tests_document_title;
#[path = "browser_js_dom/tests_dom_parser.rs"]
mod tests_dom_parser;
#[path = "browser_js_dom/tests_element_traversal.rs"]
mod tests_element_traversal;
#[path = "browser_js_dom/tests_file_input.rs"]
mod tests_file_input;
#[path = "browser_js_dom/tests_form_validation.rs"]
mod tests_form_validation;
#[path = "browser_js_dom/tests_get_elements.rs"]
mod tests_get_elements;
#[path = "browser_js_dom/tests_mutation.rs"]
mod tests_mutation;
#[path = "browser_js_dom/tests_range.rs"]
mod tests_range;
#[path = "browser_js_dom/tests_reflected_attrs.rs"]
mod tests_reflected_attrs;
#[path = "browser_js_dom/tests_style_declaration.rs"]
mod tests_style_declaration;

#[test]
fn xml_serializer_serializes_nodes_and_documents() {
    let result = eval_with_dom("<main></main>", "let d=DOMParser().parseFromString('<section><b>A&B</b></section>', 'text/html'); XMLSerializer().serializeToString(d.body.firstChild) + '|' + XMLSerializer().serializeToString(d);").unwrap();
    assert_eq!(result.value, JsValue::String("<section><b>A&amp;B</b></section>|<html><head></head><body><section><b>A&amp;B</b></section></body></html>".into()));
}

#[test]
fn template_content_behaves_like_a_fragment() {
    let result = eval_with_dom("<main></main>", "let t=document.createElement('template'); let s=document.createElement('span'); s.textContent='A'; t.content.appendChild(s); document.body.appendChild(t.content.cloneNode(true)); document.body.textContent + ':' + t.content.childNodes.length;").unwrap();
    assert_eq!(result.value, JsValue::String("A:1".into()));
}

#[test]
fn import_node_clones_and_adopt_node_detaches() {
    let result = eval_with_dom("<main></main>", "let d=DOMParser().parseFromString('<div><em>A</em></div>', 'text/html'); let imported=document.importNode(d.querySelector('div'), true); document.body.appendChild(imported); let em=d.querySelector('em'); document.body.appendChild(document.adoptNode(em)); d.querySelector('em') + ':' + document.body.textContent;").unwrap();
    assert_eq!(result.value, JsValue::String("null:AA".into()));
}
