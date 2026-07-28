use super::*;

mod collection;
mod collection_methods;
mod collection_sync;
mod handles;
mod mutation;
mod objects;
mod option;
mod owner;
mod props;
mod read;
mod setters;
mod state;
mod sync;
mod value;

#[cfg(test)]
mod tests;

pub(super) fn install(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    objects::register(handle, obj);
    sync::value_attr(handle);
    props::write(obj, handle);
    setters::install(obj, handle);
}

pub(super) fn install_option(obj: &Rc<RefCell<HashMap<String, JsValue>>>, handle: &DomHandle) {
    option::install(obj, handle);
}

pub(super) fn value(handle: &DomHandle) -> String {
    read::value(handle)
}

pub(super) fn sync_tree(handle: &DomHandle) {
    sync::tree(handle);
}

pub(super) fn reset() {
    objects::reset();
    state::reset();
}
