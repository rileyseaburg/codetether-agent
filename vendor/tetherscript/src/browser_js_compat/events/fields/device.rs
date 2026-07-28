use super::*;

pub(super) fn orientation(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["alpha", "beta", "gamma"] {
        map.insert(name.into(), nullable_number(init, name));
    }
    map.insert(
        "absolute".into(),
        JsValue::Bool(base::bool_prop(init, "absolute")),
    );
}

pub(super) fn motion(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in [
        "acceleration",
        "accelerationIncludingGravity",
        "rotationRate",
    ] {
        map.insert(name.into(), base::prop(init, name).unwrap_or(JsValue::Null));
    }
    map.insert(
        "interval".into(),
        JsValue::Number(base::num_prop(init, "interval", 0.0)),
    );
}

fn nullable_number(init: Option<&JsValue>, name: &str) -> JsValue {
    match base::prop(init, name) {
        None | Some(JsValue::Null) | Some(JsValue::Undefined) => JsValue::Null,
        Some(JsValue::Number(number)) => JsValue::Number(number),
        Some(JsValue::Bool(value)) => JsValue::Number(if value { 1.0 } else { 0.0 }),
        Some(JsValue::String(value)) => JsValue::Number(value.parse().unwrap_or(f64::NAN)),
        Some(_) => JsValue::Number(f64::NAN),
    }
}
