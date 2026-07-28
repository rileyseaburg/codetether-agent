//! Window scroll aliases for scripts that probe page offsets.

use std::collections::HashMap;

use crate::js::JsValue;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    for name in ["scrollX", "scrollY", "pageXOffset", "pageYOffset"] {
        window.insert(name.into(), JsValue::Number(0.0));
    }
}
