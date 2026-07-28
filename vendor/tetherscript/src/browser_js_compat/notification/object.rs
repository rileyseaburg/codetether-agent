use super::*;

pub(super) fn construct(args: &[JsValue]) -> Result<JsValue, String> {
    let options = args::options(args);
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut obj = object.borrow_mut();
        obj.insert("title".into(), JsValue::String(args::text_arg(args, 0, "")));
        obj.insert(
            "body".into(),
            JsValue::String(args::option_text(&options, "body", "")),
        );
        obj.insert(
            "tag".into(),
            JsValue::String(args::option_text(&options, "tag", "")),
        );
        obj.insert(
            "data".into(),
            args::option_value(&options, "data", JsValue::Null),
        );
        insert_flag(&mut obj, &options, "silent");
        insert_flag(&mut obj, &options, "renotify");
        insert_flag(&mut obj, &options, "requireInteraction");
        obj.insert("closed".into(), JsValue::Bool(false));
        for handler in ["onclick", "onclose", "onerror", "onshow"] {
            obj.insert(handler.into(), JsValue::Null);
        }
    }
    events::install(&object);
    Ok(JsValue::Object(object))
}

fn insert_flag(object: &mut HashMap<String, JsValue>, options: &JsValue, name: &str) {
    object.insert(name.into(), JsValue::Bool(args::option_bool(options, name)));
}
