use super::*;

pub(super) fn name(value: Option<&JsValue>) -> Option<String> {
    match value? {
        JsValue::Undefined | JsValue::Null => None,
        JsValue::String(name) => trimmed(name),
        JsValue::Object(object) => object
            .borrow()
            .get("name")
            .and_then(|value| trimmed(&value.display())),
        other => trimmed(&other.display()),
    }
}

pub(super) fn set_args(args: &[JsValue]) -> Option<(String, String)> {
    if let Some(JsValue::Object(object)) = args.first() {
        let object = object.borrow();
        let name = object
            .get("name")
            .and_then(|value| trimmed(&value.display()))?;
        let value = object
            .get("value")
            .map(JsValue::display)
            .unwrap_or_default();
        return Some((name, value));
    }
    Some((
        name(args.first())?,
        args.get(1).map(JsValue::display).unwrap_or_default(),
    ))
}

fn trimmed(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.into())
}
