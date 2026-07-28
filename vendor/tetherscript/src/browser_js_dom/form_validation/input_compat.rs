use super::*;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let Some(Node::Element(el)) = handle.node() else {
        return;
    };
    if el.tag != "input" {
        return;
    }
    input_attrs::install(obj, handle, &el);
    input_indeterminate::install(obj);
    input_number::install(obj, handle, &el);
    input_step::install(obj, handle);
}
