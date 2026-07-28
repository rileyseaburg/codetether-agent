//! JavaScript snippets for form-control actions.

use crate::browser_agent::keyboard_escape::{node, quote};

pub(crate) fn check(path: &[usize]) -> String {
    format!("let n={}; if(!n.checked){{n.click();}}", node(path))
}

pub(crate) fn uncheck(path: &[usize], kind: &str) -> String {
    if kind == "radio" {
        return format!(
            "let n={}; n.checked=false; n.dispatchEvent('input'); n.dispatchEvent('change');",
            node(path)
        );
    }
    format!("let n={}; n.click(); n.checked=false;", node(path))
}

pub(crate) fn select_option(path: &[usize], value: &str) -> String {
    format!(
        "let n={}; if(n.focus){{n.focus();}} n.value={}; \
         n.dispatchEvent('input'); n.dispatchEvent('change');",
        node(path),
        quote(value)
    )
}
