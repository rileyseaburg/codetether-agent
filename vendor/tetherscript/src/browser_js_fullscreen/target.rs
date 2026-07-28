use super::*;

#[derive(Clone)]
pub(super) struct Target {
    pub(super) handle: DomHandle,
    object: Option<DomObject>,
}

impl Target {
    pub(super) fn new(handle: &DomHandle, slot: &DomObjectSlot) -> Self {
        Self {
            handle: handle.clone(),
            object: slot.borrow().clone(),
        }
    }

    pub(super) fn same(&self, other: &Self) -> bool {
        self.same_handle(&other.handle)
    }

    pub(super) fn same_handle(&self, handle: &DomHandle) -> bool {
        Rc::ptr_eq(&self.handle.root, &handle.root) && self.handle.path == handle.path
    }

    pub(super) fn value(&self) -> JsValue {
        if self.handle.node().is_none() {
            return JsValue::Null;
        }
        self.object
            .as_ref()
            .map(|object| JsValue::Object(object.clone()))
            .unwrap_or_else(|| node_object(self.handle.clone()))
    }
}
