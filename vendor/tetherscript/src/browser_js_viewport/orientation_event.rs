use super::super::*;

pub(super) fn create(object: &Rc<RefCell<HashMap<String, JsValue>>>, event: JsValue) -> JsValue {
    let mut map = match event {
        JsValue::Object(event) => event.borrow().clone(),
        _ => HashMap::new(),
    };
    map.entry("type".into())
        .or_insert_with(|| JsValue::String("change".into()));
    map.insert("target".into(), JsValue::Object(object.clone()));
    map.insert("currentTarget".into(), JsValue::Object(object.clone()));
    JsValue::Object(Rc::new(RefCell::new(map)))
}

pub(super) fn event_type(event: &JsValue) -> String {
    match event {
        JsValue::Object(event) => event
            .borrow()
            .get("type")
            .map(JsValue::display)
            .unwrap_or_else(|| "change".into()),
        _ => "change".into(),
    }
}
