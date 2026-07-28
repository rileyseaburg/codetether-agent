use super::*;

pub(super) const LOAD_EVENTS: &[&str] = &[
    "loadstart",
    "durationchange",
    "loadedmetadata",
    "loadeddata",
    "canplay",
];

pub(super) fn fire(handle: &DomHandle, events: &[&str]) -> Result<(), String> {
    for event in events {
        handle.dispatch_event(JsValue::String((*event).into()))?;
    }
    Ok(())
}
