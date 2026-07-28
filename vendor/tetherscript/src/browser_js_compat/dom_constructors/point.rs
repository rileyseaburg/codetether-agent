use super::*;

const KEYS: [&str; 4] = ["x", "y", "z", "w"];

pub(super) fn constructor() -> JsValue {
    let ctor = NativeFunction::new("DOMPoint", None, create)
        .with_property("fromPoint", native("DOMPoint.fromPoint", None, from_point));
    JsValue::Native(Rc::new(ctor))
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(point(
        number::arg(args, 0, 0.0),
        number::arg(args, 1, 0.0),
        number::arg(args, 2, 0.0),
        number::arg(args, 3, 1.0),
    ))
}

fn from_point(args: &[JsValue]) -> Result<JsValue, String> {
    let source = args.first().unwrap_or(&JsValue::Undefined);
    Ok(point(
        number::field(source, "x", 0.0),
        number::field(source, "y", 0.0),
        number::field(source, "z", 0.0),
        number::field(source, "w", 1.0),
    ))
}

fn point(x: f64, y: f64, z: f64, w: f64) -> JsValue {
    let fields = [("x", x), ("y", y), ("z", z), ("w", w)];
    let object = Rc::new(RefCell::new(number::props(&fields)));
    let to_json = json::method("DOMPoint.toJSON", &object, &KEYS);
    object.borrow_mut().insert("toJSON".into(), to_json);
    JsValue::Object(object)
}
