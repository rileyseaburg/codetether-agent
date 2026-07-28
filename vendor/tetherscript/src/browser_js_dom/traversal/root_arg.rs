use super::*;

pub(super) fn root_arg(
    name: &str,
    value: Option<&JsValue>,
    fallback: Rc<RefCell<Node>>,
) -> Result<DomHandle, String> {
    match value.and_then(dom_handle_from_value) {
        Some(handle) => Ok(handle),
        None if value.is_none() => Ok(DomHandle {
            root: fallback,
            path: Vec::new(),
        }),
        None => Err(format!("document.{}: expected root node", name)),
    }
}
