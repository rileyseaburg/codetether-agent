use super::*;

pub(super) fn parse(args: &[JsValue]) -> JsValue {
    let mut text = args
        .first()
        .unwrap_or(&JsValue::Undefined)
        .display()
        .trim_start()
        .to_string();
    let sign = sign(&mut text);
    let Some(radix) = radix(args.get(1), &mut text) else {
        return JsValue::Number(f64::NAN);
    };
    digits(&text, radix, sign)
}

fn sign(text: &mut String) -> f64 {
    if text.starts_with('-') {
        text.remove(0);
        -1.0
    } else {
        if text.starts_with('+') {
            text.remove(0);
        }
        1.0
    }
}

fn digits(text: &str, radix: u32, sign: f64) -> JsValue {
    let mut value = 0.0;
    let mut seen = false;
    for ch in text.chars() {
        let Some(digit) = ch.to_digit(radix) else {
            break;
        };
        value = value * radix as f64 + digit as f64;
        seen = true;
    }
    JsValue::Number(if seen { sign * value } else { f64::NAN })
}

fn radix(value: Option<&JsValue>, text: &mut String) -> Option<u32> {
    let raw = value.map(JsValue::number).unwrap_or(0.0).trunc();
    if raw != 0.0 && !(2.0..=36.0).contains(&raw) {
        return None;
    }
    let mut radix = raw as u32;
    if radix == 0 && text.to_lowercase().starts_with("0x") {
        text.drain(..2);
        radix = 16;
    }
    Some(if radix == 0 { 10 } else { radix })
}
