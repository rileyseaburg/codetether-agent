use super::*;

#[path = "style_order.rs"]
pub(super) mod style_order;

pub(super) fn css_text(handle: &DomHandle) -> String {
    style_order::normalize(&raw(handle))
}

pub(super) fn get(handle: &DomHandle, name: &str) -> String {
    read(handle)
        .get(&name.to_ascii_lowercase())
        .cloned()
        .unwrap_or_default()
}

pub(super) fn set_css_text(handle: &DomHandle, text: String) -> Result<String, String> {
    let text = style_order::normalize(&text);
    write(handle, text.clone())?;
    Ok(text)
}

pub(super) fn set_prop(handle: &DomHandle, name: &str, value: String) -> Result<(), String> {
    write(handle, style_order::set(&raw(handle), name, &value))
}

pub(super) fn read(handle: &DomHandle) -> HashMap<String, String> {
    match handle.node() {
        Some(Node::Element(el)) => el
            .attrs
            .get("style")
            .map(|style| browser::parse_inline_style(style))
            .unwrap_or_default(),
        _ => HashMap::new(),
    }
}

pub(super) fn raw(handle: &DomHandle) -> String {
    match handle.node() {
        Some(Node::Element(el)) => el.attrs.get("style").cloned().unwrap_or_default(),
        _ => String::new(),
    }
}

fn write(handle: &DomHandle, text: String) -> Result<(), String> {
    style_write::write(handle, text)
}
