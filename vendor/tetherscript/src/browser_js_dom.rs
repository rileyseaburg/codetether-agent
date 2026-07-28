//! Focused DOM compatibility host shims.

use super::*;

#[allow(dead_code)]
#[path = "browser_js_dom/attr_update.rs"]
mod attr_update;
#[path = "browser_js_dom/construct.rs"]
mod construct;
#[path = "browser_js_dom/convenience.rs"]
mod convenience;
#[path = "browser_js_dom/dialog/mod.rs"]
mod dialog;
#[path = "browser_js_dom/document.rs"]
mod document;
#[path = "browser_js_dom/file_input/mod.rs"]
mod file_input;
#[path = "browser_js_dom/form_validation/mod.rs"]
pub(super) mod form_validation;
#[path = "browser_js_dom/observer/mod.rs"]
pub(crate) mod observer;
#[path = "browser_js_dom/ops.rs"]
mod ops;
#[path = "browser_js_dom/parser.rs"]
mod parser;
#[path = "browser_js_dom/popover/mod.rs"]
mod popover;
#[path = "browser_js_dom/serializer.rs"]
mod serializer;
#[path = "browser_js_dom/template.rs"]
mod template;
#[path = "browser_js_dom/traversal/mod.rs"]
mod traversal;

pub(super) fn install_window(window: &mut HashMap<String, JsValue>) {
    parser::install(window);
    serializer::install(window);
    traversal::install_window(window);
}

pub(super) fn install_node(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    document::install(obj, handle, node);
    template::install(obj, node);
    dialog::install(obj, handle, node);
    popover::install(obj, handle, node);
    file_input::install(obj, node);
    convenience::install(obj, handle, node);
    traversal::install_node(obj, handle, node);
}

pub(super) fn install_live_node(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    node: &Node,
) {
    form_validation::install(obj, handle, node);
}
