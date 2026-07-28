use super::*;

#[path = "class_list_add_remove.rs"]
mod add_remove;
#[path = "class_list_toggle_replace.rs"]
mod toggle_replace;

pub(super) fn install(obj: &mut ListMap, handle: &DomHandle, weak: &ListWeak) {
    add_remove::install(obj, handle, weak);
    toggle_replace::install(obj, handle, weak);
}

fn commit(handle: &DomHandle, weak: &ListWeak, tokens: Vec<String>) {
    tokens::set(handle, tokens);
    sync::refresh_weak(weak, handle);
}
