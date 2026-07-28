//! Deterministic fullscreen and pointer-lock host shims.

use super::*;

#[path = "browser_js_fullscreen/document.rs"]
mod document;
#[path = "browser_js_fullscreen/documents.rs"]
mod documents;
#[path = "browser_js_fullscreen/element.rs"]
mod element;
#[path = "browser_js_fullscreen/events.rs"]
mod events;
#[path = "browser_js_fullscreen/props.rs"]
mod props;
#[path = "browser_js_fullscreen/state.rs"]
mod state;
#[path = "browser_js_fullscreen/target.rs"]
mod target;
#[path = "browser_js_fullscreen/thenable.rs"]
mod thenable;

type DomObject = Rc<RefCell<HashMap<String, JsValue>>>;
type DomObjectSlot = Rc<RefCell<Option<DomObject>>>;

pub(super) fn reset() {
    state::reset();
    documents::reset();
}

pub(super) fn install_document(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    document::install(obj, handle);
}

pub(super) fn install_element(
    obj: &mut HashMap<String, JsValue>,
    handle: &DomHandle,
    slot: DomObjectSlot,
) {
    element::install(obj, handle, slot);
}

pub(super) fn register_document(value: &JsValue) {
    if let JsValue::Object(object) = value {
        documents::register(object);
    }
}

#[cfg(test)]
#[path = "browser_js_fullscreen/tests_fullscreen.rs"]
mod tests_fullscreen;
#[cfg(test)]
#[path = "browser_js_fullscreen/tests_pointer.rs"]
mod tests_pointer;
