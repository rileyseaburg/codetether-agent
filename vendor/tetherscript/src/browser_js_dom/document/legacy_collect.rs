use super::super::super::*;
use super::matcher;

pub(super) fn by_tag(handle: &DomHandle, tag: &str) -> JsValue {
    collection(handle, &|el| matcher::tag(el, tag))
}

pub(super) fn by_class(handle: &DomHandle, class_name: &str) -> JsValue {
    let want = matcher::tokens(class_name);
    collection(handle, &|el| matcher::classes(el, &want))
}

pub(super) fn by_name(handle: &DomHandle, name: &str) -> JsValue {
    collection(handle, &|el| matcher::name(el, name))
}

fn collection(handle: &DomHandle, pred: &impl Fn(&Element) -> bool) -> JsValue {
    let mut values = Vec::new();
    collect(
        &handle.root.borrow(),
        &mut Vec::new(),
        &handle.root,
        pred,
        &mut values,
    );
    dom_collection("HTMLCollection", values)
}

fn collect(
    node: &Node,
    path: &mut Vec<usize>,
    root: &Rc<RefCell<Node>>,
    pred: &impl Fn(&Element) -> bool,
    out: &mut Vec<JsValue>,
) {
    let Node::Element(el) = node else { return };
    if pred(el) {
        out.push(ops::handle_object(root.clone(), path.clone()));
    }
    for (index, child) in el.children.iter().enumerate() {
        path.push(index);
        collect(child, path, root, pred, out);
        path.pop();
    }
}
