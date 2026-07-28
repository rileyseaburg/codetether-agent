use super::*;

pub(super) fn from_pattern(pattern: model::Pattern) -> JsValue {
    let pattern = Rc::new(pattern);
    let mut obj = HashMap::new();
    insert_parts(&mut obj, &pattern.parts);
    insert_methods(&mut obj, pattern);
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn insert_parts(obj: &mut HashMap<String, JsValue>, parts: &model::Parts) {
    for (name, value) in [
        ("protocol", &parts.protocol),
        ("hostname", &parts.hostname),
        ("pathname", &parts.pathname),
        ("search", &parts.search),
        ("hash", &parts.hash),
    ] {
        obj.insert(name.into(), JsValue::String(value.clone()));
    }
}

fn insert_methods(obj: &mut HashMap<String, JsValue>, pattern: Rc<model::Pattern>) {
    let test_pattern = pattern.clone();
    obj.insert(
        "test".into(),
        native("URLPattern.test", None, move |args| {
            Ok(JsValue::Bool(matcher::matches(
                &test_pattern.parts,
                &input::target(args),
            )))
        }),
    );
    obj.insert(
        "exec".into(),
        native("URLPattern.exec", None, move |args| {
            let target = input::target(args);
            if matcher::matches(&pattern.parts, &target) {
                Ok(exec::result(args, &target))
            } else {
                Ok(JsValue::Null)
            }
        }),
    );
}
