use super::*;

pub(super) fn object() -> JsValue {
    let mut obj = HashMap::from([("encoding".into(), JsValue::String("utf-8".into()))]);
    obj.insert("encode".into(), native("TextEncoder.encode", None, encode));
    obj.insert(
        "encodeInto".into(),
        native("TextEncoder.encodeInto", None, encode_into),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn encode(args: &[JsValue]) -> Result<JsValue, String> {
    let text = source_text(args);
    Ok(bytes::byte_array(text.into_bytes()))
}

fn encode_into(args: &[JsValue]) -> Result<JsValue, String> {
    let source = source_text(args);
    let destination = args.get(1).unwrap_or(&JsValue::Undefined);
    let (read, written) = destination::write_utf8(destination, &source);
    Ok(result_object(read, written))
}

fn source_text(args: &[JsValue]) -> String {
    match args.first() {
        None | Some(JsValue::Undefined) => String::new(),
        Some(value) => value.display(),
    }
}

fn result_object(read: usize, written: usize) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([
        ("read".into(), JsValue::Number(read as f64)),
        ("written".into(), JsValue::Number(written as f64)),
    ]))))
}
