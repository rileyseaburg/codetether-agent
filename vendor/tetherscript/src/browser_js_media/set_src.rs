use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    let h = handle.clone();
    obj.insert(
        "__set:src".into(),
        native("set_media_src", Some(1), move |args| {
            let src = args.first().unwrap_or(&JsValue::Undefined).display();
            attrs::set(&h, "src", src.clone());
            let state = store::update(&h, |state| {
                state.src = src.clone();
                state.error = None;
            });
            props::refresh(&slot, &state);
            Ok(JsValue::String(src))
        }),
    );
}
