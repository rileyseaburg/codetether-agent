//! Array-like receiver helpers for JavaScript built-ins.

use super::JsValue;

pub(super) fn receiver(captured: JsValue, args: &[JsValue]) -> (Vec<JsValue>, JsValue, &[JsValue]) {
    if args.len() > 1 && !callable(&args[0]) {
        let receiver = args[0].clone();
        return (values(&receiver), receiver, &args[1..]);
    }
    (values(&captured), captured, args)
}

pub(super) fn values(value: &JsValue) -> Vec<JsValue> {
    match value {
        JsValue::Array(items) => items.borrow().clone(),
        JsValue::String(text) => text
            .chars()
            .map(|ch| JsValue::String(ch.to_string()))
            .collect(),
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            let length = match obj.get("length") {
                Some(JsValue::Number(n)) => *n as usize,
                _ => 0,
            };
            (0..length)
                .map(|i| {
                    obj.get(&i.to_string())
                        .cloned()
                        .unwrap_or(JsValue::Undefined)
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn callable(value: &JsValue) -> bool {
    matches!(
        value,
        JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Class(_) | JsValue::Native(_)
    )
}
