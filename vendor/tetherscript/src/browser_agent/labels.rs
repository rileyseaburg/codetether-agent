//! Form label association helpers.

use crate::browser::{text_content, Document, Element, Node};
use crate::browser_agent::text_match::clean;

pub(crate) fn label_text(
    document: &Document,
    element: &Element,
    ancestors: &[Element],
) -> Option<String> {
    if !is_labelable(element) {
        return None;
    }
    by_for(document, element).or_else(|| wrapped(ancestors))
}

fn by_for(document: &Document, element: &Element) -> Option<String> {
    let id = element.attrs.get("id")?;
    label_for(&document.children, id)
}

fn label_for(nodes: &[Node], id: &str) -> Option<String> {
    for node in nodes {
        if let Node::Element(element) = node {
            if element.tag == "label" && element.attrs.get("for").is_some_and(|for_id| for_id == id)
            {
                return Some(clean(&text_content(node)));
            }
            if let Some(text) = label_for(&element.children, id) {
                return Some(text);
            }
        }
    }
    None
}

fn wrapped(ancestors: &[Element]) -> Option<String> {
    ancestors.iter().rev().find_map(|element| {
        (element.tag == "label").then(|| clean(&text_content(&Node::Element(element.clone()))))
    })
}

fn is_labelable(element: &Element) -> bool {
    matches!(
        element.tag.as_str(),
        "button" | "input" | "meter" | "output" | "progress" | "select" | "textarea"
    )
}
