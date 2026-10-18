use super::Registry;
use crate::{
    ThoughtEvent,
    execution::{
        CapabilityLease, events,
        requests::{ExtractedToolRequest, validate_tool_args},
    },
    text::trim_for_storage,
};
use std::sync::Arc;
pub(super) async fn execute_request<R: Registry + ?Sized>(
    tool_registry: &Arc<R>,
    persona_id: &str,
    allowed_tools: &[String],
    request: ExtractedToolRequest,
) -> ThoughtEvent {
    if !allowed_tools.contains(&request.tool) {
        return events::not_allowed(persona_id, request.tool);
    }
    let tool = match tool_registry.get(&request.tool) {
        Some(t) => t,
        None => return events::not_found(persona_id, request.tool),
    };
    let schema = tool.parameters();
    if let Err(validation_error) = validate_tool_args(&schema, &request.arguments) {
        return events::invalid_args(persona_id, request.tool, validation_error);
    }
    let lease = CapabilityLease::new(persona_id, &request.tool, "policy");
    if !lease.is_valid() {
        return events::expired(persona_id, request.tool, lease.id);
    }
    // Execute the tool
    let result = tool.execute(request.arguments.clone()).await;

    let (result_text, success) = match result {
        Ok(tool_result) => (
            trim_for_storage(&tool_result.output, 500),
            tool_result.success,
        ),
        Err(e) => (format!("Error: {}", e), false),
    };

    events::executed(persona_id, request.tool, lease.id, success, result_text)
}
