use super::*;

pub(super) fn pop_state(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "state".into(),
        base::prop(init, "state").unwrap_or(JsValue::Null),
    );
}

pub(super) fn hash_change(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    for name in ["oldURL", "newURL"] {
        map.insert(name.into(), JsValue::String(base::string_prop(init, name)));
    }
}

pub(super) fn page_transition(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "persisted".into(),
        JsValue::Bool(base::bool_prop(init, "persisted")),
    );
}

pub(super) fn before_unload(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "returnValue".into(),
        JsValue::String(base::string_prop(init, "returnValue")),
    );
}

pub(super) fn progress(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "lengthComputable".into(),
        JsValue::Bool(base::bool_prop(init, "lengthComputable")),
    );
    for name in ["loaded", "total"] {
        map.insert(
            name.into(),
            JsValue::Number(base::num_prop(init, name, 0.0)),
        );
    }
}
