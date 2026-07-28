use super::*;

pub(super) fn animation(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    insert_string(map, init, "animationName");
    insert_elapsed(map, init);
    insert_string(map, init, "pseudoElement");
}

pub(super) fn transition(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    insert_string(map, init, "propertyName");
    insert_elapsed(map, init);
    insert_string(map, init, "pseudoElement");
}

fn insert_elapsed(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>) {
    map.insert(
        "elapsedTime".into(),
        JsValue::Number(base::num_prop(init, "elapsedTime", 0.0)),
    );
}

fn insert_string(map: &mut HashMap<String, JsValue>, init: Option<&JsValue>, name: &str) {
    map.insert(name.into(), JsValue::String(base::string_prop(init, name)));
}
