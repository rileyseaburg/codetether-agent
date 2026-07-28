use super::*;

#[path = "events/call.rs"]
mod call;
#[path = "events/dispatch.rs"]
mod dispatch;
#[path = "events/event.rs"]
mod event;
#[path = "events/listeners.rs"]
mod listeners;

pub(super) type ListenerList = Rc<RefCell<Vec<(String, JsValue)>>>;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let listeners = Rc::new(RefCell::new(Vec::new()));
    listeners::install(object, listeners.clone());
    dispatch::install_methods(object, listeners);
}
