use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    install_muted(obj, handle, slot.clone());
    install_volume(obj, handle, slot.clone());
    install_rate(obj, handle, slot);
}

fn install_muted(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, slot: MediaObjectSlot) {
    let h = handle.clone();
    obj.insert(
        "__set:muted".into(),
        native("set_media_muted", Some(1), move |args| {
            let value = values::bool_value(args.first());
            let state = store::update(&h, |state| state.muted = value);
            props::refresh(&slot, &state);
            events::fire(&h, &["volumechange"])?;
            Ok(JsValue::Bool(value))
        }),
    );
}

fn install_volume(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, slot: MediaObjectSlot) {
    let h = handle.clone();
    obj.insert(
        "__set:volume".into(),
        native("set_media_volume", Some(1), move |args| {
            let value = values::number(args.first(), 1.0).clamp(0.0, 1.0);
            let state = store::update(&h, |state| state.volume = value);
            props::refresh(&slot, &state);
            events::fire(&h, &["volumechange"])?;
            Ok(JsValue::Number(value))
        }),
    );
}

fn install_rate(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, slot: MediaObjectSlot) {
    let h = handle.clone();
    obj.insert(
        "__set:playbackRate".into(),
        native("set_media_playbackRate", Some(1), move |args| {
            let value = values::number(args.first(), 1.0).max(0.0);
            let state = store::update(&h, |state| state.playback_rate = value);
            props::refresh(&slot, &state);
            events::fire(&h, &["ratechange"])?;
            Ok(JsValue::Number(value))
        }),
    );
}
