use super::*;

pub(super) fn create() -> JsValue {
    let strings = Rc::new(RefCell::new(Vec::new()));
    let types = model::array();
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map = object.borrow_mut();
        insert_defaults(&mut map, types.clone());
        data::install(&mut map, strings, types);
    }
    JsValue::Object(object)
}

fn insert_defaults(map: &mut HashMap<String, JsValue>, types: model::SharedArray) {
    map.insert("dropEffect".into(), JsValue::String("none".into()));
    map.insert("effectAllowed".into(), JsValue::String("all".into()));
    map.insert("types".into(), JsValue::Array(types));
    map.insert("files".into(), JsValue::Array(model::array()));
    map.insert("items".into(), items::create());
}
