//! Query extraction from router context.

use crate::router::CrateAutoProcessContext;

/// Build the user-facing question for an RLM request.
pub fn extract(ctx: &CrateAutoProcessContext<'_>) -> String {
    if let Some(q) = ctx.tool_args.get("query").and_then(|v| v.as_str()) {
        return q.to_string();
    }
    if let Some(p) = ctx.tool_args.get("pattern").and_then(|v| v.as_str()) {
        return format!("Find all occurrences of '{p}'");
    }
    match ctx.tool_id {
        "session_context" | "context_reset" | "proactive_session_context" | "summary_index" => {
            crate::router::build_query_for_tool(ctx.tool_id, &ctx.tool_args)
        }
        _ => default_query(ctx.tool_id),
    }
}

fn default_query(tool_id: &str) -> String {
    match tool_id {
        "grep" => "Summarize and group the search matches.".into(),
        "read" => "Summarize the file structure and important logic.".into(),
        "bash" => "Summarize errors, warnings, and key command output.".into(),
        _ => "Summarize the most important information.".into(),
    }
}
