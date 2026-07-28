//! DOM-backed edit calculations for keyboard actions.

use crate::browser::{self, Node};
use crate::browser_agent::keyboard::KeyboardKey;
use crate::browser_agent::resolve::Resolved;

pub(crate) fn replacement(resolved: &Resolved, key: &KeyboardKey) -> Option<String> {
    match key {
        KeyboardKey::Backspace => Some(backspace(input_value(resolved))),
        _ => None,
    }
}

fn input_value(resolved: &Resolved) -> String {
    if resolved.dom.element.tag == "textarea" {
        return browser::text_content(&Node::Element(resolved.dom.element.clone()));
    }
    resolved
        .dom
        .element
        .attrs
        .get("value")
        .cloned()
        .unwrap_or_default()
}

fn backspace(mut value: String) -> String {
    value.pop();
    value
}
