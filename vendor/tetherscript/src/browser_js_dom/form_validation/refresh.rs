use super::*;

pub(super) fn write(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    let validity = check::validity(handle);
    let will_validate = match handle.node() {
        Some(Node::Element(el)) => controls::will_validate(&el),
        _ => false,
    };
    let mut obj = obj.borrow_mut();
    obj.insert("willValidate".into(), JsValue::Bool(will_validate));
    obj.insert("validity".into(), object::validity(&validity));
    obj.insert(
        "validationMessage".into(),
        JsValue::String(message::text(handle)),
    );
}
