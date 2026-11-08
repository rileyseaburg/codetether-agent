//! Small initial schema set with web research available without discovery.
use codetether_provider_core::ToolDefinition;

pub(super) fn retain_tools(definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    definitions
        .into_iter()
        .filter(|tool| {
            matches!(
                tool.name.as_str(),
                "read"
                    | "list"
                    | "exec_command"
                    | "write_stdin"
                    | "apply_patch"
                    | "get_goal"
                    | "session_task"
                    | "question"
                    | "websearch"
                    | "webfetch"
            )
        })
        .collect()
}
