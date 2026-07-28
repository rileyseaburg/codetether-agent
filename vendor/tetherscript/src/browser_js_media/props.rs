use super::error::MediaError;
use super::model::MediaState;
use super::*;

pub(super) fn write(obj: &mut HashMap<String, JsValue>, state: &MediaState) {
    obj.insert("src".into(), JsValue::String(state.src.clone()));
    obj.insert(
        "currentSrc".into(),
        JsValue::String(state.current_src.clone()),
    );
    obj.insert("duration".into(), JsValue::Number(state.duration));
    obj.insert("currentTime".into(), JsValue::Number(state.current_time));
    obj.insert("paused".into(), JsValue::Bool(state.paused));
    obj.insert("ended".into(), JsValue::Bool(state.ended));
    obj.insert(
        "readyState".into(),
        JsValue::Number(state.ready_state as f64),
    );
    obj.insert("muted".into(), JsValue::Bool(state.muted));
    obj.insert("volume".into(), JsValue::Number(state.volume));
    obj.insert("playbackRate".into(), JsValue::Number(state.playback_rate));
    obj.insert("error".into(), error_value(&state.error));
    obj.insert("HAVE_NOTHING".into(), JsValue::Number(0.0));
    obj.insert("HAVE_METADATA".into(), JsValue::Number(1.0));
    obj.insert("HAVE_ENOUGH_DATA".into(), JsValue::Number(4.0));
}

pub(super) fn refresh(slot: &MediaObjectSlot, state: &MediaState) {
    if let Some(object) = slot.borrow().as_ref() {
        write(&mut object.borrow_mut(), state);
    }
}

pub(super) fn error_object(error: &MediaError) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("code".into(), JsValue::Number(error.code as f64));
    obj.insert("message".into(), JsValue::String(error.message.clone()));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn error_value(error: &Option<MediaError>) -> JsValue {
    error.as_ref().map(error_object).unwrap_or(JsValue::Null)
}
