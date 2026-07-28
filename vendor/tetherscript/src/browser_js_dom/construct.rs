use super::*;

#[path = "construct/html.rs"]
mod html;
#[path = "construct/raw.rs"]
mod raw;

pub(super) fn document_from_html(html: &str) -> JsValue {
    ops::document_object(html::root(html))
}

pub(super) fn document_from_markup(markup: &str) -> JsValue {
    ops::document_object(raw::root(markup))
}

pub(super) fn fragment_from_children(children: Vec<Node>) -> JsValue {
    ops::detached_object(Node::Element(Element {
        tag: "#document-fragment".into(),
        attrs: HashMap::new(),
        children,
    }))
}
