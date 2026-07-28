use super::*;

pub(super) fn object(args: &[JsValue]) -> JsValue {
    let fatal = bool_option(args.get(1), "fatal");
    let ignore_bom = bool_option(args.get(1), "ignoreBOM");
    let mut obj = HashMap::from([
        ("encoding".into(), JsValue::String(label(args.first()))),
        ("fatal".into(), JsValue::Bool(fatal)),
        ("ignoreBOM".into(), JsValue::Bool(ignore_bom)),
    ]);
    obj.insert(
        "decode".into(),
        native("TextDecoder.decode", None, move |args| decode(args, fatal)),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn decode(args: &[JsValue], fatal: bool) -> Result<JsValue, String> {
    let Some(input) = args.first() else {
        return Ok(JsValue::String(String::new()));
    };
    if matches!(input, JsValue::Undefined) {
        return Ok(JsValue::String(String::new()));
    }
    let bytes = bytes::bytes_from_value(input);
    if fatal {
        return String::from_utf8(bytes)
            .map(JsValue::String)
            .map_err(|_| "TextDecoder.decode: invalid utf-8 input".into());
    }
    Ok(JsValue::String(String::from_utf8_lossy(&bytes).into()))
}

fn label(value: Option<&JsValue>) -> String {
    let Some(value) = value else {
        return "utf-8".into();
    };
    if matches!(value, JsValue::Undefined) {
        return "utf-8".into();
    }
    match value.display().trim().to_ascii_lowercase().as_str() {
        "" | "utf8" | "utf-8" => "utf-8".into(),
        other => other.into(),
    }
}

fn bool_option(options: Option<&JsValue>, name: &str) -> bool {
    match options {
        Some(JsValue::Object(obj)) => obj.borrow().get(name).is_some_and(JsValue::truthy),
        _ => false,
    }
}
