use super::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn install(
    navigator: &mut HashMap<String, JsValue>,
    host: Rc<RefCell<HashMap<String, JsValue>>>,
) {
    navigator.insert(
        "vibrate".into(),
        native("navigator.vibrate", None, move |args| {
            let pattern = args.first().cloned().unwrap_or(JsValue::Undefined);
            if !valid(&pattern) {
                return Ok(JsValue::Bool(false));
            }
            host.borrow_mut()
                .insert("__lastVibration".into(), copy(&pattern));
            Ok(JsValue::Bool(true))
        }),
    );
}

fn valid(pattern: &JsValue) -> bool {
    matches!(pattern, JsValue::Number(_) | JsValue::Array(_))
}

fn copy(pattern: &JsValue) -> JsValue {
    match pattern {
        JsValue::Array(items) => JsValue::Array(Rc::new(RefCell::new(items.borrow().clone()))),
        other => other.clone(),
    }
}
