use super::*;

#[path = "class_list_replace.rs"]
mod replace;
#[path = "class_list_toggle.rs"]
mod toggle;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    replace::install(obj, handle, weak);
    toggle::install(obj, handle, weak);
}
