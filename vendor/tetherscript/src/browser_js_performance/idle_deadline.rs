//! Idle callback deadline objects.

use super::*;

pub(super) fn object(options: Option<&JsValue>) -> JsValue {
    let timeout = timeout_value(options);
    let did_timeout = timeout.is_some_and(|timeout| timeout <= 0.0);
    let mut map = HashMap::new();
    map.insert("didTimeout".into(), JsValue::Bool(did_timeout));
    map.insert(
        "timeout".into(),
        timeout.map(JsValue::Number).unwrap_or(JsValue::Undefined),
    );
    map.insert(
        "timeRemaining".into(),
        native("IdleDeadline.timeRemaining", Some(0), |_| {
            Ok(JsValue::Number(50.0))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(map)))
}

fn timeout_value(value: Option<&JsValue>) -> Option<f64> {
    let Some(JsValue::Object(options)) = value else {
        return None;
    };
    match options.borrow().get("timeout") {
        Some(JsValue::Number(timeout)) if timeout.is_finite() => Some(*timeout),
        Some(value) => value.display().parse().ok(),
        None => None,
    }
}
