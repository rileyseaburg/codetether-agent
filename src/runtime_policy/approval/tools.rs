//! Tools that atomically claim authority at their own side-effect boundary.

#[path = "tools_collaboration.rs"]
mod collaboration;
#[path = "tools_networked.rs"]
mod networked;

pub(crate) fn self_verifying(tool_name: &str) -> bool {
    tool_name.starts_with("mcp:")
        || tool_name.starts_with("mcp__")
        || matches!(
            tool_name,
            "bash"
                | "exec_command"
                | "apply_patch"
                | "patch"
                | "mcp"
                | "mcp_bridge"
                | "undo"
                | "tetherscript_plugin"
                | "computer_use"
                | "write_stdin"
                | "git"
                | "browserctl"
                | "write"
                | "edit"
                | "multiedit"
                | "confirm_edit"
                | "confirm_multiedit"
                | "todowrite"
                | "todo_write"
                | "go"
                | "ralph"
                | "swarm_execute"
                | "agent"
                | "relay_autochat"
                | "mux_control"
        )
        || collaboration::self_verifying(tool_name)
        || networked::self_verifying(tool_name)
}
