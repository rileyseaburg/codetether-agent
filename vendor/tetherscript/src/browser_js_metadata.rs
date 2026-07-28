//! Browser identity, document metadata, and beacon shims.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::SharedBrowserJsRouteHandler;

#[path = "browser_js_metadata/beacon.rs"]
mod beacon;
#[path = "browser_js_metadata/document.rs"]
mod document;
#[path = "browser_js_metadata/navigator.rs"]
mod navigator;
#[path = "browser_js_metadata/scroll.rs"]
mod scroll;

pub(super) fn install(
    window: &mut HashMap<String, JsValue>,
    document: &JsValue,
    navigator: &JsValue,
    location: Rc<RefCell<HashMap<String, JsValue>>>,
    route_handler: SharedBrowserJsRouteHandler,
) {
    document::install(document, &location);
    navigator::install(navigator, route_handler);
    scroll::install(window);
}

fn href(location: &Rc<RefCell<HashMap<String, JsValue>>>) -> String {
    super::location_href(location)
}

fn set_str(target: &mut HashMap<String, JsValue>, name: &str, value: impl Into<String>) {
    target.insert(name.into(), JsValue::String(value.into()));
}
