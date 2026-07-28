use super::*;

pub(super) fn root(html: &str) -> Rc<RefCell<Node>> {
    let mut doc = browser::parse_html(html);
    doc.children = normalize_html_children(doc.children);
    Rc::new(RefCell::new(Node::Element(Element {
        tag: "#document".into(),
        attrs: HashMap::new(),
        children: doc.children,
    })))
}

fn normalize_html_children(mut nodes: Vec<Node>) -> Vec<Node> {
    if nodes.iter().any(|node| has_tag(node, "html")) {
        return nodes;
    }
    let head = take_tag(&mut nodes, "head").unwrap_or_else(|| element("head", Vec::new()));
    let mut body = take_tag(&mut nodes, "body").unwrap_or_else(|| element("body", Vec::new()));
    if let Node::Element(el) = &mut body {
        el.children.append(&mut nodes);
    }
    vec![element("html", vec![head, body])]
}

fn take_tag(nodes: &mut Vec<Node>, tag: &str) -> Option<Node> {
    let index = nodes.iter().position(|node| has_tag(node, tag))?;
    Some(nodes.remove(index))
}

fn has_tag(node: &Node, tag: &str) -> bool {
    matches!(node, Node::Element(el) if el.tag == tag)
}

fn element(tag: &str, children: Vec<Node>) -> Node {
    Node::Element(Element {
        tag: tag.into(),
        attrs: HashMap::new(),
        children,
    })
}
