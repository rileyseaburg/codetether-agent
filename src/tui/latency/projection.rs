//! Project native request and tool state into the shared inspector.
use crate::tui::app::state::App;
use codetether_latency_view::{LatencyView, ToolLatency};
pub(super) fn view(app: &App) -> LatencyView<'_> {
    LatencyView {
        cwd_display: app.state.cwd_display.clone(),
        processing: app.state.processing,
        current_request_first_token_ms: app.state.request_timing.current_request_first_token_ms,
        last_request_first_token_ms: app.state.request_timing.last_request_first_token_ms,
        last_request_last_token_ms: app.state.request_timing.last_request_last_token_ms,
        last_completion_latency_ms: app.state.last_completion_latency_ms,
        last_completion_model: app.state.last_completion_model.clone(),
        last_completion_prompt_tokens: app.state.last_completion_prompt_tokens,
        last_completion_output_tokens: app.state.last_completion_output_tokens,
        last_tool_latency_ms: app.state.tool_timing.last_tool_latency_ms,
        last_tool_name: app.state.tool_timing.last_tool_name.clone(),
        last_tool_success: app.state.tool_timing.last_tool_success,
        status: app.state.status.clone(),
        current_request_elapsed_ms: app.state.current_request_elapsed_ms(),
        chat_latency: &app.state.request_timing.chat_latency,
        tools: ToolLatency::from_messages(&app.state.messages),
    }
}
