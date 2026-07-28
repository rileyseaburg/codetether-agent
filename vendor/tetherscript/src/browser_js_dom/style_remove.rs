use super::*;

pub(super) fn remove_prop(handle: &DomHandle, name: &str) -> Result<String, String> {
    let old = style_attr::get(handle, name);
    style_write::write(
        handle,
        style_attr::style_order::remove(&style_attr::raw(handle), name),
    )?;
    Ok(old)
}
