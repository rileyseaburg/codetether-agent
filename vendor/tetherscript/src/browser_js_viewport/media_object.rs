use super::*;

pub(super) type Listeners = Rc<RefCell<Vec<JsValue>>>;

pub(super) fn create(media: String) -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::new()));
    let listeners = Rc::new(RefCell::new(Vec::new()));
    {
        let mut map = object.borrow_mut();
        map.insert(
            "matches".into(),
            JsValue::Bool(media_query::matches(&media)),
        );
        map.insert("media".into(), JsValue::String(media));
        map.insert("onchange".into(), JsValue::Null);
        media_legacy::install(&mut map, listeners.clone());
        media_event::install(&mut map, object.clone(), listeners);
    }
    JsValue::Object(object)
}
