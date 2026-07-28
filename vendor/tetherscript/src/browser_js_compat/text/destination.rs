use super::*;

pub(super) fn write_utf8(target: &JsValue, source: &str) -> (usize, usize) {
    match target {
        JsValue::Array(items) => write_array(&mut items.borrow_mut(), source),
        JsValue::Object(obj) => write_object(&mut obj.borrow_mut(), source),
        _ => (0, 0),
    }
}

fn write_array(items: &mut [JsValue], source: &str) -> (usize, usize) {
    write_slots(items.len(), source, |index, byte| {
        items[index] = JsValue::Number(byte as f64);
    })
}

fn write_object(obj: &mut HashMap<String, JsValue>, source: &str) -> (usize, usize) {
    let limit = object_len(obj);
    write_slots(limit, source, |index, byte| {
        obj.insert(index.to_string(), JsValue::Number(byte as f64));
    })
}

fn write_slots(limit: usize, source: &str, mut write: impl FnMut(usize, u8)) -> (usize, usize) {
    let mut read = 0;
    let mut written = 0;
    for ch in source.chars() {
        let mut buf = [0; 4];
        let encoded = ch.encode_utf8(&mut buf).as_bytes();
        if written + encoded.len() > limit {
            break;
        }
        for byte in encoded {
            write(written, *byte);
            written += 1;
        }
        read += ch.len_utf16();
    }
    (read, written)
}

fn object_len(obj: &HashMap<String, JsValue>) -> usize {
    match obj.get("length") {
        Some(JsValue::Number(n)) if n.is_finite() && *n > 0.0 => n.trunc() as usize,
        _ => 0,
    }
}
