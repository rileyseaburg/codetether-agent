use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    obj.insert("anchors".into(), collection(handle));
}

fn collection(handle: &DomHandle) -> JsValue {
    let mut out = Vec::new();
    collect(
        &handle.root.borrow(),
        &mut Vec::new(),
        &handle.root,
        &mut out,
    );
    dom_collection("HTMLCollection", out)
}

fn collect(node: &Node, path: &mut Vec<usize>, root: &Rc<RefCell<Node>>, out: &mut Vec<JsValue>) {
    let Node::Element(el) = node else {
        return;
    };
    if el.tag == "a" && el.attrs.contains_key("name") {
        out.push(ops::handle_object(root.clone(), path.clone()));
    }
    for (index, child) in el.children.iter().enumerate() {
        path.push(index);
        collect(child, path, root, out);
        path.pop();
    }
}
