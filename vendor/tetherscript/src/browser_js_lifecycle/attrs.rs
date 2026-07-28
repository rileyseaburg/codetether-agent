//! Metadata maps for navigation lifecycle events.

use std::collections::HashMap;

use crate::js::JsValue;

pub(super) fn state(value: JsValue) -> HashMap<String, JsValue> {
    let mut attrs = HashMap::new();
    attrs.insert("state".into(), value);
    attrs
}

pub(super) fn urls(old_url: &str, new_url: &str) -> HashMap<String, JsValue> {
    let mut attrs = HashMap::new();
    attrs.insert("oldURL".into(), JsValue::String(old_url.into()));
    attrs.insert("newURL".into(), JsValue::String(new_url.into()));
    attrs
}
