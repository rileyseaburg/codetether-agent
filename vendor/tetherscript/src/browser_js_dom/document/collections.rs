use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    obj.insert("readyState".into(), JsValue::String("complete".into()));
    obj.insert("forms".into(), selector_collection(handle, "form"));
    obj.insert("images".into(), selector_collection(handle, "img"));
    obj.insert("scripts".into(), selector_collection(handle, "script"));
    obj.insert("links".into(), link_collection(handle));
}

fn selector_collection(handle: &DomHandle, selector: &str) -> JsValue {
    let values = all_by_selector(&handle.root, selector)
        .into_iter()
        .map(|path| ops::handle_object(handle.root.clone(), path))
        .collect();
    dom_collection("HTMLCollection", values)
}

fn link_collection(handle: &DomHandle) -> JsValue {
    let mut out = Vec::new();
    collect_links(
        &handle.root.borrow(),
        &mut Vec::new(),
        &handle.root,
        &mut out,
    );
    dom_collection("HTMLCollection", out)
}

fn collect_links(
    node: &Node,
    path: &mut Vec<usize>,
    root: &Rc<RefCell<Node>>,
    out: &mut Vec<JsValue>,
) {
    let Node::Element(el) = node else {
        return;
    };
    if matches!(el.tag.as_str(), "a" | "area") && el.attrs.contains_key("href") {
        out.push(ops::handle_object(root.clone(), path.clone()));
    }
    for (index, child) in el.children.iter().enumerate() {
        path.push(index);
        collect_links(child, path, root, out);
        path.pop();
    }
}
