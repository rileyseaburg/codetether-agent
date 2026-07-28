use super::error::MediaError;
use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: MediaObjectSlot,
) {
    let h = handle.clone();
    obj.insert(
        "play".into(),
        native("HTMLMediaElement.play", Some(0), move |_| {
            let mut fired = Vec::new();
            let mut error = None;
            let state = store::update(&h, |state| {
                if state.ready_state == 0 {
                    load::load_state(&h, state);
                    fired.extend_from_slice(events::LOAD_EVENTS);
                }
                if state.current_src.is_empty() {
                    let media_error = MediaError {
                        code: 4,
                        message: "media source unavailable".into(),
                    };
                    state.error = Some(media_error.clone());
                    error = Some(media_error);
                    fired.push("error");
                } else {
                    state.paused = false;
                    state.ended = false;
                    fired.extend(["play", "playing"]);
                }
            });
            props::refresh(&slot, &state);
            events::fire(&h, &fired)?;
            Ok(error
                .as_ref()
                .map(|e| thenable::rejected(props::error_object(e)))
                .unwrap_or_else(|| thenable::resolved(JsValue::Undefined)))
        }),
    );
}
