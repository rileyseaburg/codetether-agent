use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "matchMedia".into(),
        native("matchMedia", Some(1), move |args| {
            let query = args.first().map(JsValue::display).unwrap_or_default();
            Ok(media_object::create(query))
        }),
    );
}
