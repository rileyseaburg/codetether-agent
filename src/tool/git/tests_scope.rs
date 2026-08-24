//! Signed workspace metadata for Git tool tests.

pub(crate) fn scoped(cwd: &str, mut args: serde_json::Value) -> serde_json::Value {
    args["cwd"] = serde_json::json!(cwd);
    args["__ct_parent_workspace"] = serde_json::json!(cwd);
    args["__ct_session_id"] = serde_json::json!("git-tool-test");
    crate::tool::network_access::bind_trusted(&mut args, false);
    args
}
