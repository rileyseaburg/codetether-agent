use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "normalize".into(),
        native("Node.normalize", Some(0), move |_| {
            let handle = handle.current();
            if handle.node().is_some() {
                normalize_at(&handle.root, &handle.path);
            }
            Ok(JsValue::Undefined)
        }),
    );
}

fn normalize_at(root: &Rc<RefCell<Node>>, path: &[usize]) {
    for index in 0..child_len(root, path) {
        let mut child_path = path.to_vec();
        child_path.push(index);
        normalize_at(root, &child_path);
    }
    normalize_children(root, path);
}

fn child_len(root: &Rc<RefCell<Node>>, path: &[usize]) -> usize {
    match get_node(&root.borrow(), path) {
        Some(Node::Element(el)) => el.children.len(),
        _ => 0,
    }
}

fn normalize_children(root: &Rc<RefCell<Node>>, path: &[usize]) {
    let mut index = 0;
    while index < child_len(root, path) {
        if normalize_edit::remove_empty_text(root, path, index) {
            continue;
        }
        if normalize_edit::merge_adjacent_text(root, path, index) {
            continue;
        }
        index += 1;
    }
}
