//! Deterministic media element host shims for audio and video.

use super::*;

mod attrs;
mod canplay;
mod codecs;
mod error;
mod events;
mod handlers;
mod load;
mod model;
mod object;
mod pause;
mod play;
mod props;
mod set_audio;
mod set_src;
mod set_time;
mod store;
mod thenable;
mod values;

type MediaObjectSlot = Rc<RefCell<Option<Rc<RefCell<HashMap<String, JsValue>>>>>>;

pub(super) fn install_element(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    tag: &str,
    slot: MediaObjectSlot,
) {
    object::install(obj, handle, tag, slot);
}

pub(super) fn reset_all() {
    store::reset_all();
}

#[cfg(test)]
mod tests;
