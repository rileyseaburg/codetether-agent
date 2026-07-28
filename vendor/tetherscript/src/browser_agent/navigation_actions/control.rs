//! Successful-control value extraction.

use crate::browser::{self, Element, Node};

pub(crate) fn values(node: &Node) -> Vec<(String, String)> {
    let Node::Element(element) = node else {
        return Vec::new();
    };
    if element.attrs.contains_key("disabled") {
        return Vec::new();
    }
    let Some(name) = element.attrs.get("name").cloned() else {
        return Vec::new();
    };
    match element.tag.as_str() {
        "input" => input_values(element, name),
        "textarea" => vec![(name, browser::text_content(node))],
        "select" => super::select::values(element, name),
        _ => Vec::new(),
    }
}

pub(crate) fn submitter(element: &Element) -> Option<(String, String)> {
    let name = element.attrs.get("name")?.clone();
    let value = element
        .attrs
        .get("value")
        .cloned()
        .unwrap_or_else(String::new);
    Some((name, value))
}

fn input_values(element: &Element, name: String) -> Vec<(String, String)> {
    let kind = element
        .attrs
        .get("type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into());
    if matches!(kind.as_str(), "submit" | "button" | "reset" | "file") {
        return Vec::new();
    }
    if matches!(kind.as_str(), "checkbox" | "radio") && !element.attrs.contains_key("checked") {
        return Vec::new();
    }
    let value = element.attrs.get("value").cloned().unwrap_or_else(|| {
        if kind == "checkbox" {
            "on".into()
        } else {
            String::new()
        }
    });
    vec![(name, value)]
}
