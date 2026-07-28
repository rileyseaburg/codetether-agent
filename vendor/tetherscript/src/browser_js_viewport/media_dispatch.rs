use super::*;

pub(super) fn dispatch(
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: media_object::Listeners,
    event: JsValue,
) -> Result<JsValue, String> {
    let event = event_object(&object, event);
    if event_type(&event) != "change" {
        return Ok(JsValue::Bool(true));
    }
    let this_value = JsValue::Object(object.clone());
    for listener in listeners.borrow().clone() {
        js::call_function_with_this(listener, this_value.clone(), std::slice::from_ref(&event))?;
    }
    if let Some(handler) = object.borrow().get("onchange").cloned().filter(callable) {
        js::call_function_with_this(handler, this_value, std::slice::from_ref(&event))?;
    }
    Ok(JsValue::Bool(true))
}

fn event_object(object: &Rc<RefCell<HashMap<String, JsValue>>>, event: JsValue) -> JsValue {
    let mut map = match event {
        JsValue::Object(event) => event.borrow().clone(),
        _ => HashMap::new(),
    };
    map.entry("type".into())
        .or_insert_with(|| JsValue::String("change".into()));
    map.insert("target".into(), JsValue::Object(object.clone()));
    map.insert("currentTarget".into(), JsValue::Object(object.clone()));
    for name in ["matches", "media"] {
        if let Some(value) = object.borrow().get(name).cloned() {
            map.insert(name.into(), value);
        }
    }
    JsValue::Object(Rc::new(RefCell::new(map)))
}

fn event_type(event: &JsValue) -> String {
    match event {
        JsValue::Object(object) => object
            .borrow()
            .get("type")
            .map(JsValue::display)
            .unwrap_or_else(|| "change".into()),
        _ => "change".into(),
    }
}

fn callable(value: &JsValue) -> bool {
    !matches!(value, JsValue::Undefined | JsValue::Null)
}
