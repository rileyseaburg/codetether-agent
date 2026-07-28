use super::*;

pub(super) fn write(handle: &DomHandle, text: String) {
    let mut root = handle.root.borrow_mut();
    if let Some(path) = find_tag(&root, "title", &mut Vec::new()) {
        set_text(&mut root, &path, text);
    } else if let Some(path) = find_tag(&root, "head", &mut Vec::new()) {
        push_child(&mut root, &path, title_node(text));
    } else if let Some(path) = find_tag(&root, "html", &mut Vec::new()) {
        push_child(&mut root, &path, head_node(text));
    } else if let Node::Element(el) = &mut *root {
        el.children.insert(0, head_node(text));
    }
}

fn set_text(root: &mut Node, path: &[usize], text: String) {
    if let Some(Node::Element(el)) = get_node_mut(root, path) {
        el.children = vec![Node::Text(text)];
    }
}

fn push_child(root: &mut Node, path: &[usize], child: Node) {
    if let Some(Node::Element(el)) = get_node_mut(root, path) {
        el.children.push(child);
    }
}

fn head_node(text: String) -> Node {
    element("head", vec![title_node(text)])
}

fn title_node(text: String) -> Node {
    element("title", vec![Node::Text(text)])
}
