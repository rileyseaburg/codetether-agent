use super::*;

#[path = "style_index.rs"]
mod style_index;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    if !matches!(node, Node::Element(el) if !el.tag.starts_with('#')) {
        return;
    }
    obj.insert("style".into(), object(handle_ref::new(obj, handle)));
}

fn object(handle: handle_ref::HandleRef) -> JsValue {
    let obj = Rc::new(RefCell::new(HashMap::new()));
    style_refresh::update(&obj, &handle);
    style_methods::install(&obj, handle.clone());
    style_index::install(&obj, handle);
    JsValue::Object(obj)
}
