use super::*;

#[path = "fields/animation.rs"]
mod animation;
#[path = "fields/clipboard.rs"]
mod clipboard;
#[path = "fields/close_event.rs"]
mod close_event;
#[path = "fields/constructors.rs"]
pub(super) mod constructors;
#[path = "fields/device.rs"]
mod device;
#[path = "fields/error_event.rs"]
mod error_event;
#[path = "fields/interactions.rs"]
mod interactions;
#[path = "fields/keyboard.rs"]
mod keyboard;
#[path = "fields/lifecycle.rs"]
mod lifecycle;
#[path = "fields/media_query.rs"]
mod media_query;
#[path = "fields/message_event.rs"]
mod message_event;
#[path = "fields/misc.rs"]
mod misc;
#[path = "fields/mouse.rs"]
mod mouse;
#[path = "fields/pointer.rs"]
mod pointer;
#[path = "fields/promise_rejection.rs"]
mod promise_rejection;
#[path = "fields/storage.rs"]
mod storage;
#[path = "fields/text.rs"]
mod text;
#[path = "fields/wheel.rs"]
mod wheel;

pub(super) fn insert(
    map: &mut HashMap<String, JsValue>,
    event_class: class::EventClass,
    init: Option<&JsValue>,
) {
    event_class.insert_fields(map, init);
}
