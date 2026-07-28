use super::*;

#[path = "prototype/view.rs"]
mod view;
#[path = "prototype/write.rs"]
mod write;

pub(super) fn object(constructor: &'static str, bytes_per_element: usize) -> JsValue {
    let mut object = HashMap::new();
    object.insert(
        "BYTES_PER_ELEMENT".into(),
        JsValue::Number(bytes_per_element as f64),
    );
    object.insert("set".into(), method(constructor, "set", write::set));
    object.insert("fill".into(), method(constructor, "fill", write::fill));
    object.insert(
        "subarray".into(),
        method(constructor, "subarray", view::subarray),
    );
    object.insert(
        "copyWithin".into(),
        method(constructor, "copyWithin", view::copy_within),
    );
    JsValue::Object(Rc::new(RefCell::new(object)))
}

fn method(
    constructor: &str,
    name: &str,
    func: fn(&[JsValue]) -> Result<JsValue, String>,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(
        format!("{constructor}.prototype.{name}"),
        None,
        func,
    )))
}
