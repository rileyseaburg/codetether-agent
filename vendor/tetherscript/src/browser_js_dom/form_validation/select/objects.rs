use super::super::*;

type Object = Rc<RefCell<HashMap<String, JsValue>>>;

thread_local! {
    static SELECTS: RefCell<HashMap<String, Object>> = RefCell::new(HashMap::new());
}

pub(super) fn register(handle: &DomHandle, obj: &Object) {
    SELECTS.with(|selects| {
        selects.borrow_mut().insert(handle.event_key(), obj.clone());
    });
}

pub(super) fn refresh(handle: &DomHandle) {
    SELECTS.with(|selects| {
        if let Some(obj) = selects.borrow().get(&handle.event_key()) {
            super::props::write(obj, handle);
        }
    });
}

pub(super) fn reset() {
    SELECTS.with(|selects| selects.borrow_mut().clear());
}
