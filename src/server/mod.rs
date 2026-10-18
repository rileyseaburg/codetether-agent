//! HTTP Server
//!
//! Main API server for the CodeTether Agent

mod approval_routes;
mod audit_http_state;
pub mod auth;
mod bus_http_state;
mod cognition_http_state;
mod config_status;
mod http_router;
mod k8s_http_state;
mod models_catalog;
mod openai_http_host;
mod openai_model_options;
mod openai_stream;
mod plugin_http_state;
pub mod policy;
mod session_realtime;
mod session_recovery;
mod session_routes;
mod startup_host;
mod task_http_state;
mod task_ingest_state;
mod task_queue;
mod tool_contract;
mod tool_discovery_host;
mod voice_http_state;
mod worker_modules;

use crate::audit::AuditLog;
use crate::bus::AgentBus;
use crate::cli::ServeArgs;
use crate::cognition::CognitionRuntime;
use crate::config::Config;
use crate::k8s::K8sManager;
use crate::tool::PluginRegistry as AgentPluginRegistry;
use anyhow::Result;
use auth::AuthState;
use std::sync::Arc;

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

/// Server state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub cognition: Arc<CognitionRuntime>,
    pub audit_log: AuditLog,
    pub k8s: Arc<K8sManager>,
    pub auth: AuthState,
    pub bus: Arc<AgentBus>,
    pub knative_tasks: KnativeTaskQueue,
    pub tool_registry: ToolRegistry,
    pub plugin_registry: AgentPluginRegistry,
    pub server_fingerprint: String,
}
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
