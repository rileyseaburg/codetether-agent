use std::collections::HashMap;

use crate::browser::Node;

pub(crate) fn render(attrs: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = attrs.iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(right.0));
    pairs
        .into_iter()
        .map(|(key, value)| format!("{key}={value:?}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn node(node: &Node) -> String {
    match node {
        Node::Element(element) => {
            let attrs = render(&element.attrs);
            if attrs.is_empty() {
                format!("<{}>", element.tag)
            } else {
                format!("<{} {}>", element.tag, attrs)
            }
        }
        Node::Text(value) => text(value),
    }
}
