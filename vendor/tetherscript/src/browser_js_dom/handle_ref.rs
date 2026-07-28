use super::*;

#[derive(Clone)]
pub(super) struct HandleRef {
    id: String,
    fallback: DomHandle,
}

pub(super) fn new(obj: &HashMap<String, JsValue>, fallback: &DomHandle) -> HandleRef {
    let id = match obj.get("__domHandleId") {
        Some(JsValue::String(id)) => id.clone(),
        _ => String::new(),
    };
    HandleRef {
        id,
        fallback: fallback.clone(),
    }
}

impl HandleRef {
    pub(super) fn current(&self) -> DomHandle {
        DOM_HANDLE_REGISTRY
            .with(|registry| registry.borrow().get(&self.id).cloned())
            .unwrap_or_else(|| self.fallback.clone())
    }
}
