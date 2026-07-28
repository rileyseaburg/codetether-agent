use super::*;

pub(super) fn byte_array(bytes: impl IntoIterator<Item = u8>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        bytes
            .into_iter()
            .map(|byte| JsValue::Number(byte as f64))
            .collect(),
    )))
}

pub(super) fn bytes_from_value(value: &JsValue) -> Vec<u8> {
    match value {
        JsValue::Array(items) => items.borrow().iter().map(byte_from_value).collect(),
        JsValue::Object(obj) => object_bytes(&obj.borrow()),
        JsValue::String(text) => text.as_bytes().to_vec(),
        other => other.display().into_bytes(),
    }
}

pub(super) fn fill_array(target: &JsValue, mut next: impl FnMut() -> u8) -> Result<(), String> {
    let JsValue::Array(items) = target else {
        return Err("crypto.getRandomValues: expected byte array".into());
    };
    for slot in items.borrow_mut().iter_mut() {
        *slot = JsValue::Number(next() as f64);
    }
    Ok(())
}

fn object_bytes(object: &HashMap<String, JsValue>) -> Vec<u8> {
    let len = object.get("length").map(byte_len).unwrap_or(0);
    (0..len)
        .map(|index| {
            object
                .get(&index.to_string())
                .map(byte_from_value)
                .unwrap_or(0)
        })
        .collect()
}

fn byte_len(value: &JsValue) -> usize {
    match value {
        JsValue::Number(value) if value.is_finite() && *value > 0.0 => *value as usize,
        _ => 0,
    }
}

fn byte_from_value(value: &JsValue) -> u8 {
    match value {
        JsValue::Number(value) if value.is_finite() => (*value as i64 & 0xff) as u8,
        JsValue::Bool(true) => 1,
        _ => 0,
    }
}
