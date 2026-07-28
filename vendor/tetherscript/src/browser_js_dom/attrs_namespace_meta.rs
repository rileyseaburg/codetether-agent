use std::cell::RefCell;

use super::super::super::*;

pub(super) struct AttrNs {
    pub(super) qualified: String,
    pub(super) local: String,
    pub(super) namespace: Option<String>,
}

thread_local! {
    static ATTRS: RefCell<Vec<(String, AttrNs)>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn set(handle: &DomHandle, attr: AttrNs) {
    let key = key(handle);
    ATTRS.with(|attrs| {
        let mut attrs = attrs.borrow_mut();
        attrs.retain(|(item_key, item)| {
            item_key != &key || item.namespace != attr.namespace || item.local != attr.local
        });
        attrs.push((key, attr));
    });
}

pub(super) fn remove(
    handle: &DomHandle,
    namespace: &Option<String>,
    local: &str,
) -> Option<String> {
    let key = key(handle);
    ATTRS.with(|attrs| {
        let mut attrs = attrs.borrow_mut();
        let index = attrs.iter().position(|(item_key, item)| {
            item_key == &key && &item.namespace == namespace && item.local == local
        })?;
        Some(attrs.remove(index).1.qualified)
    })
}

pub(super) fn find(handle: &DomHandle, namespace: &Option<String>, local: &str) -> Option<String> {
    let key = key(handle);
    ATTRS.with(|attrs| {
        attrs
            .borrow()
            .iter()
            .find(|(item_key, item)| {
                item_key == &key && &item.namespace == namespace && item.local == local
            })
            .map(|(_, item)| item.qualified.clone())
    })
}

fn key(handle: &DomHandle) -> String {
    let path = handle.path.iter().map(usize::to_string).collect::<Vec<_>>();
    format!("{:p}:{}", std::rc::Rc::as_ptr(&handle.root), path.join("."))
}
