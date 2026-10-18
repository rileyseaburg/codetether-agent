//! HTTP Server
//!
//! Main API server for the CodeTether Agent

use codetether_approval_http as approval_routes;
pub use codetether_http_auth as auth;
mod config_status;
mod http_router;
mod openai_http_host;
pub use codetether_authorization_policy as policy;
use codetether_openai_adapter::options as openai_model_options;
use codetether_openai_wire::stream as openai_stream;
mod session_realtime;
mod session_recovery;
mod session_routes;
mod startup_host;
use codetether_task_queue as task_queue;
use codetether_tool_discovery::contract as tool_contract;
mod worker_modules;

use crate::cli::ServeArgs;
use crate::cognition::CognitionRuntime;
use anyhow::Result;

use codetether_openai_adapter::{
    OpenAiApiResult, canonicalize_provider_name, convert_finish_reason, convert_openai_messages,
    convert_openai_tools, convert_response_message, make_openai_model_id,
    normalize_model_reference, openai_error, parse_openai_content_part, parse_openai_content_parts,
    parse_openai_tool_content,
};
use codetether_openai_wire::{
    OpenAiChatCompletionChoice, OpenAiChatCompletionMessage, OpenAiChatCompletionResponse,
    OpenAiRequestMessage, OpenAiRequestTool, OpenAiRequestToolCall, OpenAiRequestToolCallFunction,
    OpenAiRequestToolDefinition, OpenAiResponseToolCall, OpenAiResponseToolCallFunction,
    OpenAiStop, OpenAiUsage, openai_stream_chunk,
};
pub use codetether_tool_discovery::{RegisterToolRequest, RegisteredTool, ToolRegistry};
pub use task_queue::{ClaimError, KnativeTask, KnativeTaskQueue};

/// HTTP request state using the application cognition runtime.
pub type AppState = codetether_server_state::AppState<CognitionRuntime>;
/// Start the HTTP server using the native runtime bindings.
pub async fn serve(args: ServeArgs) -> Result<()> {
    codetether_server_runtime::serve::<startup_host::AppHost>(
        &args.hostname,
        args.port,
        env!("CARGO_PKG_VERSION"),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::normalize_model_reference;
    use axum::http::StatusCode;

    #[test]
    fn normalize_model_reference_accepts_colon_format() {
        assert_eq!(
            normalize_model_reference("openai:gpt-4o"),
            "openai/gpt-4o".to_string()
        );
    }
}
