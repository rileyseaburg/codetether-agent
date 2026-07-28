use super::*;

pub(super) fn uint8_array(args: &[JsValue]) -> JsValue {
    let source = args.first().unwrap_or(&JsValue::Number(0.0));
    match source {
        JsValue::Number(len) if len.is_finite() && *len > 0.0 => {
            bytes::byte_array(std::iter::repeat_n(0, *len as usize))
        }
        JsValue::Array(_) | JsValue::Object(_) | JsValue::String(_) => {
            bytes::byte_array(window_bytes(source, args))
        }
        _ => bytes::byte_array([]),
    }
}

fn window_bytes(source: &JsValue, args: &[JsValue]) -> Vec<u8> {
    let bytes = bytes::bytes_from_value(source);
    let start = args
        .get(1)
        .map(|value| index(value, bytes.len()))
        .unwrap_or(0);
    let end = args
        .get(2)
        .map(|value| start.saturating_add(number::usize(Some(value), 0)))
        .unwrap_or(bytes.len())
        .min(bytes.len());
    bytes[start.min(bytes.len())..end].to_vec()
}

fn index(value: &JsValue, len: usize) -> usize {
    let index = number::signed(value);
    if index < 0.0 {
        len.saturating_sub(index.abs() as usize)
    } else {
        (index as usize).min(len)
    }
}
