use super::*;

pub(super) fn create() -> JsValue {
    let event = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map = event.borrow_mut();
        map.insert("type".into(), JsValue::String(String::new()));
        map.insert("bubbles".into(), JsValue::Bool(false));
        map.insert("cancelable".into(), JsValue::Bool(false));
        map.insert("defaultPrevented".into(), JsValue::Bool(false));
        map.insert("target".into(), JsValue::Null);
        map.insert("currentTarget".into(), JsValue::Null);
        map.insert("eventPhase".into(), JsValue::Number(0.0));
        map.insert("timeStamp".into(), JsValue::Number(0.0));
    }
    methods::install(&event);
    JsValue::Object(event)
}
