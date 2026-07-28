use super::*;

pub(crate) fn create(document_object: &JsValue) -> JsValue {
    let mut object = HashMap::new();
    object.insert("get".into(), read::get());
    object.insert("getAll".into(), read::get_all());
    object.insert("set".into(), write::set(document_object.clone()));
    object.insert("delete".into(), write::delete(document_object.clone()));
    events::install(&mut object);
    JsValue::Object(Rc::new(RefCell::new(object)))
}
