//! Text extraction for compact visual evidence.

use crate::browser::{self, Element, Node};

const MAX_TEXT: usize = 160;

pub fn element_text(element: &Element) -> String {
    let text = browser::text_content(&Node::Element(element.clone()));
    if text.len() <= MAX_TEXT {
        return text;
    }
    let mut end = MAX_TEXT;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &text[..end])
}
