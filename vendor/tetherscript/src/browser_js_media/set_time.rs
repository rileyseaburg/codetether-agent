use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    let h = handle.clone();
    obj.insert(
        "__set:currentTime".into(),
        native("set_media_currentTime", Some(1), move |args| {
            let wanted = values::number(args.first(), 0.0).max(0.0);
            let mut fired = vec!["seeking", "timeupdate", "seeked"];
            let state = store::update(&h, |state| {
                let cap = if state.duration > 0.0 {
                    state.duration
                } else {
                    wanted
                };
                state.current_time = wanted.min(cap);
                state.ended = state.duration > 0.0 && state.current_time >= state.duration;
                if state.ended {
                    state.paused = true;
                    fired.push("ended");
                }
            });
            props::refresh(&slot, &state);
            events::fire(&h, &fired)?;
            Ok(JsValue::Number(state.current_time))
        }),
    );
}
