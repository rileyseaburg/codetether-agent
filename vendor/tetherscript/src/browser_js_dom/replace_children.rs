use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    let method = native("replaceChildren", None, move |args| replace(&handle, args));
    obj.insert("replaceChildren".into(), method);
}

fn replace(handle: &handle_ref::HandleRef, args: &[JsValue]) -> Result<JsValue, String> {
    let parent = handle.current();
    let added = node_args::from_values(args);
    let Some(removed) = swap_children(&parent, added.clone()) else {
        return Ok(JsValue::Undefined);
    };
    if removed.is_empty() && added.is_empty() { return Ok(JsValue::Undefined); }
    queue_mutation_record(&parent, "childList", None, None, added.clone(), removed.clone());
    for node in removed {
        custom_host::disconnected(node)?;
    }
    inserted::connect(&parent, 0, added.len())?;
    Ok(JsValue::Undefined)
}

fn swap_children(parent: &DomHandle, added: Vec<Node>) -> Option<Vec<Node>> {
    let removed = {
        let mut root = parent.root.borrow_mut();
        let Some(Node::Element(el)) = get_node_mut(&mut root, &parent.path) else {
            return None;
        };
        std::mem::replace(&mut el.children, added)
    };
    detach_removed(parent, &removed);
    Some(removed)
}

fn detach_removed(parent: &DomHandle, removed: &[Node]) {
    for (index, node) in removed.iter().enumerate() {
        let detached_root = Rc::new(RefCell::new(node.clone()));
        let mut removed_path = parent.path.clone();
        removed_path.push(index);
        detach_path(&parent.root, &removed_path, detached_root);
    }
}

fn detach_path(root: &Rc<RefCell<Node>>, path: &[usize], detached: Rc<RefCell<Node>>) {
    DOM_HANDLE_REGISTRY.with(|registry| {
        for handle in registry.borrow_mut().values_mut() {
            if Rc::ptr_eq(root, &handle.root) && handle.path.starts_with(path) {
                handle.root = detached.clone();
                handle.path = handle.path[path.len()..].to_vec();
            }
        }
    });
}
