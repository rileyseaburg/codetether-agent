//! JavaScript snippets for keyboard actions.

use crate::browser_agent::keyboard::keyboard_default;
use crate::browser_agent::keyboard::KeyboardKey;
use crate::browser_agent::keyboard_escape::{node, quote};

pub(crate) fn press(path: &[usize], key: &KeyboardKey, replacement: Option<&str>) -> String {
    format!(
        "let n={}; n.focus(); let k={}; let __ts_key_event={{type:'keydown',key:k}}; \
         let ok=n.dispatchEvent(__ts_key_event); if(ok){{{}}} n.dispatchEvent({{type:'keyup',key:k}}); ok;",
        node(path),
        quote(&key.js_key()),
        keyboard_default::body(key, replacement)
    )
}
