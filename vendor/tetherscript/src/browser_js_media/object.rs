use super::*;

pub(super) fn install(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    tag: &str,
    slot: MediaObjectSlot,
) {
    let state = store::get(handle);
    props::write(obj, &state);
    handlers::install(obj, handle);
    load::install(obj, handle, slot.clone());
    play::install(obj, handle, slot.clone());
    pause::install(obj, handle, slot.clone());
    canplay::install(obj, tag);
    set_src::install(obj, handle, slot.clone());
    set_time::install(obj, handle, slot.clone());
    set_audio::install(obj, handle, slot);
}
