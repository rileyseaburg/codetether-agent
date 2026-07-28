use super::*;

pub(super) fn range(args: &[JsValue], len: usize) -> (usize, usize) {
    let start = args.first().map(|value| index(value, len, 0)).unwrap_or(0);
    let end = args
        .get(1)
        .map(|value| index(value, len, len))
        .unwrap_or(len);
    (start, end.max(start))
}

fn index(value: &JsValue, len: usize, fallback: usize) -> usize {
    let number = number(value).unwrap_or(fallback as f64).trunc() as i64;
    let len = len as i64;
    if number < 0 {
        (len + number).clamp(0, len) as usize
    } else {
        number.min(len) as usize
    }
}

fn number(value: &JsValue) -> Option<f64> {
    match value {
        JsValue::Number(value) if value.is_finite() => Some(*value),
        JsValue::String(value) => value.parse().ok(),
        _ => None,
    }
}
