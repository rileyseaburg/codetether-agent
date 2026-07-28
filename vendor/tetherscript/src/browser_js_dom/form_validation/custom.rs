use super::*;

thread_local! {
    static MESSAGES: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub(super) fn get(handle: &DomHandle) -> Option<String> {
    MESSAGES.with(|messages| messages.borrow().get(&handle.event_key()).cloned())
}

pub(super) fn set(handle: &DomHandle, message: String) {
    MESSAGES.with(|messages| {
        if message.is_empty() {
            messages.borrow_mut().remove(&handle.event_key());
        } else {
            messages.borrow_mut().insert(handle.event_key(), message);
        }
    });
}

pub(super) fn reset() {
    MESSAGES.with(|messages| messages.borrow_mut().clear());
}
