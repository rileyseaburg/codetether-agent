//! Anchor element navigation target extraction.

use crate::browser::Document;

pub(crate) fn href(document: &Document, path: &[usize]) -> Option<String> {
    let element = super::dom::element_at_path(document, path)?;
    if !element.tag.eq_ignore_ascii_case("a") || element.attrs.contains_key("download") {
        return None;
    }
    let href = element.attrs.get("href")?.trim();
    if href.is_empty() || href.to_ascii_lowercase().starts_with("javascript:") {
        return None;
    }
    Some(href.to_string())
}
