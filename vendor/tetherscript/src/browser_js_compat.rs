//! Focused browser script-compatibility host shims.

use super::*;

#[path = "browser_js_compat/base64.rs"]
mod base64;
#[path = "browser_js_compat/blob.rs"]
mod blob;
#[path = "browser_js_compat/bytes.rs"]
mod bytes;
#[path = "browser_js_compat/clipboard_item.rs"]
mod clipboard_item;
#[path = "browser_js_compat/crypto.rs"]
mod crypto;
#[path = "browser_js_compat/dom_constructors.rs"]
mod dom_constructors;
#[path = "browser_js_compat/dom_exception.rs"]
mod dom_exception;
#[path = "browser_js_compat/events.rs"]
mod events;
#[path = "browser_js_compat/file_picker.rs"]
mod file_picker;
#[path = "browser_js_compat/file_reader.rs"]
mod file_reader;
#[path = "browser_js_compat/form_data.rs"]
mod form_data;
#[path = "browser_js_compat/installer.rs"]
mod installer;
#[path = "browser_js_compat/notification.rs"]
mod notification;
#[path = "browser_js_compat/promise.rs"]
pub(super) mod promise;
#[path = "browser_js_compat/structured.rs"]
mod structured;
#[path = "browser_js_compat/text.rs"]
mod text;
#[path = "browser_js_compat/typed_array.rs"]
mod typed_array;
#[path = "browser_js_compat/url_pattern.rs"]
mod url_pattern;

#[cfg(test)]
#[path = "browser_js_compat/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "browser_js_compat/tests_notification.rs"]
mod tests_notification;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    installer::install(window);
}

pub(super) fn structured_clone(value: &JsValue) -> Result<JsValue, String> {
    structured::clone_value(value)
}
