//! Human-readable command approval summaries.

use serde_json::Value;

pub(super) fn render(args: &Value, key: &str) -> Option<String> {
    let command = super::field(args, key)?;
    let cwd = super::field(args, "workdir")
        .or_else(|| super::field(args, "cwd"))
        .or_else(|| super::field(args, "__ct_parent_workspace"))
        .unwrap_or_else(|| "process workspace".into());
    let network = if crate::tool::network_access::allowed_for(args) {
        "allowed"
    } else {
        "isolated"
    };
    let sandbox = super::field(args, "sandbox_permissions").unwrap_or_else(|| "sandboxed".into());
    Some(format!(
        "run: {}; cwd: {}; network: {network}; authority: {sandbox}",
        super::clip(&command),
        super::clip(&cwd),
    ))
}
