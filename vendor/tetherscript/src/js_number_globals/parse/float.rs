use super::*;

pub(super) fn native() -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new("parseFloat", None, |args| {
        Ok(JsValue::Number(parse(
            &args.first().unwrap_or(&JsValue::Undefined).display(),
        )))
    })))
}

fn parse(input: &str) -> f64 {
    let text = input.trim_start();
    let bytes = text.as_bytes();
    let mut i = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    if text[i..].starts_with("Infinity") {
        return if text.starts_with('-') {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        };
    }
    let digits_start = i;
    while matches!(bytes.get(i), Some(b'0'..=b'9')) {
        i += 1;
    }
    if matches!(bytes.get(i), Some(b'.')) {
        i += 1;
        while matches!(bytes.get(i), Some(b'0'..=b'9')) {
            i += 1;
        }
    }
    if i == digits_start || (i == digits_start + 1 && bytes.get(digits_start) == Some(&b'.')) {
        return f64::NAN;
    }
    let end = exponent_end(bytes, i).unwrap_or(i);
    text[..end].parse().unwrap_or(f64::NAN)
}

fn exponent_end(bytes: &[u8], start: usize) -> Option<usize> {
    if !matches!(bytes.get(start), Some(b'e' | b'E')) {
        return None;
    }
    let mut i = start + 1;
    i += usize::from(matches!(bytes.get(i), Some(b'+' | b'-')));
    let digits = i;
    while matches!(bytes.get(i), Some(b'0'..=b'9')) {
        i += 1;
    }
    (i > digits).then_some(i)
}
