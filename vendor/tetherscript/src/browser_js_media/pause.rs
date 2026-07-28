use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    let h = handle.clone();
    obj.insert(
        "pause".into(),
        native("HTMLMediaElement.pause", Some(0), move |_| {
            let mut fired = Vec::new();
            let state = store::update(&h, |state| {
                if !state.paused {
                    state.paused = true;
                    fired.push("pause");
                }
            });
            props::refresh(&slot, &state);
            events::fire(&h, &fired)?;
            Ok(JsValue::Undefined)
        }),
    );
}
