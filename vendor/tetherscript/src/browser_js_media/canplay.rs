use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, tag: &str) {
    let tag = tag.to_string();
    obj.insert(
        "canPlayType".into(),
        native("HTMLMediaElement.canPlayType", Some(1), move |args| {
            let mime = args.first().unwrap_or(&JsValue::Undefined).display();
            Ok(JsValue::String(codecs::can_play_type(&tag, &mime).into()))
        }),
    );
}
