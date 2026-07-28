use super::*;

pub(super) fn remove_empty_text(root: &Rc<RefCell<Node>>, path: &[usize], index: usize) -> bool {
    let mut tree = root.borrow_mut();
    let Some(Node::Element(el)) = get_node_mut(&mut tree, path) else {
        return false;
    };
    if !matches!(el.children.get(index), Some(Node::Text(text)) if text.is_empty()) {
        return false;
    }
    let removed = el.children.remove(index);
    drop(tree);
    adjust_dom_handles_after_remove(root, path, index, &removed);
    true
}

pub(super) fn merge_adjacent_text(root: &Rc<RefCell<Node>>, path: &[usize], index: usize) -> bool {
    let mut tree = root.borrow_mut();
    let Some(Node::Element(el)) = get_node_mut(&mut tree, path) else {
        return false;
    };
    let Some(right) = right_text(&el.children, index) else {
        return false;
    };
    let Some(Node::Text(left)) = el.children.get_mut(index) else {
        return false;
    };
    left.push_str(&right);
    let removed = el.children.remove(index + 1);
    drop(tree);
    adjust_dom_handles_after_remove(root, path, index + 1, &removed);
    true
}

fn right_text(children: &[Node], index: usize) -> Option<String> {
    match (children.get(index), children.get(index + 1)) {
        (Some(Node::Text(_)), Some(Node::Text(right))) => Some(right.clone()),
        _ => None,
    }
}
