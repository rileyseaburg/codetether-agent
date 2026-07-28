//! JavaScript snippets for clipboard-backed actions.

use crate::browser_agent::keyboard_escape::{node, quote};

pub(crate) fn copy(path: &[usize]) -> String {
    format!("let n={}; n.dispatchEvent({{type:'copy'}});", node(path))
}

pub(crate) fn paste(path: &[usize], text: &str) -> String {
    format!(
        "let n={}; n.focus(); let e={{type:'paste'}}; \
         if(n.dispatchEvent(e)){{n.inputText({});}}",
        node(path),
        quote(text)
    )
}
