use super::*;

pub(super) fn change(handle: &DomHandle, event: &str) -> Result<(), String> {
    handle.dispatch_event(JsValue::String(event.into()))?;
    Ok(())
}

pub(super) fn change_or_document(
    handle: &DomHandle,
    document: &DomHandle,
    event: &str,
) -> Result<(), String> {
    if handle.node().is_some() {
        change(handle, event)
    } else {
        change(document, event)
    }
}

pub(super) fn error(handle: &DomHandle, event: &str) -> Result<(), String> {
    let document = DomHandle {
        root: handle.root.clone(),
        path: Vec::new(),
    };
    change_or_document(handle, &document, event)
}
