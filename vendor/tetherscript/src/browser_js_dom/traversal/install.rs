use super::*;

pub(super) fn window(window: &mut HashMap<String, JsValue>) {
    window.insert("NodeFilter".into(), node_filter_object());
}

pub(super) fn node(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    if !matches!(node, Node::Element(el) if el.tag == "#document") {
        return;
    }
    obj.insert(
        "createTreeWalker".into(),
        create_method(handle, TraversalKind::TreeWalker),
    );
    obj.insert(
        "createNodeIterator".into(),
        create_method(handle, TraversalKind::NodeIterator),
    );
}

fn create_method(document: &DomHandle, kind: TraversalKind) -> JsValue {
    let root = document.root.clone();
    let name = match kind {
        TraversalKind::TreeWalker => "createTreeWalker",
        TraversalKind::NodeIterator => "createNodeIterator",
    };
    native(name, None, move |args| {
        let root_handle = root_arg(name, args.first(), root.clone())?;
        let mask = show_mask(args.get(1));
        let filter = args.get(2).cloned().unwrap_or(JsValue::Null);
        let paths = collect_paths(&root_handle, mask, &filter, kind)?;
        Ok(traversal_object(root_handle, paths, kind))
    })
}
