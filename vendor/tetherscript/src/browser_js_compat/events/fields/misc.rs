use super::*;

pub(super) fn custom(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "detail".into(),
        base::prop(init, "detail").unwrap_or(JsValue::Null),
    );
}

pub(super) fn submit(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "submitter".into(),
        base::prop(init, "submitter").unwrap_or(JsValue::Null),
    );
}

pub(super) fn focus(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "relatedTarget".into(),
        base::prop(init, "relatedTarget").unwrap_or(JsValue::Null),
    );
}
