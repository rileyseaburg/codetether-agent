//! Minimal DOM Selection and Range host bindings.

use super::*;

mod common;
mod control;
mod edit;
mod edit_set;
mod editable_attr;
pub(super) mod editable_props;
mod editable_state;
mod extent;
mod props;
mod range_clone;
mod range_collapse;
mod range_object;
mod range_offsets;
mod range_select;
mod registry;
mod selection_mutation;
mod selection_object;
mod selection_read;
mod state;
mod text;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_contenteditable;
#[cfg(test)]
mod tests_range_clone;

pub(super) fn reset() {
    registry::reset();
}

pub(super) fn install_document(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    range_object::install_document(obj, handle);
    selection_object::install_document(obj, handle);
}

pub(super) fn install_window(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    selection_object::install_window(obj, handle);
}

pub(super) fn is_contenteditable(element: &Element) -> bool {
    editable_state::state(element).unwrap_or(false)
}

pub(super) fn replace_contenteditable_selection(handle: &DomHandle, text: &str) -> Option<String> {
    edit::replace(handle, text)
}
