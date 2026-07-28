use super::*;

pub(super) fn text_arg(args: &[JsValue], index: usize, default: &str) -> String {
    text(args.get(index), default)
}

pub(super) fn options(args: &[JsValue]) -> JsValue {
    args.get(1).cloned().unwrap_or(JsValue::Undefined)
}

pub(super) fn option_text(options: &JsValue, name: &str, default: &str) -> String {
    text(property(options, name).as_ref(), default)
}

pub(super) fn option_bool(options: &JsValue, name: &str) -> bool {
    property(options, name).is_some_and(|value| value.truthy())
}

pub(super) fn option_value(options: &JsValue, name: &str, default: JsValue) -> JsValue {
    match property(options, name) {
        Some(JsValue::Undefined) | None => default,
        Some(value) => value,
    }
}

fn property(options: &JsValue, name: &str) -> Option<JsValue> {
    let JsValue::Object(object) = options else {
        return None;
    };
    object.borrow().get(name).cloned()
}

fn text(value: Option<&JsValue>, default: &str) -> String {
    match value {
        Some(JsValue::Undefined) | None => default.into(),
        Some(value) => value.display(),
    }
}
