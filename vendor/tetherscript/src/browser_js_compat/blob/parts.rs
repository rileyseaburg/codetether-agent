use super::*;

pub(super) fn collect(value: Option<&JsValue>) -> Vec<u8> {
    let Some(value) = value else {
        return Vec::new();
    };
    match value {
        JsValue::Array(parts) => {
            let mut out = Vec::new();
            for part in parts.borrow().iter() {
                append_part(&mut out, part);
            }
            out
        }
        other => part_bytes(other),
    }
}

pub(super) fn option_type(value: Option<&JsValue>) -> String {
    property(value, "type")
        .map(|value| normalize_type(&value.display()))
        .unwrap_or_default()
}

pub(super) fn option_last_modified(value: Option<&JsValue>) -> f64 {
    match property(value, "lastModified") {
        Some(JsValue::Number(value)) if value.is_finite() => value,
        Some(value) => value.display().parse().unwrap_or(0.0),
        None => 0.0,
    }
}

pub(super) fn normalize_type(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_control())
        .collect::<String>()
        .to_ascii_lowercase()
}

fn append_part(out: &mut Vec<u8>, value: &JsValue) {
    out.extend(part_bytes(value));
}

fn part_bytes(value: &JsValue) -> Vec<u8> {
    access::bytes(value).unwrap_or_else(|| bytes::bytes_from_value(value))
}

fn property(value: Option<&JsValue>, name: &str) -> Option<JsValue> {
    let Some(JsValue::Object(object)) = value else {
        return None;
    };
    object.borrow().get(name).cloned()
}
