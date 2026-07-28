//! Channel and worker compatibility internals.

use super::*;

mod broadcast;
mod broadcast_methods;
mod broadcast_state;
mod delivery;
mod event;
mod listener_store;
mod listeners;
mod message_channel;
mod port;
mod worker;
mod worker_host;
mod worker_main;
mod worker_scope;
mod worker_scripts;

type JsObject = Rc<RefCell<HashMap<String, JsValue>>>;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    message_channel::install(window, timers.clone());
    broadcast::install(window, timers.clone());
    worker::install(window, timers);
}

pub(super) fn reset_all() {
    broadcast::reset_all();
    worker_scripts::reset_all();
}

fn closed(object: &JsObject) -> bool {
    matches!(object.borrow().get("__closed"), Some(JsValue::Bool(true)))
}

fn mark_closed(object: &JsObject) {
    object
        .borrow_mut()
        .insert("__closed".into(), JsValue::Bool(true));
}

fn location_origin(window: &HashMap<String, JsValue>) -> String {
    let Some(JsValue::Object(location)) = window.get("location") else {
        return "http://localhost".into();
    };
    location
        .borrow()
        .get("origin")
        .map(JsValue::display)
        .unwrap_or_else(|| "http://localhost".into())
}
