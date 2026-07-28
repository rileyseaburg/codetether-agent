use crate::browser::Node;

pub(crate) fn child(base: &str, index: usize, node: &Node) -> String {
    let label = match node {
        Node::Element(element) => element.tag.as_str(),
        Node::Text(_) => "#text",
    };
    format!("{base}/{label}[{index}]")
}
