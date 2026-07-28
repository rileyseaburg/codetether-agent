//! JavaScript snippets for focus traversal.

use crate::browser_agent::keyboard_escape::node;

pub(crate) fn focus(path: &[usize]) -> String {
    format!("let n={}; if(n.focus){{n.focus();}}", node(path))
}
