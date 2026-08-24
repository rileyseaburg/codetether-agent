//! Signed policy arguments for one MCP subprocess boundary.

use serde_json::json;

pub(super) fn scoped(
    command: &str,
    args: &[&str],
    approval_id: Option<&str>,
    network_allowed: bool,
    session_id: &str,
) -> serde_json::Value {
    let mut value = json!({
        "command": rendered(command, args),
        "argv": args,
        "__ct_session_id": session_id,
    });
    crate::tool::network_access::bind_trusted(&mut value, network_allowed);
    if let Some(approval_id) = approval_id.filter(|id| !id.trim().is_empty()) {
        value["approval_id"] = json!(approval_id);
    }
    value
}

fn rendered(command: &str, args: &[&str]) -> String {
    std::iter::once(command)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
pub(super) fn policy_args(command: &str, args: &[&str], id: Option<&str>) -> serde_json::Value {
    scoped(
        command,
        args,
        id,
        crate::tool::network_access::allowed(),
        "mcp-test",
    )
}
