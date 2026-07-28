use super::*;

pub(super) fn value() -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new("URLPattern", None, create)))
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(object::from_pattern(input::pattern(args)))
}
