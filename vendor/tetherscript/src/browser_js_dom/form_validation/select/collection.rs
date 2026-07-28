use super::super::*;

pub(super) fn create(select: &DomHandle) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    super::collection_methods::install(&object, select);
    super::collection_sync::write(&object, select);
    JsValue::Object(object)
}
