use super::*;

#[path = "events/dispatch.rs"]
mod dispatch;
#[path = "events/methods.rs"]
mod methods;

pub(super) type ListenerList = Rc<RefCell<Vec<(String, JsValue)>>>;

pub(super) fn dispatch(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &ListenerList,
    kind: &str,
) -> Result<(), String> {
    dispatch::dispatch(object, listeners, kind)
}

pub(super) fn install_methods(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: ListenerList,
) {
    methods::install_methods(object, listeners);
}
