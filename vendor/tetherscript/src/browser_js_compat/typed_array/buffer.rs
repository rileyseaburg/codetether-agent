use super::*;

pub(super) fn constructor(name: &'static str) -> JsValue {
    let prototype = JsValue::Object(Rc::new(RefCell::new(HashMap::new())));
    let mut ctor = NativeFunction::new(name, None, move |args| Ok(buffer(name, args)));
    if name == "ArrayBuffer" {
        ctor = ctor.with_property(
            "isView",
            JsValue::Native(Rc::new(NativeFunction::new(
                "ArrayBuffer.isView",
                Some(1),
                |args| Ok(JsValue::Bool(is_view(&args[0]))),
            ))),
        );
    }
    JsValue::Native(Rc::new(ctor.with_property("prototype", prototype)))
}

fn buffer(name: &str, args: &[JsValue]) -> JsValue {
    let len = number::usize(args.first(), 0);
    let mut object = HashMap::new();
    object.insert("__array_buffer".into(), JsValue::Bool(true));
    object.insert("Symbol.toStringTag".into(), JsValue::String(name.into()));
    object.insert("constructor".into(), JsValue::String(name.into()));
    object.insert("byteLength".into(), JsValue::Number(len as f64));
    object.insert("length".into(), JsValue::Number(len as f64));
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn is_view(value: &JsValue) -> bool {
    matches!(
        js::get_host_property(value, "__typed_array"),
        Ok(JsValue::Bool(true))
    )
}
