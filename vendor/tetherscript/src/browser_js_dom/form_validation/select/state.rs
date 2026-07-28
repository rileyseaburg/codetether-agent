use super::super::*;

thread_local! {
    static NONE: RefCell<HashMap<String, bool>> = RefCell::new(HashMap::new());
}

pub(super) fn is_none(handle: &DomHandle) -> bool {
    NONE.with(|none| none.borrow().contains_key(&handle.event_key()))
}

pub(super) fn set_none(handle: &DomHandle) {
    NONE.with(|none| {
        none.borrow_mut().insert(handle.event_key(), true);
    });
}

pub(super) fn clear(handle: &DomHandle) {
    NONE.with(|none| {
        none.borrow_mut().remove(&handle.event_key());
    });
}

pub(super) fn reset() {
    NONE.with(|none| none.borrow_mut().clear());
}
