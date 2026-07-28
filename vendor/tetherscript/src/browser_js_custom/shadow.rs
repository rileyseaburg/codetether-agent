use super::*;

#[path = "shadow/object.rs"]
mod object;
#[path = "shadow/options.rs"]
mod options;
#[path = "shadow/store.rs"]
mod store;

#[derive(Clone)]
pub(super) struct ShadowEntry {
    pub(super) root: Rc<RefCell<Node>>,
    pub(super) host: DomHandle,
    pub(super) mode: String,
    pub(super) delegates_focus: bool,
}

pub(super) fn reset() {
    store::reset();
}

pub(super) fn attach(host: &DomHandle, options: Option<&JsValue>) -> JsValue {
    let mode = options::mode(options);
    if mode != "open" && mode != "closed" {
        return JsValue::Null;
    }
    let entry = store::entry(host, mode, options::delegates_focus(options));
    object::from_entry(entry)
}

pub(super) fn open_object(host: &DomHandle) -> JsValue {
    store::get(host)
        .filter(|entry| entry.mode == "open")
        .map(object::from_entry)
        .unwrap_or(JsValue::Null)
}
