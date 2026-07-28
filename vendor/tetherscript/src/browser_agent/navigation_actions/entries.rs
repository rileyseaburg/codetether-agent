//! Form entry-list collection.

use crate::browser::{Element, Node};

pub(crate) fn collect(form: &Element, submitter: Option<&Element>) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    for child in &form.children {
        walk(child, &mut entries);
    }
    if let Some(entry) = submitter.and_then(super::control::submitter) {
        entries.push(entry);
    }
    entries
}

fn walk(node: &Node, entries: &mut Vec<(String, String)>) {
    entries.extend(super::control::values(node));
    let Node::Element(element) = node else {
        return;
    };
    for child in &element.children {
        walk(child, entries);
    }
}
