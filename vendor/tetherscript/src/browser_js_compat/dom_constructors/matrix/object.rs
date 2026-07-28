use super::*;

pub(super) fn create(values: [f64; 6], keys: &'static [&'static str]) -> JsValue {
    let object = Rc::new(RefCell::new(number::props(&fields(values))));
    object
        .borrow_mut()
        .insert("is2D".into(), JsValue::Bool(true));
    object.borrow_mut().insert(
        "isIdentity".into(),
        JsValue::Bool(values == [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
    );
    object.borrow_mut().insert(
        "toJSON".into(),
        json::method("DOMMatrix.toJSON", &object, keys),
    );
    let text = text(values);
    object.borrow_mut().insert(
        "toString".into(),
        native("DOMMatrix.toString", Some(0), move |_| {
            Ok(JsValue::String(text.clone()))
        }),
    );
    JsValue::Object(object)
}

fn fields(v: [f64; 6]) -> [(&'static str, f64); 12] {
    [
        ("a", v[0]),
        ("b", v[1]),
        ("c", v[2]),
        ("d", v[3]),
        ("e", v[4]),
        ("f", v[5]),
        ("m11", v[0]),
        ("m12", v[1]),
        ("m21", v[2]),
        ("m22", v[3]),
        ("m41", v[4]),
        ("m42", v[5]),
    ]
}

fn text(v: [f64; 6]) -> String {
    format!(
        "matrix({}, {}, {}, {}, {}, {})",
        v[0], v[1], v[2], v[3], v[4], v[5]
    )
}
