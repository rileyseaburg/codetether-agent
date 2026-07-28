use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    obj.insert("isConnected".into(), JsValue::Bool(connected(handle)));
    install_contains(obj, handle);
    install_same_node(obj, handle);
}

fn install_contains(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "contains".into(),
        native("Node.contains", Some(1), move |args| {
            let Some(other) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Bool(false));
            };
            Ok(JsValue::Bool(contains(&handle.current(), &other)))
        }),
    );
}

fn install_same_node(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let handle = handle_ref::new(obj, handle);
    obj.insert(
        "isSameNode".into(),
        native("Node.isSameNode", Some(1), move |args| {
            let Some(other) = args.first().and_then(dom_handle_from_value) else {
                return Ok(JsValue::Bool(false));
            };
            Ok(JsValue::Bool(same_node(&handle.current(), &other)))
        }),
    );
}

pub(super) fn connected(handle: &DomHandle) -> bool {
    handle.node().is_some()
        && matches!(&*handle.root.borrow(), Node::Element(el) if el.tag == "#document")
}

pub(super) fn contains(parent: &DomHandle, child: &DomHandle) -> bool {
    same_tree(parent, child)
        && parent.node().is_some()
        && child.node().is_some()
        && child.path.starts_with(&parent.path)
}

pub(super) fn same_node(left: &DomHandle, right: &DomHandle) -> bool {
    same_tree(left, right) && left.path == right.path && left.node().is_some()
}

pub(super) fn same_tree(left: &DomHandle, right: &DomHandle) -> bool {
    Rc::ptr_eq(&left.root, &right.root)
}
