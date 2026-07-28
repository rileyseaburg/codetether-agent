use super::super::*;

#[path = "orientation_events_dispatch.rs"]
mod dispatch;
#[path = "orientation_events_listeners.rs"]
mod listeners;

pub(super) type Listeners = Rc<RefCell<Vec<JsValue>>>;

pub(super) fn install(object: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let listeners: Listeners = Rc::new(RefCell::new(Vec::new()));
    listeners::install(object, listeners.clone());
    dispatch::install(object, listeners);
}
