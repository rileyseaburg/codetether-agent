use super::*;

pub(super) fn constructor(
    name: &'static str,
    bytes_per_element: usize,
    prototype: JsValue,
    make: fn(&[JsValue]) -> JsValue,
) -> JsValue {
    let proto = prototype.clone();
    JsValue::Native(Rc::new(
        NativeFunction::new(name, None, move |args| {
            let array = make(args);
            js::set_host_property(&array, "__proto__", proto.clone())?;
            js::set_host_property(&array, "__typed_array", JsValue::Bool(true))?;
            Ok(array)
        })
        .with_property(
            "BYTES_PER_ELEMENT",
            JsValue::Number(bytes_per_element as f64),
        )
        .with_property("prototype", prototype),
    ))
}
