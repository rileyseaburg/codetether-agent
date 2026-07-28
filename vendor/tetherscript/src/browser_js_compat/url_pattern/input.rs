use super::*;

pub(super) fn pattern(args: &[JsValue]) -> model::Pattern {
    let first = args.first().unwrap_or(&JsValue::Undefined);
    let base = args.get(1).map(JsValue::display);
    model::Pattern {
        parts: parts(first, base.as_deref(), true),
    }
}

pub(super) fn target(args: &[JsValue]) -> model::Parts {
    let first = args.first().unwrap_or(&JsValue::Undefined);
    let base = args.get(1).map(JsValue::display);
    parts(first, base.as_deref(), false)
}

fn parts(value: &JsValue, base: Option<&str>, pattern: bool) -> model::Parts {
    match value {
        JsValue::Object(obj) => object_input::parts(obj, pattern),
        other => string_input::parts(&other.display(), base, pattern),
    }
}
