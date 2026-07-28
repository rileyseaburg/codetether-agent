use super::*;

pub(super) fn from_arg(value: Option<&JsValue>) -> JsValue {
    match value {
        Some(JsValue::Object(_)) => value.cloned().unwrap_or(JsValue::Undefined),
        _ => fresh("event"),
    }
}

pub(super) fn fresh(kind: &str) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([(
        "type".into(),
        JsValue::String(kind.into()),
    )]))))
}

pub(super) fn kind(event: &JsValue) -> String {
    let Some(object) = object(event) else {
        return "event".into();
    };
    let kind = object
        .borrow()
        .get("type")
        .map(JsValue::display)
        .filter(|kind| !kind.is_empty())
        .unwrap_or_else(|| "event".into());
    kind
}

pub(super) fn prepare(event: &JsValue, target: &Rc<RefCell<HashMap<String, JsValue>>>, kind: &str) {
    let Some(event) = object(event) else {
        return;
    };
    let target = JsValue::Object(target.clone());
    event
        .borrow_mut()
        .insert("type".into(), JsValue::String(kind.into()));
    event.borrow_mut().insert("target".into(), target.clone());
    event.borrow_mut().insert("currentTarget".into(), target);
}

fn object(value: &JsValue) -> Option<Rc<RefCell<HashMap<String, JsValue>>>> {
    match value {
        JsValue::Object(object) => Some(object.clone()),
        _ => None,
    }
}
