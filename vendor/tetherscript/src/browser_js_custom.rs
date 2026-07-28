//! Custom elements and shadow-root host shims.

use super::*;

#[path = "browser_js_custom/api.rs"]
mod api;
#[path = "browser_js_custom/lifecycle.rs"]
mod lifecycle;
#[path = "browser_js_custom/registry.rs"]
mod registry;
#[path = "browser_js_custom/shadow.rs"]
mod shadow;
#[path = "browser_js_custom/upgrade.rs"]
mod upgrade;
#[path = "browser_js_custom/util.rs"]
mod util;
#[path = "browser_js_custom/wait.rs"]
mod wait;

#[cfg(test)]
#[path = "browser_js_custom/tests.rs"]
mod tests;

pub(super) fn install(window: &mut HashMap<String, JsValue>, root: Rc<RefCell<Node>>) {
    api::install(window, root);
}

pub(super) fn reset_all() {
    registry::reset();
    shadow::reset();
}

pub(super) fn construct_created(tag: &str, element: &JsValue) -> Result<(), String> {
    lifecycle::construct_created(tag, element)
}

pub(super) fn connected(handle: &DomHandle) -> Result<(), String> {
    lifecycle::connected(handle)
}

pub(super) fn disconnected(node: Node) -> Result<(), String> {
    lifecycle::disconnected(node)
}

pub(super) fn attribute_changed(
    handle: &DomHandle,
    name: &str,
    old_value: Option<String>,
    new_value: Option<String>,
) -> Result<(), String> {
    lifecycle::attribute_changed(handle, name, old_value, new_value)
}

pub(super) fn attach_shadow_root(host: &DomHandle, options: Option<&JsValue>) -> JsValue {
    shadow::attach(host, options)
}

pub(super) fn open_shadow_root_object(host: &DomHandle) -> JsValue {
    shadow::open_object(host)
}
