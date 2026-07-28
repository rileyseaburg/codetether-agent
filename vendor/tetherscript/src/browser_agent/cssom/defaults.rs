//! Minimal deterministic user-agent style defaults.

use std::collections::BTreeMap;

pub(crate) fn apply(tag: &str, properties: &mut BTreeMap<String, String>) {
    insert(properties, "display", display_for(tag));
    insert(properties, "position", "static");
    insert(properties, "visibility", "visible");
    insert(properties, "opacity", "1");
    insert(properties, "box-sizing", "content-box");
    insert(properties, "color", "black");
    insert(properties, "font-size", "16px");
}

fn insert(properties: &mut BTreeMap<String, String>, name: &str, value: &str) {
    properties
        .entry(name.to_string())
        .or_insert_with(|| value.to_string());
}

fn display_for(tag: &str) -> &'static str {
    match tag {
        "a" | "abbr" | "b" | "button" | "em" | "i" | "img" | "input" | "label" | "select"
        | "span" | "strong" | "textarea" => "inline-block",
        "script" | "style" | "template" => "none",
        _ => "block",
    }
}
