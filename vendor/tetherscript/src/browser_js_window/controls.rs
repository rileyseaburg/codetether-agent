//! Native window control and deterministic probe method installers.

use super::*;

mod methods;
mod popup;
mod probes;

type WindowObject = Rc<RefCell<HashMap<String, JsValue>>>;

pub(super) fn install(window: &JsValue) {
    let JsValue::Object(obj) = window else {
        return;
    };
    probes::install(obj);
    popup::install(obj, window.clone());
}
