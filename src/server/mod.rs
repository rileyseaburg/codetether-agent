//! HTTP Server
//!
//! Main API server for the CodeTether Agent

mod approval_routes;
pub mod auth;
mod config_status;
mod models_catalog;
mod openai_model_options;
mod openai_stream;
pub mod policy;
mod session_realtime;
mod session_recovery;
mod session_routes;
mod task_dispatch;
mod task_output_ingest;
mod task_output_poll;
use task_output_ingest::agent_task_output;
use task_output_poll::get_agent_task_output;
mod task_output_stream;
mod task_queue;
mod tool_contract;
mod version_info;
mod worker_modules;

use crate::a2a;
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::{AgentBus, BusEnvelope};
use crate::cli::ServeArgs;
use crate::cognition::{
    AttentionItem, CognitionRuntime, CognitionStatus, CreatePersonaRequest, GlobalWorkspace,
    LineageGraph, MemorySnapshot, Proposal, ReapPersonaRequest, ReapPersonaResponse,
    SpawnPersonaRequest, StartCognitionRequest, StopCognitionRequest, beliefs::Belief,
    executor::DecisionReceipt,
};
use crate::config::Config;
use crate::k8s::K8sManager;
use crate::tool::{PluginRegistry as AgentPluginRegistry, hash_bytes};
use anyhow::Result;
use auth::AuthState;
use axum::{
    body::Body,
    extract::Path,
    extract::{Query, State},
    http::{HeaderMap, Method, Request, StatusCode},
    middleware::{self, Next},
    response::sse::{Event, KeepAlive, Sse},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use codetether_task_ingest::CloudEventResponse;
use futures::{StreamExt, stream};
use http::HeaderValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, broadcast};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use codetether_audit_http::AuditQuery;
use codetether_authorization_policy::policy_middleware;
use codetether_cognition_api::BeliefFilter;
use codetether_event_replay::{list_session_event_files, replay_session_events};
use codetether_k8s_api::{ScaleRequest, SpawnSubagentRequest};
use codetether_openai_adapter::{
    OpenAiApiError, OpenAiApiResult, canonicalize_provider_name, convert_finish_reason,
    convert_openai_messages, convert_openai_tools, convert_response_message, make_openai_model_id,
    normalize_model_reference, openai_bad_request, openai_error, openai_internal_error,
    parse_openai_content_part, parse_openai_content_parts, parse_openai_tool_content,
};
use codetether_openai_wire::{
    OpenAiChatCompletionChoice, OpenAiChatCompletionMessage, OpenAiChatCompletionRequest,
    OpenAiChatCompletionResponse, OpenAiRequestMessage, OpenAiRequestTool, OpenAiRequestToolCall,
    OpenAiRequestToolCallFunction, OpenAiRequestToolDefinition, OpenAiResponseToolCall,
    OpenAiResponseToolCallFunction, OpenAiStop, OpenAiUsage, openai_stream_chunk,
};
use codetether_task_api::{CreateAgentTaskRequest, ListAgentTasksQuery};
use codetether_tool_discovery::RegisterToolResponse;
use codetether_tool_discovery::RegisteredToolView;
pub use codetether_tool_discovery::{RegisterToolRequest, RegisteredTool, ToolRegistry};
use codetether_voice_api::{CreateVoiceSessionRequest, get_voice_session_rest, list_voices_rest};
use task_dispatch::dispatch_task;
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
async fn audit_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    codetether_audit_http::audit_middleware(&state.audit_log, request, next).await
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
    let grpc_port = std::env::var("CODETETHER_GRPC_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(50051);
    let grpc_addr: std::net::SocketAddr = format!("{}:{}", args.hostname, grpc_port).parse()?;
    let grpc_store = crate::a2a::grpc::GrpcTaskStore::with_bus(agent_card, bus.clone());
    let grpc_service = grpc_store.into_service();
    let voice_service = crate::a2a::voice_grpc::VoiceServiceImpl::new(bus.clone()).into_service();
    tokio::spawn(async move {
        tracing::info!(addr = %grpc_addr, "Starting gRPC A2A server");

        // Configure CORS for marketing site (production + local dev)
        let allowed_origins = [
            HeaderValue::from_static("https://codetether.run"),
            HeaderValue::from_static("https://api.codetether.run"),
            HeaderValue::from_static("https://docs.codetether.run"),
            HeaderValue::from_static("https://codetether.com"),
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://localhost:3001"),
        ];
        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::list(allowed_origins))
            .allow_methods(AllowMethods::any())
            .allow_headers(AllowHeaders::any())
            .expose_headers(tower_http::cors::ExposeHeaders::list([
                http::header::HeaderName::from_static("grpc-status"),
                http::header::HeaderName::from_static("grpc-message"),
            ]));

        if let Err(error) = tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(cors)
            .layer(GrpcWebLayer::new())
            .add_service(grpc_service)
            .add_service(voice_service)
            .serve(grpc_addr)
            .await
        {
            tracing::warn!(addr = %grpc_addr, error = ?error, "gRPC A2A server exited");
        }
    });

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
    let tool_registry = state.tool_registry.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(15));
        loop {
            interval.tick().await;
            tool_registry.cleanup().await;
            tracing::debug!("Tool reaper ran cleanup");
        }
    });

    let app = approval_routes::router()
        // Health check (public — auth exempt)
        .route("/health", get(health))
        // CloudEvent receiver for Knative Eventing (public — auth exempt)
        .route("/task", post(receive_task_event))
        // Knative task queue APIs
        .route("/v1/knative/tasks", get(list_knative_tasks))
        .route("/v1/knative/tasks/{task_id}", get(get_knative_task))
        .route(
            "/v1/knative/tasks/{task_id}/claim",
            post(claim_knative_task),
        )
        .route(
            "/v1/knative/tasks/{task_id}/complete",
            post(complete_knative_task),
        )
        // API routes
        .merge(session_routes::router())
        .merge(session_realtime::router())
        .merge(config_status::router())
        .route("/api/provider", get(list_providers))
        .route("/api/agent", get(list_agents))
        // OpenAI-compatible APIs
        .merge(models_catalog::router())
        .route("/v1/chat/completions", post(openai_chat_completions))
        // Perpetual cognition APIs
        .route("/v1/cognition/start", post(start_cognition))
        .route("/v1/cognition/stop", post(stop_cognition))
        .route("/v1/cognition/status", get(get_cognition_status))
        .route("/v1/cognition/stream", get(stream_cognition))
        .route("/v1/cognition/snapshots/latest", get(get_latest_snapshot))
        // Swarm persona lifecycle APIs
        .route("/v1/swarm/personas", post(create_persona))
        .route("/v1/swarm/personas/{id}/spawn", post(spawn_persona))
        .route("/v1/swarm/personas/{id}/reap", post(reap_persona))
        .route("/v1/swarm/lineage", get(get_swarm_lineage))
        // Belief, attention, governance, workspace APIs
        .route("/v1/cognition/beliefs", get(list_beliefs))
        .route("/v1/cognition/beliefs/{id}", get(get_belief))
        .route("/v1/cognition/attention", get(list_attention))
        .route("/v1/cognition/proposals", get(list_proposals))
        .route(
            "/v1/cognition/proposals/{id}/approve",
            post(approve_proposal),
        )
        .route("/v1/cognition/receipts", get(list_receipts))
        .route("/v1/cognition/workspace", get(get_workspace))
        .route("/v1/cognition/governance", get(get_governance))
        .route("/v1/cognition/personas/{id}", get(get_persona))
        // Audit trail API
        .route("/v1/audit", get(list_audit_entries))
        // Event stream replay API (for SOC 2, FedRAMP, ATO compliance)
        .route("/v1/audit/replay", get(replay_session_events))
        .route("/v1/audit/replay/index", get(list_session_event_files))
        // K8s self-deployment APIs
        .route("/v1/k8s/status", get(get_k8s_status))
        .route("/v1/k8s/scale", post(k8s_scale))
        .route("/v1/k8s/restart", post(k8s_restart))
        .route("/v1/k8s/pods", get(k8s_list_pods))
        .route("/v1/k8s/actions", get(k8s_actions))
        .route("/v1/k8s/subagent", post(k8s_spawn_subagent))
        .route(
            "/v1/k8s/subagent/{id}",
            axum::routing::delete(k8s_delete_subagent),
        )
        // Plugin registry API
        .route("/v1/plugins", get(list_plugins))
        // Tool registry API
        .route("/v1/tools", get(list_tools))
        .route("/v1/tools/register", post(register_tool))
        .route("/v1/tools/{id}/heartbeat", post(tool_heartbeat))
        // MCP compatibility alias (marketing site SDK expects /mcp/v1/tools)
        .route("/mcp/v1/tools", get(list_tools))
        // Agent task APIs (compatibility surface for dashboard)
        .route(
            "/v1/agent/tasks",
            get(list_agent_tasks).post(create_agent_task),
        )
        .route("/v1/agent/tasks/{task_id}", get(get_agent_task))
        .route(
            "/v1/agent/tasks/{task_id}/output",
            get(get_agent_task_output).post(agent_task_output),
        )
        .route(
            "/v1/agent/tasks/{task_id}/output/stream",
            get(stream_agent_task_output),
        )
        // Worker lifecycle and Rust-native mux realtime
        .merge(worker_modules::router())
        // Task dispatch (Knative-backed)
        .route("/v1/tasks/dispatch", post(dispatch_task))
        // Voice REST bridge (dashboard expects REST, server has gRPC)
        .route("/v1/voice/sessions", post(create_voice_session_rest))
        .route(
            "/v1/voice/sessions/{room_name}",
            get(get_voice_session_rest).delete(delete_voice_session_rest),
        )
        .route("/v1/voice/voices", get(list_voices_rest))
        // Session resume (dashboard uses this for codebase-scoped resume)
        .route(
            "/v1/agent/codebases/{codebase_id}/sessions/{session_id}/resume",
            post(session_recovery::handle),
        )
        // Agent Bus — SSE stream + publish
        .route("/v1/bus/stream", get(stream_bus_events))
        .with_state(state.clone())
        // A2A routes at both the root and /a2a so local runtimes can discover
        // and invoke the same server through either shape.
        .merge(a2a_router)
        .nest("/a2a", a2a_nested_router)
        // Mandatory auth middleware — applies to all routes
        .layer(middleware::from_fn_with_state(
            state.clone(),
            audit_middleware,
        ))
        .layer(middleware::from_fn(policy_middleware))
        .layer(middleware::from_fn(auth::require_auth))
        .layer(axum::Extension(state.auth.clone()))
        // CORS + tracing — explicit origin allowlist so headers are present on
        // ALL responses (including 4xx/5xx) and not dependent on request mirroring.
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "https://codetether.run".parse::<HeaderValue>().unwrap(),
                    "https://api.codetether.run".parse::<HeaderValue>().unwrap(),
                    "https://docs.codetether.run"
                        .parse::<HeaderValue>()
                        .unwrap(),
                    "https://codetether.com".parse::<HeaderValue>().unwrap(),
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:3001".parse::<HeaderValue>().unwrap(),
                    "http://localhost:8000".parse::<HeaderValue>().unwrap(),
                ])
                .allow_credentials(true)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([
                    http::header::AUTHORIZATION,
                    http::header::CONTENT_TYPE,
                    http::header::ACCEPT,
                    http::header::ORIGIN,
                    http::header::CACHE_CONTROL,
                    http::header::HeaderName::from_static("x-worker-id"),
                    http::header::HeaderName::from_static("x-agent-name"),
                    http::header::HeaderName::from_static("x-workspaces"),
                    http::header::HeaderName::from_static("x-requested-with"),
                ]),
        )
        .layer(TraceLayer::new_for_http());

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

/// Health check response
async fn health() -> &'static str {
    "ok"
}

/// List all Knative tasks in queue
async fn list_knative_tasks(State(state): State<AppState>) -> Json<Vec<KnativeTask>> {
    codetether_task_api::list_knative_tasks(&state.knative_tasks).await
}

/// Get a specific Knative task by ID
async fn get_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    codetether_task_api::get_knative_task(&state.knative_tasks, task_id).await
}

/// Claim/start processing a Knative task
async fn claim_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    codetether_task_api::claim_knative_task(&state.knative_tasks, task_id).await
}

/// Complete a Knative task
async fn complete_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    codetether_task_api::complete_knative_task(&state.knative_tasks, task_id).await
}
async fn receive_task_event(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<CloudEventResponse>, (StatusCode, String)> {
    codetether_task_ingest::receive_task_event(
        &state.knative_tasks,
        &state.audit_log,
        headers,
        body,
    )
    .await
}

/// List providers
async fn list_providers() -> Json<Vec<String>> {
    Json(vec![
        "openai".to_string(),
        "anthropic".to_string(),
        "google".to_string(),
    ])
}

/// List agents
async fn list_agents() -> Json<Vec<crate::agent::AgentInfo>> {
    let registry = crate::agent::AgentRegistry::with_builtins();
    Json(registry.list().into_iter().cloned().collect())
}

async fn openai_chat_completions(
    Json(req): Json<OpenAiChatCompletionRequest>,
) -> Result<Response, OpenAiApiError> {
    let is_stream = req.stream.unwrap_or(false);
    if req.model.trim().is_empty() {
        return Err(openai_bad_request("`model` is required"));
    }
    if req.messages.is_empty() {
        return Err(openai_bad_request("`messages` must not be empty"));
    }

    let registry = crate::provider::ProviderRegistry::shared_from_vault()
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "Failed to load providers from Vault");
            openai_internal_error(format!("failed to load providers: {error}"))
        })?;

    let providers = registry.list();
    let (selected_provider, model_id) = codetether_openai_adapter::select_request_model(
        &req.model,
        &providers,
        req.service_tier.as_deref(),
        req.reasoning_effort.as_deref(),
    )?;
    let provider = registry.get(&selected_provider).ok_or_else(|| {
        openai_internal_error(format!(
            "Provider `{selected_provider}` was available but could not be loaded"
        ))
    })?;

    let completion_request = codetether_openai_adapter::completion_request(req, &model_id)?;

    codetether_openai_serving::serve(
        provider,
        completion_request,
        is_stream,
        selected_provider,
        model_id,
    )
    .await
}

async fn start_cognition(
    State(state): State<AppState>,
    payload: Option<Json<StartCognitionRequest>>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    state
        .cognition
        .start(payload.map(|Json(body)| body))
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn stop_cognition(
    State(state): State<AppState>,
    payload: Option<Json<StopCognitionRequest>>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    let reason = payload.and_then(|Json(body)| body.reason);
    state
        .cognition
        .stop(reason)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn get_cognition_status(
    State(state): State<AppState>,
) -> Result<Json<CognitionStatus>, (StatusCode, String)> {
    Ok(Json(state.cognition.status().await))
}

async fn stream_cognition(
    State(state): State<AppState>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    codetether_cognition_api::stream_cognition(state.cognition.subscribe_events())
}

/// Stream bus events as SSE, filtered by JWT topic claims.
///
/// The JWT must contain a `topics` claim with an array of topic patterns
/// to subscribe to. Events are filtered to only include envelopes whose
/// topic matches one of the allowed patterns.
async fn stream_bus_events(
    State(state): State<AppState>,
    req: Request<Body>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    // Extract JWT claims from request extensions (set by auth middleware)
    let allowed_topics: Vec<String> = req
        .extensions()
        .get::<crate::server::auth::JwtClaims>()
        .map(|claims| claims.topics.clone())
        .unwrap_or_default();

    // Subscribe to the bus
    let bus_handle = state.bus.handle("stream_bus_events");
    let rx = bus_handle.into_receiver();

    let event_stream = codetether_bus_sse::bus_events(rx, allowed_topics);

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
}

async fn get_latest_snapshot(
    State(state): State<AppState>,
) -> Result<Json<MemorySnapshot>, (StatusCode, String)> {
    codetether_cognition_api::latest_snapshot(state.cognition.latest_snapshot().await)
}

async fn create_persona(
    State(state): State<AppState>,
    Json(req): Json<CreatePersonaRequest>,
) -> Result<Json<crate::cognition::PersonaRuntimeState>, (StatusCode, String)> {
    state
        .cognition
        .create_persona(req)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn spawn_persona(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<SpawnPersonaRequest>,
) -> Result<Json<crate::cognition::PersonaRuntimeState>, (StatusCode, String)> {
    state
        .cognition
        .spawn_child(&id, req)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn reap_persona(
    State(state): State<AppState>,
    Path(id): Path<String>,
    payload: Option<Json<ReapPersonaRequest>>,
) -> Result<Json<ReapPersonaResponse>, (StatusCode, String)> {
    let req = payload
        .map(|Json(body)| body)
        .unwrap_or(ReapPersonaRequest {
            cascade: Some(false),
            reason: None,
        });

    state
        .cognition
        .reap_persona(&id, req)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn get_swarm_lineage(
    State(state): State<AppState>,
) -> Result<Json<LineageGraph>, (StatusCode, String)> {
    Ok(Json(state.cognition.lineage_graph().await))
}

async fn list_beliefs(
    State(state): State<AppState>,
    Query(filter): Query<BeliefFilter>,
) -> Result<Json<Vec<Belief>>, (StatusCode, String)> {
    codetether_cognition_api::list_beliefs(state.cognition.get_beliefs().await, filter)
}

async fn get_belief(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Belief>, (StatusCode, String)> {
    codetether_cognition_api::belief(state.cognition.get_belief(&id).await, &id)
}

async fn list_attention(
    State(state): State<AppState>,
) -> Result<Json<Vec<AttentionItem>>, (StatusCode, String)> {
    Ok(Json(state.cognition.get_attention_queue().await))
}

async fn list_proposals(
    State(state): State<AppState>,
) -> Result<Json<Vec<Proposal>>, (StatusCode, String)> {
    codetether_cognition_api::list_proposals(state.cognition.get_proposals().await)
}

async fn approve_proposal(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    state
        .cognition
        .approve_proposal(&id)
        .await
        .map(|_| Json(serde_json::json!({ "approved": true, "proposal_id": id })))
        .map_err(internal_error)
}

async fn list_receipts(
    State(state): State<AppState>,
) -> Result<Json<Vec<DecisionReceipt>>, (StatusCode, String)> {
    Ok(Json(state.cognition.get_receipts().await))
}

async fn get_workspace(
    State(state): State<AppState>,
) -> Result<Json<GlobalWorkspace>, (StatusCode, String)> {
    Ok(Json(state.cognition.get_workspace().await))
}

async fn get_governance(
    State(state): State<AppState>,
) -> Result<Json<crate::cognition::SwarmGovernance>, (StatusCode, String)> {
    Ok(Json(state.cognition.get_governance().await))
}

async fn get_persona(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::cognition::PersonaRuntimeState>, (StatusCode, String)> {
    codetether_cognition_api::persona(state.cognition.get_persona(&id).await, &id)
}
async fn list_audit_entries(
    State(state): State<AppState>,
    Query(query): Query<AuditQuery>,
) -> Result<Json<Vec<audit::AuditEntry>>, (StatusCode, String)> {
    codetether_audit_http::list_audit_entries(&state.audit_log, query).await
}

// ── K8s self-deployment endpoints ──

async fn get_k8s_status(
    State(state): State<AppState>,
) -> Result<Json<crate::k8s::K8sStatus>, (StatusCode, String)> {
    codetether_k8s_api::get_k8s_status(&state.k8s).await
}

async fn k8s_scale(
    State(state): State<AppState>,
    Json(req): Json<ScaleRequest>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    codetether_k8s_api::k8s_scale(&state.k8s, &state.audit_log, req, internal_error).await
}

async fn k8s_restart(
    State(state): State<AppState>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    codetether_k8s_api::k8s_restart(&state.k8s, &state.audit_log, internal_error).await
}

async fn k8s_list_pods(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::k8s::PodInfo>>, (StatusCode, String)> {
    codetether_k8s_api::k8s_list_pods(&state.k8s, internal_error).await
}

async fn k8s_actions(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::k8s::DeployAction>>, (StatusCode, String)> {
    codetether_k8s_api::k8s_actions(&state.k8s).await
}

async fn k8s_spawn_subagent(
    State(state): State<AppState>,
    Json(req): Json<SpawnSubagentRequest>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    codetether_k8s_api::k8s_spawn_subagent(&state.k8s, req, internal_error).await
}

async fn k8s_delete_subagent(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    codetether_k8s_api::k8s_delete_subagent(&state.k8s, id, internal_error).await
}
async fn list_tools(State(state): State<AppState>) -> Json<Vec<RegisteredToolView>> {
    codetether_tool_discovery::list_tools(&state.tool_registry).await
}
async fn register_tool(
    State(state): State<AppState>,
    Json(req): Json<RegisterToolRequest>,
) -> Result<Json<RegisterToolResponse>, (StatusCode, String)> {
    codetether_tool_discovery::register_tool(&state.tool_registry, req).await
}
async fn tool_heartbeat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<RegisteredToolView>, (StatusCode, String)> {
    codetether_tool_discovery::tool_heartbeat(&state.tool_registry, id).await
}
async fn list_plugins(
    State(state): State<AppState>,
) -> Json<codetether_plugin_http::PluginListResponse> {
    codetether_plugin_http::list_plugins(&state.plugin_registry, &state.server_fingerprint).await
}

// ── Agent task compatibility surface ─────────────────────────────────────
// These handlers provide the /v1/agent/tasks/* surface the marketing-site
// dashboard expects, backed by the existing KnativeTaskQueue.

/// List all agent tasks (wraps Knative task queue)
async fn list_agent_tasks(
    State(state): State<AppState>,
    Query(params): Query<ListAgentTasksQuery>,
) -> Json<serde_json::Value> {
    codetether_task_api::list_agent_tasks(&state.knative_tasks, params).await
}

/// Create a new agent task
async fn create_agent_task(
    State(state): State<AppState>,
    Json(req): Json<CreateAgentTaskRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    codetether_task_api::create_agent_task(&state.knative_tasks, req).await
}

/// Get a single agent task by ID
async fn get_agent_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    codetether_task_api::get_agent_task(&state.knative_tasks, task_id).await
}

/// Stream task output as SSE events
async fn stream_agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Sse<impl stream::Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    task_output_stream::handler(State(state), Path(task_id)).await
}
async fn list_connected_workers(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    codetether_k8s_api::list_connected_workers(&state.k8s).await
}
async fn create_voice_session_rest(
    State(state): State<AppState>,
    Json(req): Json<CreateVoiceSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    codetether_voice_api::create_voice_session_rest(&state.bus, req).await
}
async fn delete_voice_session_rest(
    State(state): State<AppState>,
    Path(room_name): Path<String>,
) -> Json<serde_json::Value> {
    codetether_voice_api::delete_voice_session_rest(&state.bus, Path(room_name)).await
}

// ── Session resume ──────────────────────────────────────────────────────

/// Resume a durable session without replacing its identity on a storage failure.

fn internal_error(error: anyhow::Error) -> (StatusCode, String) {
    let message = error.to_string();
    if message.contains("not found") {
        return (StatusCode::NOT_FOUND, message);
    }
    if message.contains("disabled") || message.contains("exceeds") || message.contains("limit") {
        return (StatusCode::BAD_REQUEST, message);
    }
    (StatusCode::INTERNAL_SERVER_ERROR, message)
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
