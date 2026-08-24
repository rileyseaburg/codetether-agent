//! Authoritative PRD content identity for Ralph approval resources.

use serde_json::Value;

pub(super) fn bind(tool: &str, args: &mut Value) {
    if tool == "ralph" {
        let _ = crate::tool::ralph::scope::bind(args);
    }
}
