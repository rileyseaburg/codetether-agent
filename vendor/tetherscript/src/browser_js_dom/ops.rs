use super::*;

pub(super) fn document_object(root: Rc<RefCell<Node>>) -> JsValue {
    node_object(DomHandle {
        root,
        path: Vec::new(),
    })
}

pub(super) fn handle_object(root: Rc<RefCell<Node>>, path: Vec<usize>) -> JsValue {
    node_object(DomHandle { root, path })
}

pub(super) fn detached_object(node: Node) -> JsValue {
    detached_node_object(node)
}

pub(super) fn clone_for_import(value: &JsValue, deep: bool) -> Node {
    clone_node(&js_value_to_node(value), deep)
}

pub(super) fn serialize_value(value: &JsValue) -> String {
    if let Some(handle) = dom_handle_from_value(value) {
        return handle.node().map_or(String::new(), |node| serialize(&node));
    }
    serialize(&js_value_to_node(value))
}

fn serialize(node: &Node) -> String {
    match node {
        Node::Element(el) if el.tag == "#document" || el.tag == "#document-fragment" => {
            inner_html(node)
        }
        _ => outer_html(node),
    }
}
