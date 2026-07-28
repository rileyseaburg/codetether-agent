use super::*;

const KEYS: [&str; 8] = [
    "x", "y", "width", "height", "left", "top", "right", "bottom",
];

pub(super) fn constructor() -> JsValue {
    let ctor = NativeFunction::new("DOMRect", None, create)
        .with_property("fromRect", native("DOMRect.fromRect", None, from_rect));
    JsValue::Native(Rc::new(ctor))
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(rect(
        number::arg(args, 0, 0.0),
        number::arg(args, 1, 0.0),
        number::arg(args, 2, 0.0),
        number::arg(args, 3, 0.0),
    ))
}

fn from_rect(args: &[JsValue]) -> Result<JsValue, String> {
    let source = args.first().unwrap_or(&JsValue::Undefined);
    Ok(rect(
        number::field(source, "x", 0.0),
        number::field(source, "y", 0.0),
        number::field(source, "width", 0.0),
        number::field(source, "height", 0.0),
    ))
}

fn rect(x: f64, y: f64, width: f64, height: f64) -> JsValue {
    let fields = fields(x, y, width, height);
    let object = Rc::new(RefCell::new(number::props(&fields)));
    let to_json = json::method("DOMRect.toJSON", &object, &KEYS);
    object.borrow_mut().insert("toJSON".into(), to_json);
    JsValue::Object(object)
}

fn fields(x: f64, y: f64, width: f64, height: f64) -> [(&'static str, f64); 8] {
    let right = x + width;
    let bottom = y + height;
    [
        ("x", x),
        ("y", y),
        ("width", width),
        ("height", height),
        ("left", x.min(right)),
        ("top", y.min(bottom)),
        ("right", x.max(right)),
        ("bottom", y.max(bottom)),
    ]
}
