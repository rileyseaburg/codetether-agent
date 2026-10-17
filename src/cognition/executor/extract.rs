//! Live thinker invocation followed by serial tool-request dispatch.
use super::*;

/// Execute tool requests extracted from a Test-phase thought.
///
/// Returns ThoughtEvents for each tool execution result.
pub async fn execute_tool_requests(
    thinker: Option<&ThinkerClient>,
    tool_registry: &Arc<ToolRegistry>,
    persona_id: &str,
    thought_text: &str,
    allowed_tools: &[String],
) -> Vec<ThoughtEvent> {
    let Some(client) = thinker else {
        return Vec::new();
    };

    let (system_prompt, user_prompt) = requests::extraction_prompts(thought_text, allowed_tools);
    let output = match client.think(&system_prompt, &user_prompt).await {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    let parsed = requests::parse_tool_requests(&output.text);
    let mut events = Vec::new();
    for request in parsed {
        events.push(
            super::run::execute_request(tool_registry, persona_id, allowed_tools, request).await,
        );
    }
    events
}
