//! HTML serialization helpers for inlined deterministic resources.

use crate::browser::{Document, Element, Node};

pub(crate) fn document_html(document: &Document) -> String {
    document.children.iter().map(outer_html).collect()
}

fn outer_html(node: &Node) -> String {
    match node {
        Node::Text(text) => escape_html(text),
        Node::Element(element) => element_html(element),
    }
}

fn element_html(element: &Element) -> String {
    let mut out = format!("<{}", element.tag);
    let mut attrs = element.attrs.iter().collect::<Vec<_>>();
    attrs.sort_by(|left, right| left.0.cmp(right.0));
    for (name, value) in attrs {
        out.push_str(&format!(" {}=\"{}\"", name, escape_attr(value)));
    }
    out.push('>');
    out.push_str(&children_html(element));
    out.push_str(&format!("</{}>", element.tag));
    out
}

fn children_html(element: &Element) -> String {
    if matches!(element.tag.as_str(), "script" | "style") {
        return element.children.iter().map(raw_text).collect();
    }
    element.children.iter().map(outer_html).collect()
}

fn raw_text(node: &Node) -> String {
    match node {
        Node::Text(text) => text.clone(),
        Node::Element(element) => element.children.iter().map(raw_text).collect(),
    }
}

fn escape_attr(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
