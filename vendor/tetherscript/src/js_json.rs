use super::*;

#[path = "js_json/convert.rs"]
mod convert;

pub(super) fn install(env: &EnvRef) {
    let mut object = HashMap::new();
    object.insert(
        "parse".into(),
        JsValue::Native(Rc::new(NativeFunction::new("JSON.parse", Some(1), parse))),
    );
    object.insert(
        "stringify".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "JSON.stringify",
            Some(1),
            stringify,
        ))),
    );
    env.borrow_mut()
        .define("JSON", JsValue::Object(Rc::new(RefCell::new(object))));
}

fn parse(args: &[JsValue]) -> Result<JsValue, String> {
    let text = args.first().unwrap_or(&JsValue::Undefined).display();
    crate::json::parse_str(&text)
        .map(|value| convert::value_to_js(&value))
        .map_err(|error| format!("JSON.parse: {error}"))
}

fn stringify(args: &[JsValue]) -> Result<JsValue, String> {
    let value = args.first().unwrap_or(&JsValue::Undefined);
    crate::json::encode_to_string(&convert::js_to_value(value))
        .map(JsValue::String)
        .map_err(|error| format!("JSON.stringify: {error}"))
}
