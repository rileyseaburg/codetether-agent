use super::*;

const IDENTITY: [f64; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

pub(super) fn read(source: Option<&JsValue>) -> [f64; 6] {
    match source {
        Some(JsValue::Array(items)) if items.borrow().len() >= 6 => {
            std::array::from_fn(|index| value(&items.borrow()[index]))
        }
        Some(JsValue::String(text)) => string(text),
        Some(value) => [
            number::field(value, "a", 1.0),
            number::field(value, "b", 0.0),
            number::field(value, "c", 0.0),
            number::field(value, "d", 1.0),
            number::field(value, "e", 0.0),
            number::field(value, "f", 0.0),
        ],
        None => IDENTITY,
    }
}

fn string(text: &str) -> [f64; 6] {
    let Some(body) = text
        .strip_prefix("matrix(")
        .and_then(|value| value.strip_suffix(')'))
    else {
        return IDENTITY;
    };
    let parts = body
        .split(',')
        .map(|part| part.trim().parse().unwrap_or(0.0))
        .collect::<Vec<_>>();
    if parts.len() >= 6 {
        std::array::from_fn(|index| parts[index])
    } else {
        IDENTITY
    }
}

fn value(value: &JsValue) -> f64 {
    match value {
        JsValue::Null => 0.0,
        JsValue::Bool(true) => 1.0,
        JsValue::Bool(false) => 0.0,
        JsValue::Number(value) => *value,
        JsValue::String(value) => value.trim().parse().unwrap_or(f64::NAN),
        _ => f64::NAN,
    }
}
