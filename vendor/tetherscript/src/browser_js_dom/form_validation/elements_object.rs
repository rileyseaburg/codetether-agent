use super::*;

pub(super) fn create(handle: &DomHandle) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    elements_methods::install(&object, handle);
    elements_sync::write(&object, handle);
    JsValue::Object(object)
}
