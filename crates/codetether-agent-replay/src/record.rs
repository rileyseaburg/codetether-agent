//! Tool usage, feedback and image-message replay.
use crate::*;
/// Append executed tool results to the session history.
/// Append tool usage and tool-role messages in paired call/result order.
/// # Arguments
/// Mutable session, original calls and execution results.
/// # Returns
/// Records each paired tool call, including any returned images.
/// # Examples
/// ```rust,no_run
/// # use codetether_agent_replay::*;
/// # async fn example<H: Host>(host: &H, session: &mut H::Session) {
/// record_results::<H>(session,vec![],vec![]);
/// # }
/// ```
pub fn record_results<H: Host>(
    session: &mut H::Session,
    tool_calls: Vec<ContentPart>,
    results: Vec<ToolResult>,
) {
    for (part, result) in tool_calls.into_iter().zip(results) {
        let Some(call) = part.as_tool_call() else {
            continue;
        };
        let (id, name, arguments) = (
            call.id.to_string(),
            call.name.to_string(),
            call.arguments.to_string(),
        );
        H::record_tool(
            session,
            ToolUse {
                id: id.clone(),
                name: name.clone(),
                input: arguments,
                output: result.output.clone(),
                success: result.success,
            },
        );
        let mut content = vec![ContentPart::ToolResult {
            tool_call_id: id,
            content: codetether_tool_core::feedback::render(&name, result.success, &result.output),
        }];
        content.extend(crate::images::tool_images(&result));
        H::add_message(
            session,
            Message {
                role: Role::Tool,
                content,
            },
        );
    }
}
