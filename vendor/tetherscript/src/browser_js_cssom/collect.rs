use crate::browser::{Document, Node};

pub(super) fn style_sources(document: &Document) -> Vec<String> {
    let mut sources = Vec::new();
    for node in &document.children {
        collect_node(node, &mut sources);
    }
    sources
}

fn collect_node(node: &Node, sources: &mut Vec<String>) {
    let Node::Element(element) = node else {
        return;
    };
    if element.tag == "style" {
        sources.push(text_children(&element.children));
        return;
    }
    for child in &element.children {
        collect_node(child, sources);
    }
}

fn text_children(children: &[Node]) -> String {
    let mut text = String::new();
    for child in children {
        if let Node::Text(value) = child {
            text.push_str(value);
        }
    }
    text
}
