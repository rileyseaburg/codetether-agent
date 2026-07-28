use super::*;

pub(super) fn from_values(values: &[JsValue]) -> Vec<Node> {
    values
        .iter()
        .flat_map(|value| expand(js_value_to_node(value)))
        .collect()
}

pub(super) fn from_html(source: &str) -> Vec<Node> {
    browser::parse_html(source)
        .children
        .into_iter()
        .flat_map(expand)
        .collect()
}

pub(super) fn from_text(text: String) -> Vec<Node> {
    vec![Node::Text(text)]
}

fn expand(node: Node) -> Vec<Node> {
    match node {
        Node::Element(el) if el.tag == "#document-fragment" => el.children,
        node => vec![node],
    }
}
