use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    let h = handle.clone();
    obj.insert(
        "load".into(),
        native("HTMLMediaElement.load", Some(0), move |_| {
            let state = store::update(&h, |state| load_state(&h, state));
            props::refresh(&slot, &state);
            events::fire(&h, events::LOAD_EVENTS)?;
            Ok(JsValue::Undefined)
        }),
    );
}

pub(super) fn load_state(handle: &DomHandle, state: &mut model::MediaState) {
    state.src = attrs::src(handle);
    state.current_src = state.src.clone();
    if let Some(duration) = attrs::duration(handle) {
        state.duration = duration;
    }
    state.current_time = state.current_time.min(state.duration.max(0.0));
    state.ready_state = if state.current_src.is_empty() { 0 } else { 4 };
    state.ended = false;
    state.error = None;
}
