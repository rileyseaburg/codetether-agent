//! Backend runtime-policy gate for direct Bash tool invocation.

pub(super) fn bind_authority(args: &mut serde_json::Value, default: Option<&std::path::Path>) {
    if args.get("cwd").is_none()
        && let (Some(map), Some(path)) = (args.as_object_mut(), default)
    {
        map.insert("cwd".into(), path.display().to_string().into());
    }
    let allowed = crate::tool::network_access::allowed_for(args);
    crate::tool::network_access::bind(args, allowed);
}

pub(super) async fn blocked(args: &serde_json::Value) -> Option<crate::tool::ToolResult> {
    if let Some(result) = crate::runtime_policy::evaluate_tool_invocation("bash", args).await {
        return Some(result);
    }
    if let Some(result) = crate::tool::command_workdir::result("bash", args) {
        return Some(result);
    }
    crate::tool::shell_command_guard::result_for_args("bash", args)
}