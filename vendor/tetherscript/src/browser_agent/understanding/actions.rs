//! Actionable element detection.

use super::ElementSummary;

#[derive(Clone, Debug)]
pub struct ActionableElement {
    pub selector: String,
    pub kind: String,
    pub label: String,
    pub role: Option<String>,
}

fn label(e: &ElementSummary) -> String {
    e.attr("aria-label")
        .or_else(|| e.attr("title"))
        .map(str::to_string)
        .unwrap_or_else(|| e.text.clone())
}

pub fn detect_actionable(elements: &[ElementSummary]) -> Vec<ActionableElement> {
    elements
        .iter()
        .filter_map(|e| {
            let tag = e.tag.to_ascii_lowercase();
            let role = e.role.clone().unwrap_or_default().to_ascii_lowercase();
            let kind = if tag == "button" || role == "button" {
                "button"
            } else if tag == "a" || role == "link" {
                "link"
            } else if ["input", "select", "textarea"].contains(&tag.as_str()) {
                "input"
            } else {
                return None;
            };
            Some(ActionableElement {
                selector: e.selector.clone(),
                kind: kind.into(),
                label: label(e),
                role: e.role.clone(),
            })
        })
        .collect()
}
