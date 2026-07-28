//! Text accessibility leaf construction.

use crate::browser_agent::text_match;

use super::super::model::{AccessibilityNode, AccessibilityState};

pub(super) fn node(text: &str, path: Vec<usize>) -> Vec<AccessibilityNode> {
    let name = text_match::clean(text);
    if name.is_empty() {
        return Vec::new();
    }
    vec![AccessibilityNode {
        role: "text".into(),
        name,
        tag: "#text".into(),
        selector: path_key(&path),
        focusable: false,
        focus_index: None,
        states: AccessibilityState::default(),
        path,
        children: Vec::new(),
    }]
}

fn path_key(path: &[usize]) -> String {
    let joined = path
        .iter()
        .map(|index| index.to_string())
        .collect::<Vec<_>>()
        .join(".");
    format!("path:{joined}")
}
