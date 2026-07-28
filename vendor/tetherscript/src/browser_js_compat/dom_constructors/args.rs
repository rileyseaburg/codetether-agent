use super::*;

pub(super) fn string_arg(args: &[JsValue], index: usize, default: &str) -> String {
    match args.get(index).unwrap_or(&JsValue::Undefined) {
        JsValue::Undefined | JsValue::Null => default.into(),
        value => value.display(),
    }
}

pub(super) fn bool_arg(args: &[JsValue], index: usize, default: bool) -> bool {
    args.get(index).map(JsValue::truthy).unwrap_or(default)
}

pub(super) fn number_arg(args: &[JsValue], index: usize) -> Option<f64> {
    match args.get(index) {
        Some(JsValue::Number(value)) if value.is_finite() => Some(value.max(0.0).trunc()),
        Some(value) => value
            .display()
            .parse::<f64>()
            .ok()
            .map(|n| n.max(0.0).trunc()),
        None => None,
    }
}

pub(super) fn object(value: &JsValue) -> Option<Rc<RefCell<HashMap<String, JsValue>>>> {
    match value {
        JsValue::Object(obj) => Some(obj.clone()),
        _ => None,
    }
}
