//! Native window event dispatchers used by JavaScript bootstrap functions.

use super::*;

pub(super) fn install(window: &JsValue) {
    let JsValue::Object(obj) = window else {
        return;
    };
    install_one(obj, "__tsDispatchScroll", "scroll", window.clone());
    install_one(obj, "__tsDispatchResize", "resize", window.clone());
}

fn install_one(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    name: &'static str,
    event_type: &'static str,
    window: JsValue,
) {
    obj.borrow_mut().insert(
        name.into(),
        native(name, Some(0), move |_| {
            dispatch_window_lifecycle(&window, event_type)?;
            Ok(JsValue::Undefined)
        }),
    );
}
