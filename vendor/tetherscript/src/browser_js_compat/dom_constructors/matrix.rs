use super::*;

#[path = "matrix/object.rs"]
mod object;
#[path = "matrix/values.rs"]
mod values;

static KEYS: [&str; 12] = [
    "a", "b", "c", "d", "e", "f", "m11", "m12", "m21", "m22", "m41", "m42",
];

pub(super) fn constructor(name: &'static str) -> JsValue {
    let ctor = NativeFunction::new(name, None, create).with_property(
        "fromMatrix",
        native("DOMMatrix.fromMatrix", None, from_matrix),
    );
    JsValue::Native(Rc::new(ctor))
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(object::create(values::read(args.first()), &KEYS))
}

fn from_matrix(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(object::create(values::read(args.first()), &KEYS))
}
