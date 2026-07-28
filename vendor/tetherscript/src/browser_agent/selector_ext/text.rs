//! Text predicates for selector extensions.

use crate::browser::{text_content, Node};
use crate::browser_agent::text_match;

pub(crate) fn has_text(node: &Node, expected: &str) -> bool {
    text_match::contains(&text_content(node), expected)
}
