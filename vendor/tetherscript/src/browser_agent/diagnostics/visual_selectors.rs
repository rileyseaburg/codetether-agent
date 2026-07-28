//! Selector candidates for visual evidence records.

use crate::browser::Element;

pub fn candidates(element: &Element, path: &[usize]) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(selector) = id_selector(element) {
        out.push(selector);
    }
    if let Some(selector) = class_selector(element) {
        out.push(selector);
    }
    out.push(path_selector(element, path));
    out
}

fn id_selector(element: &Element) -> Option<String> {
    element
        .attrs
        .get("id")
        .filter(|id| !id.trim().is_empty())
        .map(|id| format!("#{id}"))
}

fn class_selector(element: &Element) -> Option<String> {
    let class = element
        .attrs
        .get("class")?
        .split_whitespace()
        .find(|part| !part.is_empty())?;
    Some(format!("{}.{}", element.tag, class))
}

fn path_selector(element: &Element, path: &[usize]) -> String {
    let path = path
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!("{}:nth-path({path})", element.tag)
}
