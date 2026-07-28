use super::*;

pub(super) fn create(args: &[JsValue]) -> JsValue {
    let message = string_arg(args.first(), "");
    let name = string_arg(args.get(1), "Error");
    let code = codes::for_name(&name);
    let object = Rc::new(RefCell::new(HashMap::from([
        ("name".into(), JsValue::String(name)),
        ("message".into(), JsValue::String(message)),
    ])));
    object
        .borrow_mut()
        .insert("code".into(), JsValue::Number(code));
    install_to_string(&object);
    JsValue::Object(object)
}

fn install_to_string(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let for_method = object.clone();
    object.borrow_mut().insert(
        "toString".into(),
        native("DOMException.toString", Some(0), move |_| {
            Ok(JsValue::String(format_exception(&for_method.borrow())))
        }),
    );
}

fn format_exception(object: &HashMap<String, JsValue>) -> String {
    let name = object.get("name").map(JsValue::display).unwrap_or_default();
    let message = object
        .get("message")
        .map(JsValue::display)
        .unwrap_or_default();
    match (name.is_empty(), message.is_empty()) {
        (true, true) => String::new(),
        (true, false) => message,
        (false, true) => name,
        (false, false) => format!("{name}: {message}"),
    }
}

fn string_arg(value: Option<&JsValue>, default: &str) -> String {
    match value {
        Some(JsValue::Undefined) | None => default.into(),
        Some(value) => value.display(),
    }
}
