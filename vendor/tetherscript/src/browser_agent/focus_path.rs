//! Stable focus target keys.

use crate::browser::Element;

pub(crate) fn selector_for(path: &[usize], element: &Element) -> String {
    element
        .attrs
        .get("id")
        .filter(|id| !id.is_empty())
        .map(|id| format!("#{id}"))
        .unwrap_or_else(|| path_key(path))
}

pub(crate) fn path_key(path: &[usize]) -> String {
    let indexes = path
        .iter()
        .map(|index| index.to_string())
        .collect::<Vec<_>>()
        .join(".");
    format!("path:{indexes}")
}
