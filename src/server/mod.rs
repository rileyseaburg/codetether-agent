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
mod task_http_state;
mod task_ingest_state;
mod task_queue;
mod tool_contract;
mod tool_discovery_host;
mod voice_http_state;
mod worker_modules;

use crate::a2a;
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::{AgentBus, BusEnvelope};
use crate::cli::ServeArgs;
use crate::cognition::CognitionRuntime;
use crate::config::Config;
use crate::k8s::K8sManager;
use crate::tool::{PluginRegistry as AgentPluginRegistry, hash_bytes};
use anyhow::Result;
use auth::AuthState;
use futures::StreamExt;
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

/// Start the HTTP server
pub async fn serve(args: ServeArgs) -> Result<()> {
    let t0 = std::time::Instant::now();
    tracing::info!("[startup] begin");
    let config = Config::load().await?;
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] config loaded"
    );
    let mut cognition = CognitionRuntime::new_from_env();
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] cognition runtime created"
    );

    // Set up tool registry for cognition execution engine.
    let runtime_tools = Arc::new(crate::tool::ToolRegistry::with_defaults());
    let plugin_registry = runtime_tools.plugins().clone();
    cognition.set_tools(runtime_tools);
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] tools registered"
    );
    let cognition = Arc::new(cognition);

    // Initialize audit log.
    let audit_log = AuditLog::from_env();
    let _ = audit::init_audit_log(audit_log.clone());

    // Initialize K8s manager.
    tracing::info!(elapsed_ms = t0.elapsed().as_millis(), "[startup] pre-k8s");
    let k8s = Arc::new(K8sManager::new().await);
    tracing::info!(elapsed_ms = t0.elapsed().as_millis(), "[startup] k8s done");
    if k8s.is_available() {
        tracing::info!("K8s self-deployment enabled");
    }

    // Initialize mandatory auth.
    let auth_state = AuthState::from_env();
    tracing::info!(
        token_len = auth_state.token().len(),
        "Auth is mandatory. Only /health is served without a Bearer token."
    );
    tracing::info!(
        audit_entries = audit_log.count().await,
        "Audit log initialized"
    );

    // Create agent bus for in-process communication
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());

    // Auto-start S3 sink if MinIO is configured (set MINIO_ENDPOINT to enable)
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] bus created"
    );

    if cognition.is_enabled() && env_bool("CODETETHER_COGNITION_AUTO_START", true) {
        tracing::info!(
            elapsed_ms = t0.elapsed().as_millis(),
            "[startup] auto-starting cognition"
        );
        if let Err(error) = cognition.start(None).await {
            tracing::warn!(%error, "Failed to auto-start cognition loop");
        } else {
            tracing::info!("Perpetual cognition auto-started");
        }
    }

    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] building routes"
    );
    let addr = format!("{}:{}", args.hostname, args.port);

    // Build the agent card
    let agent_card = a2a::server::A2AServer::default_card(&format!("http://{}", addr));
    let a2a_server = a2a::server::A2AServer::with_bus(agent_card.clone(), bus.clone());

    // Build A2A router separately
    let a2a_router = a2a_server.clone().router();
    let a2a_nested_router = a2a_server.router();

    // Start gRPC transport on a separate port
    codetether_a2a_grpc::listener::spawn(&args.hostname, agent_card, bus.clone())?;

    let state = AppState {
        config: Arc::new(config),
        cognition,
        audit_log,
        k8s,
        auth: auth_state.clone(),
        bus,
        knative_tasks: KnativeTaskQueue::new(),
        tool_registry: ToolRegistry::new(),
        plugin_registry,
        server_fingerprint: hash_bytes(env!("CARGO_PKG_VERSION").as_bytes()),
    };

    // Spawn the tool reaper background task (runs every 15s to clean up expired tools)
    codetether_tool_discovery::spawn_reaper(state.tool_registry.clone());

    let app = http_router::build(&state, a2a_router, a2a_nested_router);

    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] router built, binding"
    );
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] listening on http://{}",
        addr
    );

    axum::serve(listener, app).await?;

    Ok(())
}
use codetether_util::environment::env_bool;

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
