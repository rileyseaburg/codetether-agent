use super::*;

#[path = "orientation_lock_thenable.rs"]
mod thenable;

const LOCK_REASON: &str = "screen.orientation.lock: unsupported";

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    insert_lock(object);
    insert_unlock(object);
}

fn insert_lock(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    object.borrow_mut().insert(
        "lock".into(),
        native("screen.orientation.lock", Some(1), |_| {
            Ok(thenable::rejected(LOCK_REASON))
        }),
    );
}

fn insert_unlock(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    object.borrow_mut().insert(
        "unlock".into(),
        native("screen.orientation.unlock", Some(0), |_| {
            Ok(JsValue::Undefined)
        }),
    );
}
