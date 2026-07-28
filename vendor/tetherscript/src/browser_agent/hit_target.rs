//! Hit-test target values.

use crate::browser::Element;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HitTarget {
    pub path: Vec<usize>,
    pub label: String,
}

impl HitTarget {
    pub(crate) fn new(path: Vec<usize>, element: &Element) -> Self {
        Self {
            path,
            label: label(element),
        }
    }
}

fn label(element: &Element) -> String {
    element
        .attrs
        .get("id")
        .map(|id| format!("{}#{id}", element.tag))
        .unwrap_or_else(|| element.tag.clone())
}
