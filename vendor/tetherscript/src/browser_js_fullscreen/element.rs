use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, slot: DomObjectSlot) {
    let h = handle.clone();
    let fullscreen_slot = slot.clone();
    obj.insert(
        "requestFullscreen".into(),
        native("Element.requestFullscreen", Some(0), move |_| {
            if h.node().is_some() {
                let target = target::Target::new(&h, &fullscreen_slot);
                if state::request_fullscreen(target) {
                    documents::refresh();
                    events::change(&h, "fullscreenchange")?;
                }
            }
            Ok(thenable::resolved(JsValue::Undefined))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "requestPointerLock".into(),
        native("Element.requestPointerLock", Some(0), move |_| {
            if h.node().is_none() {
                events::error(&h, "pointerlockerror")?;
                return Ok(JsValue::Undefined);
            }
            let target = target::Target::new(&h, &slot);
            if state::request_pointer_lock(target) {
                documents::refresh();
                events::change(&h, "pointerlockchange")?;
            }
            Ok(JsValue::Undefined)
        }),
    );
}
