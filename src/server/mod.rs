//! HTTP Server
//!
//! Main API server for the CodeTether Agent

pub mod auth;
pub mod policy;

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
use crate::tool::{PluginManifest, SigningKey, hash_bytes, hash_file};
use anyhow::Result;
use auth::AuthState;
use axum::{
    Router,
    body::Body,
    extract::Path,
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::sse::{Event, KeepAlive, Sse},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use futures::{StreamExt, future::join_all, stream};
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

/// Task received from Knative Eventing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnativeTask {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub agent_type: String,
    pub priority: i32,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

/// Queue for Knative tasks waiting to be processed
#[derive(Clone)]
pub struct KnativeTaskQueue {
    tasks: Arc<Mutex<Vec<KnativeTask>>>,
}

impl KnativeTaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn push(&self, task: KnativeTask) {
        self.tasks.lock().await.push(task);
    }

    #[allow(dead_code)]
    pub async fn pop(&self) -> Option<KnativeTask> {
        self.tasks.lock().await.pop()
    }

    pub async fn list(&self) -> Vec<KnativeTask> {
        self.tasks.lock().await.clone()
    }

    #[allow(dead_code)]
    pub async fn get(&self, task_id: &str) -> Option<KnativeTask> {
        self.tasks
            .lock()
            .await
            .iter()
            .find(|t| t.task_id == task_id)
            .cloned()
    }

    pub async fn update_status(&self, task_id: &str, status: &str) -> bool {
        let mut tasks = self.tasks.lock().await;
        if let Some(task) = tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.status = status.to_string();
            true
        } else {
            false
        }
    }
}

impl Default for KnativeTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// A registered tool with TTL tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisteredTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub parameters: serde_json::Value,
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Tool registry with TTL-based expiry
#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<tokio::sync::RwLock<HashMap<String, RegisteredTool>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    /// Register a new tool
    pub async fn register(&self, tool: RegisteredTool) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.id.clone(), tool);
    }

    /// Get a tool by ID
    #[allow(dead_code)]
    pub async fn get(&self, id: &str) -> Option<RegisteredTool> {
        let tools = self.tools.read().await;
        tools.get(id).cloned()
    }

    /// List all tools (excluding expired)
    pub async fn list(&self) -> Vec<RegisteredTool> {
        let tools = self.tools.read().await;
        let now = chrono::Utc::now();
        tools
            .values()
            .filter(|t| t.expires_at > now)
            .cloned()
            .collect()
    }

    /// Update heartbeat for a tool (extends TTL by 30s)
    pub async fn heartbeat(&self, id: &str) -> Option<RegisteredTool> {
        let mut tools = self.tools.write().await;
        if let Some(tool) = tools.get_mut(id) {
            let now = chrono::Utc::now();
            tool.last_heartbeat = now;
            tool.expires_at = now + Duration::from_secs(90);
            return Some(tool.clone());
        }
        None
    }

    /// Clean up expired tools
    pub async fn cleanup(&self) {
        let mut tools = self.tools.write().await;
        let now = chrono::Utc::now();
        tools.retain(|_, t| t.expires_at > now);
    }
}

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
}

/// Audit middleware — logs every request/response to the audit trail.
async fn audit_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let started = std::time::Instant::now();

    let response = next.run(request).await;

    let duration_ms = started.elapsed().as_millis() as u64;
    let status = response.status().as_u16();
    let outcome = if status < 400 {
        AuditOutcome::Success
    } else if status == 401 || status == 403 {
        AuditOutcome::Denied
    } else {
        AuditOutcome::Failure
    };

    state
        .audit_log
        .record(audit::AuditEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            category: AuditCategory::Api,
            action: format!("{} {}", method, path),
            principal: None,
            outcome,
            detail: Some(serde_json::json!({ "status": status })),
            duration_ms: Some(duration_ms),
            okr_id: None,
            okr_run_id: None,
            relay_id: None,
            session_id: None,
        })
        .await;

    response
}

/// Mapping from (path pattern, HTTP method) → OPA permission action.
/// The first matching rule wins.
struct PolicyRule {
    pattern: &'static str,
    methods: Option<&'static [&'static str]>,
    permission: &'static str,
}

const POLICY_RULES: &[PolicyRule] = &[
    // Public / exempt
    PolicyRule {
        pattern: "/health",
        methods: None,
        permission: "",
    },
    PolicyRule {
        pattern: "/task",
        methods: None,
        permission: "",
    },
    PolicyRule {
        pattern: "/v1/knative/",
        methods: None,
        permission: "",
    },
    // OpenAI-compatible model discovery and chat completions
    PolicyRule {
        pattern: "/v1/models",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/chat/completions",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    // Tool registry — read/write
    PolicyRule {
        pattern: "/v1/tools",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/tools/register",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/tools/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/a2a/",
        methods: None,
        permission: "",
    },
    // K8s management — admin only
    PolicyRule {
        pattern: "/v1/k8s/scale",
        methods: Some(&["POST"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/k8s/restart",
        methods: Some(&["POST"]),
        permission: "admin:access",
    },
    PolicyRule {
        pattern: "/v1/k8s/",
        methods: Some(&["GET"]),
        permission: "admin:access",
    },
    // K8s sub-agent lifecycle — admin only
    PolicyRule {
        pattern: "/v1/k8s/subagent",
        methods: Some(&["POST", "DELETE"]),
        permission: "admin:access",
    },
    // Plugin registry — read
    PolicyRule {
        pattern: "/v1/plugins",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Audit — admin
    PolicyRule {
        pattern: "/v1/audit",
        methods: None,
        permission: "admin:access",
    },
    // Event stream replay — admin (compliance feature)
    PolicyRule {
        pattern: "/v1/audit/replay",
        methods: Some(&["GET"]),
        permission: "admin:access",
    },
    // Cognition — write operations
    PolicyRule {
        pattern: "/v1/cognition/start",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/cognition/stop",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/cognition/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Swarm persona lifecycle
    PolicyRule {
        pattern: "/v1/swarm/personas",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/swarm/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/swarm/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Agent Bus — read access for stream, filtered by JWT topic claims
    PolicyRule {
        pattern: "/v1/bus/stream",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Agent Bus — publish messages (requires write permission)
    PolicyRule {
        pattern: "/v1/bus/publish",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    // Session management
    // Session prompt execution — requires execute permission
    PolicyRule {
        pattern: "/api/session/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/api/session",
        methods: Some(&["POST"]),
        permission: "sessions:write",
    },
    PolicyRule {
        pattern: "/api/session",
        methods: Some(&["GET"]),
        permission: "sessions:read",
    },
    // MCP compatibility alias
    PolicyRule {
        pattern: "/mcp/v1/tools",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Agent task APIs (dashboard compatibility)
    PolicyRule {
        pattern: "/v1/agent/tasks",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/agent/tasks",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    PolicyRule {
        pattern: "/v1/agent/tasks/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Worker connectivity
    PolicyRule {
        pattern: "/v1/worker/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/agent/workers",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    // Task dispatch
    PolicyRule {
        pattern: "/v1/tasks/dispatch",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    // Voice REST bridge
    PolicyRule {
        pattern: "/v1/voice/",
        methods: Some(&["GET"]),
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/v1/voice/",
        methods: Some(&["POST", "DELETE"]),
        permission: "agent:execute",
    },
    // Session resume
    PolicyRule {
        pattern: "/v1/agent/codebases/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    // Proposal approval — governance action
    PolicyRule {
        pattern: "/v1/cognition/proposals/",
        methods: Some(&["POST"]),
        permission: "agent:execute",
    },
    // Config, version, providers, agents — read
    PolicyRule {
        pattern: "/api/version",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/config",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/provider",
        methods: None,
        permission: "agent:read",
    },
    PolicyRule {
        pattern: "/api/agent",
        methods: None,
        permission: "agent:read",
    },
];

/// Find the required permission for a given path + method.
/// Returns `Some("")` for exempt, `Some(perm)` for required, `None` for unmatched (pass-through).
fn match_policy_rule(path: &str, method: &str) -> Option<&'static str> {
    for rule in POLICY_RULES {
        let matches = if rule.pattern.ends_with('/') {
            path.starts_with(rule.pattern)
        } else {
            path == rule.pattern || path.starts_with(&format!("{}/", rule.pattern))
        };
        if matches {
            if let Some(allowed_methods) = rule.methods
                && !allowed_methods.contains(&method)
            {
                continue;
            }
            return Some(rule.permission);
        }
    }
    None
}

/// Policy authorization middleware for Axum.
///
/// Maps request paths to OPA permission strings and enforces authorization.
/// Runs after `require_auth` so the bearer token is already validated.
/// Currently maps the static bearer token to an admin role since
/// codetether-agent uses a single shared token model.
async fn policy_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    let method = request.method().as_str().to_string();

    let permission = match match_policy_rule(&path, &method) {
        None | Some("") => return Ok(next.run(request).await),
        Some(perm) => perm,
    };

    // The current auth model uses a single static token for all access.
    // When this is the case, the authenticated user effectively has admin role.
    // Future: extract user claims from JWT and build a proper PolicyUser.
    let user = policy::PolicyUser {
        user_id: "bearer-token-user".to_string(),
        roles: vec!["admin".to_string()],
        tenant_id: None,
        scopes: vec![],
        auth_source: "static_token".to_string(),
    };

    if let Err(status) = policy::enforce_policy(&user, permission, None).await {
        tracing::warn!(
            path = %path,
            method = %method,
            permission = %permission,
            "Policy middleware denied request"
        );
        return Err(status);
    }

    Ok(next.run(request).await)
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
    cognition.set_tools(Arc::new(crate::tool::ToolRegistry::with_defaults()));
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
        "Auth is mandatory. Token required for all API endpoints."
    );
    tracing::info!(
        audit_entries = audit_log.count().await,
        "Audit log initialized"
    );

    // Create agent bus for in-process communication
    let bus = AgentBus::new().into_arc();

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
    let a2a_router = a2a_server.router();

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
        tracing::info!("gRPC A2A server listening on {}", grpc_addr);

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

        if let Err(e) = tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(cors)
            .layer(GrpcWebLayer::new())
            .add_service(grpc_service)
            .add_service(voice_service)
            .serve(grpc_addr)
            .await
        {
            tracing::error!("gRPC server error: {}", e);
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

    let app = Router::new()
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
        .route("/api/version", get(get_version))
        .route("/api/session", get(list_sessions).post(create_session))
        .route("/api/session/{id}", get(get_session))
        .route("/api/session/{id}/prompt", post(prompt_session))
        .route("/api/config", get(get_config))
        .route("/api/provider", get(list_providers))
        .route("/api/agent", get(list_agents))
        // OpenAI-compatible APIs
        .route("/v1/models", get(list_openai_models))
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
        // Worker connectivity (dashboard polls this)
        .route("/v1/worker/connected", get(list_connected_workers))
        .route("/v1/agent/workers", get(list_connected_workers))
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
            post(resume_codebase_session),
        )
        // Agent Bus — SSE stream + publish
        .route("/v1/bus/stream", get(stream_bus_events))
        .with_state(state.clone())
        // A2A routes (nested to work with different state type)
        .nest("/a2a", a2a_router)
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
                .allow_methods(AllowMethods::any())
                .allow_headers(AllowHeaders::any()),
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
    Json(state.knative_tasks.list().await)
}

/// Get a specific Knative task by ID
async fn get_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    state
        .knative_tasks
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}

/// Claim/start processing a Knative task
async fn claim_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    // Update status to processing
    let updated = state
        .knative_tasks
        .update_status(&task_id, "processing")
        .await;
    if !updated {
        return Err((StatusCode::NOT_FOUND, format!("Task {} not found", task_id)));
    }

    state
        .knative_tasks
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}

/// Complete a Knative task
async fn complete_knative_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    // Update status to completed
    let updated = state
        .knative_tasks
        .update_status(&task_id, "completed")
        .await;
    if !updated {
        return Err((StatusCode::NOT_FOUND, format!("Task {} not found", task_id)));
    }

    state
        .knative_tasks
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}

/// CloudEvent handler for Knative Eventing
/// Receives task events from Knative Broker and triggers execution
async fn receive_task_event(
    State(state): State<AppState>,
    Json(event): Json<CloudEvent>,
) -> Result<Json<CloudEventResponse>, (StatusCode, String)> {
    tracing::info!(
        event_type = %event.event_type,
        event_id = %event.id,
        "Received CloudEvent from Knative"
    );

    // Log the incoming event for audit
    state
        .audit_log
        .log(
            audit::AuditCategory::Api,
            format!("cloudevents:{}", event.event_type),
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

    // Process based on event type
    match event.event_type.as_str() {
        "codetether.task.created" | "task.created" => {
            // Extract task data and queue for execution
            if let Some(data) = event.data {
                let task_id = data
                    .get("task_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let title = data
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let description = data
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let agent_type = data
                    .get("agent_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("build")
                    .to_string();
                let priority = data.get("priority").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

                let task = KnativeTask {
                    task_id: task_id.clone(),
                    title,
                    description,
                    agent_type,
                    priority,
                    received_at: chrono::Utc::now(),
                    status: "queued".to_string(),
                };

                state.knative_tasks.push(task).await;
                tracing::info!(task_id = %task_id, "Task queued for execution");
            }
        }
        "codetether.task.cancelled" => {
            tracing::info!("Task cancellation event received");
            // Update task status if we have the task_id
            if let Some(data) = event.data
                && let Some(task_id) = data.get("task_id").and_then(|v| v.as_str())
            {
                let _ = state
                    .knative_tasks
                    .update_status(task_id, "cancelled")
                    .await;
                tracing::info!(task_id = %task_id, "Task cancelled");
            }
        }
        _ => {
            tracing::warn!(event_type = %event.event_type, "Unknown CloudEvent type");
        }
    }

    Ok(Json(CloudEventResponse {
        status: "accepted".to_string(),
        event_id: event.id,
    }))
}

/// CloudEvent structure (CloudEvents v1.0 spec)
#[derive(Deserialize, Serialize)]
struct CloudEvent {
    /// Event unique identifier
    id: String,
    /// Event source (e.g., knative://broker/a2a-server)
    source: String,
    /// Event type (e.g., codetether.task.created)
    #[serde(rename = "type")]
    event_type: String,
    /// Event timestamp (RFC 3339)
    #[serde(rename = "time")]
    timestamp: Option<String>,
    /// CloudEvents spec version
    #[serde(rename = "specversion")]
    spec_version: Option<String>,
    /// Event data payload
    data: Option<serde_json::Value>,
}

/// Response to CloudEvent acknowledgment
#[derive(Serialize)]
struct CloudEventResponse {
    status: String,
    event_id: String,
}

/// Version info
#[derive(Serialize)]
struct VersionInfo {
    version: &'static str,
    name: &'static str,
    binary_hash: Option<String>,
}

async fn get_version() -> Json<VersionInfo> {
    let binary_hash = std::env::current_exe()
        .ok()
        .and_then(|p| hash_file(&p).ok());
    Json(VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        name: env!("CARGO_PKG_NAME"),
        binary_hash,
    })
}

/// List sessions
#[derive(Deserialize)]
struct ListSessionsQuery {
    limit: Option<usize>,
    offset: Option<usize>,
}

async fn list_sessions(
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<Vec<crate::session::SessionSummary>>, (StatusCode, String)> {
    let sessions = crate::session::list_sessions()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(100);
    Ok(Json(
        sessions.into_iter().skip(offset).take(limit).collect(),
    ))
}

/// Create a new session
#[derive(Deserialize)]
struct CreateSessionRequest {
    title: Option<String>,
    agent: Option<String>,
}

async fn create_session(
    Json(req): Json<CreateSessionRequest>,
) -> Result<Json<crate::session::Session>, (StatusCode, String)> {
    let mut session = crate::session::Session::new()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    session.title = req.title;
    if let Some(agent) = req.agent {
        session.agent = agent;
    }

    session
        .save()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(session))
}

/// Get a session by ID
async fn get_session(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::session::Session>, (StatusCode, String)> {
    let session = crate::session::Session::load(&id)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(session))
}

/// Prompt a session
#[derive(Deserialize)]
struct PromptRequest {
    message: String,
}

async fn prompt_session(
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<PromptRequest>,
) -> Result<Json<crate::session::SessionResult>, (StatusCode, String)> {
    // Validate the message is not empty
    if req.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Message cannot be empty".to_string(),
        ));
    }

    // Log the prompt request (uses the message field)
    tracing::info!(
        session_id = %id,
        message_len = req.message.len(),
        "Received prompt request"
    );

    // TODO: Implement actual prompting
    Err((
        StatusCode::NOT_IMPLEMENTED,
        "Prompt execution not yet implemented".to_string(),
    ))
}

/// Get configuration
async fn get_config(State(state): State<AppState>) -> Json<Config> {
    Json((*state.config).clone())
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

type OpenAiApiError = (StatusCode, Json<serde_json::Value>);
type OpenAiApiResult<T> = Result<T, OpenAiApiError>;

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionRequest {
    model: String,
    messages: Vec<OpenAiRequestMessage>,
    #[serde(default)]
    tools: Vec<OpenAiRequestTool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    max_tokens: Option<usize>,
    max_completion_tokens: Option<usize>,
    stop: Option<OpenAiStop>,
    stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OpenAiStop {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct OpenAiRequestMessage {
    role: String,
    #[serde(default)]
    content: Option<serde_json::Value>,
    #[serde(default)]
    tool_call_id: Option<String>,
    #[serde(default)]
    tool_calls: Vec<OpenAiRequestToolCall>,
}

#[derive(Debug, Deserialize)]
struct OpenAiRequestToolCall {
    id: String,
    #[serde(rename = "type", default = "default_function_type")]
    kind: String,
    function: OpenAiRequestToolCallFunction,
}

#[derive(Debug, Deserialize)]
struct OpenAiRequestToolCallFunction {
    name: String,
    #[serde(default = "default_json_object_string")]
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiRequestTool {
    #[serde(rename = "type", default = "default_function_type")]
    kind: String,
    function: OpenAiRequestToolDefinition,
}

#[derive(Debug, Deserialize)]
struct OpenAiRequestToolDefinition {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default = "default_json_object")]
    parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct OpenAiModelsResponse {
    object: String,
    data: Vec<OpenAiModel>,
}

#[derive(Debug, Serialize)]
struct OpenAiModel {
    id: String,
    object: String,
    created: i64,
    owned_by: String,
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAiChatCompletionChoice>,
    usage: OpenAiUsage,
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionChoice {
    index: usize,
    message: OpenAiChatCompletionMessage,
    finish_reason: String,
}

#[derive(Debug, Serialize)]
struct OpenAiChatCompletionMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAiResponseToolCall>>,
}

#[derive(Debug, Serialize)]
struct OpenAiResponseToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    function: OpenAiResponseToolCallFunction,
}

#[derive(Debug, Serialize)]
struct OpenAiResponseToolCallFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize)]
struct OpenAiUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

fn default_function_type() -> String {
    "function".to_string()
}

fn default_json_object() -> serde_json::Value {
    serde_json::json!({})
}

fn default_json_object_string() -> String {
    "{}".to_string()
}

fn openai_error(
    status: StatusCode,
    message: impl Into<String>,
    error_type: &str,
    code: &str,
) -> OpenAiApiError {
    (
        status,
        Json(serde_json::json!({
            "error": {
                "message": message.into(),
                "type": error_type,
                "code": code,
            }
        })),
    )
}

fn openai_bad_request(message: impl Into<String>) -> OpenAiApiError {
    openai_error(
        StatusCode::BAD_REQUEST,
        message,
        "invalid_request_error",
        "invalid_request",
    )
}

fn openai_internal_error(message: impl Into<String>) -> OpenAiApiError {
    openai_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        message,
        "server_error",
        "internal_error",
    )
}

fn canonicalize_provider_name(provider: &str) -> &str {
    if provider == "zhipuai" {
        "zai"
    } else {
        provider
    }
}

fn normalize_model_reference(model: &str) -> String {
    let trimmed = model.trim();
    if let Some((provider, model_id)) = trimmed.split_once(':')
        && !provider.is_empty()
        && !model_id.is_empty()
        && !trimmed.contains('/')
    {
        return format!("{provider}/{model_id}");
    }
    trimmed.to_string()
}

fn make_openai_model_id(provider: &str, model_id: &str) -> String {
    let provider = canonicalize_provider_name(provider);
    let trimmed = model_id.trim_start_matches('/');
    if trimmed.starts_with(&format!("{provider}/")) {
        trimmed.to_string()
    } else {
        format!("{provider}/{trimmed}")
    }
}

fn parse_openai_content_part(value: &serde_json::Value) -> Option<crate::provider::ContentPart> {
    let obj = value.as_object()?;
    let part_type = obj
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("text");

    match part_type {
        "text" | "input_text" => obj
            .get("text")
            .and_then(serde_json::Value::as_str)
            .map(|text| crate::provider::ContentPart::Text {
                text: text.to_string(),
            }),
        "image_url" => obj
            .get("image_url")
            .and_then(serde_json::Value::as_object)
            .and_then(|image| image.get("url"))
            .and_then(serde_json::Value::as_str)
            .map(|url| crate::provider::ContentPart::Image {
                url: url.to_string(),
                mime_type: None,
            }),
        _ => obj
            .get("text")
            .and_then(serde_json::Value::as_str)
            .map(|text| crate::provider::ContentPart::Text {
                text: text.to_string(),
            }),
    }
}

fn parse_openai_content_parts(
    content: &Option<serde_json::Value>,
) -> Vec<crate::provider::ContentPart> {
    let Some(value) = content else {
        return Vec::new();
    };

    match value {
        serde_json::Value::String(text) => {
            vec![crate::provider::ContentPart::Text { text: text.clone() }]
        }
        serde_json::Value::Array(parts) => {
            parts.iter().filter_map(parse_openai_content_part).collect()
        }
        serde_json::Value::Object(_) => parse_openai_content_part(value).into_iter().collect(),
        _ => Vec::new(),
    }
}

fn parse_openai_tool_content(content: &Option<serde_json::Value>) -> String {
    parse_openai_content_parts(content)
        .into_iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text } => Some(text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn convert_openai_messages(
    messages: &[OpenAiRequestMessage],
) -> OpenAiApiResult<Vec<crate::provider::Message>> {
    let mut converted = Vec::with_capacity(messages.len());

    for (index, message) in messages.iter().enumerate() {
        let role = message.role.to_ascii_lowercase();
        match role.as_str() {
            "system" | "user" => {
                let content = parse_openai_content_parts(&message.content);
                if content.is_empty() {
                    return Err(openai_bad_request(format!(
                        "messages[{index}] must include text content"
                    )));
                }

                converted.push(crate::provider::Message {
                    role: if role == "system" {
                        crate::provider::Role::System
                    } else {
                        crate::provider::Role::User
                    },
                    content,
                });
            }
            "assistant" => {
                let mut content = parse_openai_content_parts(&message.content);
                for tool_call in &message.tool_calls {
                    if tool_call.kind != "function" {
                        return Err(openai_bad_request(format!(
                            "messages[{index}].tool_calls only support `function`"
                        )));
                    }
                    if tool_call.function.name.trim().is_empty() {
                        return Err(openai_bad_request(format!(
                            "messages[{index}].tool_calls[].function.name is required"
                        )));
                    }

                    content.push(crate::provider::ContentPart::ToolCall {
                        id: tool_call.id.clone(),
                        name: tool_call.function.name.clone(),
                        arguments: tool_call.function.arguments.clone(),
                        thought_signature: None,
                    });
                }

                if content.is_empty() {
                    return Err(openai_bad_request(format!(
                        "messages[{index}] must include `content` or `tool_calls` for assistant role"
                    )));
                }

                converted.push(crate::provider::Message {
                    role: crate::provider::Role::Assistant,
                    content,
                });
            }
            "tool" => {
                let tool_call_id = message
                    .tool_call_id
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        openai_bad_request(format!(
                            "messages[{index}].tool_call_id is required for tool role"
                        ))
                    })?
                    .to_string();

                converted.push(crate::provider::Message {
                    role: crate::provider::Role::Tool,
                    content: vec![crate::provider::ContentPart::ToolResult {
                        tool_call_id,
                        content: parse_openai_tool_content(&message.content),
                    }],
                });
            }
            _ => {
                return Err(openai_bad_request(format!(
                    "messages[{index}].role `{}` is not supported",
                    message.role
                )));
            }
        }
    }

    Ok(converted)
}

fn convert_openai_tools(
    tools: &[OpenAiRequestTool],
) -> OpenAiApiResult<Vec<crate::provider::ToolDefinition>> {
    let mut converted = Vec::with_capacity(tools.len());

    for (index, tool) in tools.iter().enumerate() {
        if tool.kind != "function" {
            return Err(openai_bad_request(format!(
                "tools[{index}].type `{}` is not supported",
                tool.kind
            )));
        }
        if tool.function.name.trim().is_empty() {
            return Err(openai_bad_request(format!(
                "tools[{index}].function.name is required"
            )));
        }

        converted.push(crate::provider::ToolDefinition {
            name: tool.function.name.clone(),
            description: tool.function.description.clone(),
            parameters: tool.function.parameters.clone(),
        });
    }

    Ok(converted)
}

fn convert_finish_reason(reason: crate::provider::FinishReason) -> &'static str {
    match reason {
        crate::provider::FinishReason::Stop => "stop",
        crate::provider::FinishReason::Length => "length",
        crate::provider::FinishReason::ToolCalls => "tool_calls",
        crate::provider::FinishReason::ContentFilter => "content_filter",
        crate::provider::FinishReason::Error => "error",
    }
}

fn convert_response_message(
    message: &crate::provider::Message,
) -> (Option<String>, Option<Vec<OpenAiResponseToolCall>>) {
    let mut texts = Vec::new();
    let mut tool_calls = Vec::new();

    for part in &message.content {
        match part {
            crate::provider::ContentPart::Text { text } => {
                if !text.is_empty() {
                    texts.push(text.clone());
                }
            }
            crate::provider::ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                tool_calls.push(OpenAiResponseToolCall {
                    id: id.clone(),
                    kind: "function".to_string(),
                    function: OpenAiResponseToolCallFunction {
                        name: name.clone(),
                        arguments: arguments.clone(),
                    },
                });
            }
            _ => {}
        }
    }

    let content = if texts.is_empty() {
        None
    } else {
        Some(texts.join("\n"))
    };
    let tool_calls = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls)
    };

    (content, tool_calls)
}

fn openai_stream_chunk(
    id: &str,
    created: i64,
    model: &str,
    delta: serde_json::Value,
    finish_reason: Option<&str>,
) -> serde_json::Value {
    let finish_reason = finish_reason
        .map(|value| serde_json::Value::String(value.to_string()))
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": created,
        "model": model,
        "choices": [{
            "index": 0,
            "delta": delta,
            "finish_reason": finish_reason,
        }]
    })
}

fn openai_stream_event(payload: &serde_json::Value) -> Event {
    Event::default().data(payload.to_string())
}

async fn list_openai_models() -> OpenAiApiResult<Json<OpenAiModelsResponse>> {
    let registry = crate::provider::ProviderRegistry::from_vault()
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "Failed to load providers from Vault");
            openai_internal_error(format!("failed to load providers: {error}"))
        })?;

    let model_futures = registry.list().into_iter().map(|provider_id| {
        let provider_id = provider_id.to_string();
        let provider = registry.get(&provider_id);

        async move {
            let Some(provider) = provider else {
                return Vec::new();
            };

            let now = chrono::Utc::now().timestamp();
            match provider.list_models().await {
                Ok(models) => models
                    .into_iter()
                    .map(|model| OpenAiModel {
                        id: make_openai_model_id(&provider_id, &model.id),
                        object: "model".to_string(),
                        created: now,
                        owned_by: canonicalize_provider_name(&provider_id).to_string(),
                    })
                    .collect(),
                Err(error) => {
                    tracing::warn!(
                        provider = %provider_id,
                        error = %error,
                        "Failed to list models for provider"
                    );
                    Vec::new()
                }
            }
        }
    });

    let mut data: Vec<OpenAiModel> = join_all(model_futures)
        .await
        .into_iter()
        .flatten()
        .collect();
    data.sort_by(|a, b| a.owned_by.cmp(&b.owned_by).then(a.id.cmp(&b.id)));

    Ok(Json(OpenAiModelsResponse {
        object: "list".to_string(),
        data,
    }))
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

    let registry = crate::provider::ProviderRegistry::from_vault()
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "Failed to load providers from Vault");
            openai_internal_error(format!("failed to load providers: {error}"))
        })?;

    let providers = registry.list();
    if providers.is_empty() {
        return Err(openai_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "No providers are configured in Vault",
            "server_error",
            "no_providers",
        ));
    }

    let normalized_model = normalize_model_reference(&req.model);
    let (maybe_provider, model_id) = crate::provider::parse_model_string(&normalized_model);
    let model_id = if maybe_provider.is_some() {
        if model_id.trim().is_empty() {
            return Err(openai_bad_request(
                "Model must be in `provider/model` format",
            ));
        }
        model_id.to_string()
    } else {
        req.model.trim().to_string()
    };

    let selected_provider = if let Some(provider_name) = maybe_provider {
        let provider_name = canonicalize_provider_name(provider_name);
        if providers.contains(&provider_name) {
            provider_name.to_string()
        } else {
            return Err(openai_bad_request(format!(
                "Provider `{provider_name}` is not configured in Vault"
            )));
        }
    } else if providers.len() == 1 {
        providers[0].to_string()
    } else if providers.contains(&"openai") {
        "openai".to_string()
    } else {
        return Err(openai_bad_request(
            "When multiple providers are configured, use `provider/model`. See GET /v1/models.",
        ));
    };

    let provider = registry.get(&selected_provider).ok_or_else(|| {
        openai_internal_error(format!(
            "Provider `{selected_provider}` was available but could not be loaded"
        ))
    })?;

    let messages = convert_openai_messages(&req.messages)?;
    let tools = convert_openai_tools(&req.tools)?;
    let stop = match req.stop {
        Some(OpenAiStop::Single(stop)) => vec![stop],
        Some(OpenAiStop::Multiple(stops)) => stops,
        None => Vec::new(),
    };

    let completion_request = crate::provider::CompletionRequest {
        messages,
        tools,
        model: model_id.clone(),
        temperature: req.temperature,
        top_p: req.top_p,
        max_tokens: req.max_completion_tokens.or(req.max_tokens),
        stop,
    };

    let chat_id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let now = chrono::Utc::now().timestamp();
    let openai_model = make_openai_model_id(&selected_provider, &model_id);

    if is_stream {
        let mut provider_stream =
            provider
                .complete_stream(completion_request)
                .await
                .map_err(|error| {
                    tracing::warn!(
                        provider = %selected_provider,
                        model = %model_id,
                        error = %error,
                        "Streaming completion request failed"
                    );
                    openai_internal_error(error.to_string())
                })?;

        let stream_id = chat_id.clone();
        let stream_model = openai_model.clone();
        let event_stream = async_stream::stream! {
            let mut tool_call_indices: HashMap<String, usize> = HashMap::new();
            let mut next_tool_call_index = 0usize;
            let mut saw_text = false;
            let mut saw_tool_calls = false;

            let role_chunk = openai_stream_chunk(
                &stream_id,
                now,
                &stream_model,
                serde_json::json!({ "role": "assistant" }),
                None,
            );
            yield Ok::<Event, Infallible>(openai_stream_event(&role_chunk));

            while let Some(chunk) = provider_stream.next().await {
                match chunk {
                    crate::provider::StreamChunk::Text(text) => {
                        if text.is_empty() {
                            continue;
                        }
                        saw_text = true;
                        let content_chunk = openai_stream_chunk(
                            &stream_id,
                            now,
                            &stream_model,
                            serde_json::json!({ "content": text }),
                            None,
                        );
                        yield Ok(openai_stream_event(&content_chunk));
                    }
                    crate::provider::StreamChunk::ToolCallStart { id, name } => {
                        saw_tool_calls = true;
                        let index = *tool_call_indices.entry(id.clone()).or_insert_with(|| {
                            let value = next_tool_call_index;
                            next_tool_call_index += 1;
                            value
                        });
                        let tool_chunk = openai_stream_chunk(
                            &stream_id,
                            now,
                            &stream_model,
                            serde_json::json!({
                                "tool_calls": [{
                                    "index": index,
                                    "id": id,
                                    "type": "function",
                                    "function": {
                                        "name": name,
                                        "arguments": "",
                                    }
                                }]
                            }),
                            None,
                        );
                        yield Ok(openai_stream_event(&tool_chunk));
                    }
                    crate::provider::StreamChunk::ToolCallDelta { id, arguments_delta } => {
                        if arguments_delta.is_empty() {
                            continue;
                        }
                        saw_tool_calls = true;
                        let index = *tool_call_indices.entry(id.clone()).or_insert_with(|| {
                            let value = next_tool_call_index;
                            next_tool_call_index += 1;
                            value
                        });
                        let tool_delta_chunk = openai_stream_chunk(
                            &stream_id,
                            now,
                            &stream_model,
                            serde_json::json!({
                                "tool_calls": [{
                                    "index": index,
                                    "id": id,
                                    "type": "function",
                                    "function": {
                                        "arguments": arguments_delta,
                                    }
                                }]
                            }),
                            None,
                        );
                        yield Ok(openai_stream_event(&tool_delta_chunk));
                    }
                    crate::provider::StreamChunk::ToolCallEnd { .. } => {}
                    crate::provider::StreamChunk::Done { .. } => {}
                    crate::provider::StreamChunk::Error(error) => {
                        let error_chunk = openai_stream_chunk(
                            &stream_id,
                            now,
                            &stream_model,
                            serde_json::json!({ "content": error }),
                            Some("error"),
                        );
                        yield Ok(openai_stream_event(&error_chunk));
                        yield Ok(Event::default().data("[DONE]"));
                        return;
                    }
                }
            }

            let finish_reason = if saw_tool_calls && !saw_text {
                "tool_calls"
            } else {
                "stop"
            };
            let final_chunk = openai_stream_chunk(
                &stream_id,
                now,
                &stream_model,
                serde_json::json!({}),
                Some(finish_reason),
            );
            yield Ok(openai_stream_event(&final_chunk));
            yield Ok(Event::default().data("[DONE]"));
        };

        return Ok(Sse::new(event_stream)
            .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
            .into_response());
    }

    let completion = provider
        .complete(completion_request)
        .await
        .map_err(|error| {
            tracing::warn!(
                provider = %selected_provider,
                model = %model_id,
                error = %error,
                "Completion request failed"
            );
            openai_internal_error(error.to_string())
        })?;

    let (content, tool_calls) = convert_response_message(&completion.message);

    Ok(Json(OpenAiChatCompletionResponse {
        id: chat_id,
        object: "chat.completion".to_string(),
        created: now,
        model: openai_model,
        choices: vec![OpenAiChatCompletionChoice {
            index: 0,
            message: OpenAiChatCompletionMessage {
                role: "assistant".to_string(),
                content,
                tool_calls,
            },
            finish_reason: convert_finish_reason(completion.finish_reason).to_string(),
        }],
        usage: OpenAiUsage {
            prompt_tokens: completion.usage.prompt_tokens,
            completion_tokens: completion.usage.completion_tokens,
            total_tokens: completion.usage.total_tokens,
        },
    })
    .into_response())
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
    let rx = state.cognition.subscribe_events();

    let event_stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(event) => {
                let payload = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
                let sse_event = Event::default().event("cognition").data(payload);
                Some((Ok(sse_event), rx))
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                let lag_event = Event::default()
                    .event("lag")
                    .data(format!("skipped {}", skipped));
                Some((Ok(lag_event), rx))
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
        }
    });

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
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

    let event_stream = stream::unfold(rx, move |mut rx: broadcast::Receiver<BusEnvelope>| {
        let allowed_topics = allowed_topics.clone();
        async move {
            match rx.recv().await {
                Ok(envelope) => {
                    // Filter by allowed topics if any are specified
                    let should_send = allowed_topics.is_empty()
                        || allowed_topics
                            .iter()
                            .any(|pattern| topic_matches(&envelope.topic, pattern));

                    if should_send {
                        let payload =
                            serde_json::to_string(&envelope).unwrap_or_else(|_| "{}".to_string());
                        let sse_event = Event::default().event("bus").data(payload);
                        return Some((Ok(sse_event), rx));
                    }

                    // Skip this event but keep the receiver
                    Some((Ok(Event::default().event("keepalive").data("")), rx))
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                    let lag_event = Event::default()
                        .event("lag")
                        .data(format!("skipped {}", skipped));
                    Some((Ok(lag_event), rx))
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
            }
        }
    });

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
}

/// Check if a topic matches a pattern.
/// Supports wildcards: `agent.*` matches `agent.123`, `agent.*.events` matches `agent.456.events`
fn topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    if let Some(suffix) = pattern.strip_prefix(".*") {
        return topic.ends_with(suffix);
    }
    topic == pattern
}

async fn get_latest_snapshot(
    State(state): State<AppState>,
) -> Result<Json<MemorySnapshot>, (StatusCode, String)> {
    match state.cognition.latest_snapshot().await {
        Some(snapshot) => Ok(Json(snapshot)),
        None => Err((StatusCode::NOT_FOUND, "No snapshots available".to_string())),
    }
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

// ── Belief, Attention, Governance, Workspace handlers ──

#[derive(Deserialize)]
struct BeliefFilter {
    status: Option<String>,
    persona: Option<String>,
}

async fn list_beliefs(
    State(state): State<AppState>,
    Query(filter): Query<BeliefFilter>,
) -> Result<Json<Vec<Belief>>, (StatusCode, String)> {
    let beliefs = state.cognition.get_beliefs().await;
    let mut result: Vec<Belief> = beliefs.into_values().collect();

    if let Some(status) = &filter.status {
        result.retain(|b| {
            let s = serde_json::to_string(&b.status).unwrap_or_default();
            s.contains(status)
        });
    }
    if let Some(persona) = &filter.persona {
        result.retain(|b| &b.asserted_by == persona);
    }

    result.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(Json(result))
}

async fn get_belief(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Belief>, (StatusCode, String)> {
    match state.cognition.get_belief(&id).await {
        Some(belief) => Ok(Json(belief)),
        None => Err((StatusCode::NOT_FOUND, format!("Belief not found: {}", id))),
    }
}

async fn list_attention(
    State(state): State<AppState>,
) -> Result<Json<Vec<AttentionItem>>, (StatusCode, String)> {
    Ok(Json(state.cognition.get_attention_queue().await))
}

async fn list_proposals(
    State(state): State<AppState>,
) -> Result<Json<Vec<Proposal>>, (StatusCode, String)> {
    let proposals = state.cognition.get_proposals().await;
    let mut result: Vec<Proposal> = proposals.into_values().collect();
    result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(Json(result))
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
    state
        .cognition
        .get_persona(&id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Persona not found: {}", id)))
}

// ── Audit trail endpoints ──

#[derive(Deserialize)]
struct AuditQuery {
    limit: Option<usize>,
    category: Option<String>,
}

async fn list_audit_entries(
    State(state): State<AppState>,
    Query(query): Query<AuditQuery>,
) -> Result<Json<Vec<audit::AuditEntry>>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(100).min(1000);

    let entries = if let Some(ref cat) = query.category {
        let category = match cat.as_str() {
            "api" => AuditCategory::Api,
            "tool" | "tool_execution" => AuditCategory::ToolExecution,
            "session" => AuditCategory::Session,
            "cognition" => AuditCategory::Cognition,
            "swarm" => AuditCategory::Swarm,
            "auth" => AuditCategory::Auth,
            "k8s" => AuditCategory::K8s,
            "sandbox" => AuditCategory::Sandbox,
            "config" => AuditCategory::Config,
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Unknown category: {}", cat),
                ));
            }
        };
        state.audit_log.by_category(category, limit).await
    } else {
        state.audit_log.recent(limit).await
    };

    Ok(Json(entries))
}

// ── Event Stream Replay API ──
// Enables auditors to reconstruct sessions from byte-range offsets.
// This is the key compliance feature for SOC 2, FedRAMP, and ATO processes.

#[derive(Deserialize)]
struct ReplayQuery {
    /// Session ID to replay
    session_id: String,
    /// Optional: starting byte offset (for seeking to specific point)
    start_offset: Option<u64>,
    /// Optional: ending byte offset
    end_offset: Option<u64>,
    /// Optional: limit number of events to return
    limit: Option<usize>,
    /// Optional: filter by tool name
    tool_name: Option<String>,
}

/// Replay session events from the JSONL event stream by byte-range offsets.
/// This allows auditors to reconstruct exactly what happened in a session,
/// including tool execution durations and success/failure status.
async fn replay_session_events(
    Query(query): Query<ReplayQuery>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, String)> {
    use std::path::PathBuf;

    let base_dir = std::env::var("CODETETHER_EVENT_STREAM_PATH")
        .map(PathBuf::from)
        .ok()
        .ok_or_else(|| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Event stream not configured. Set CODETETHER_EVENT_STREAM_PATH.".to_string(),
            )
        })?;

    let session_dir = base_dir.join(&query.session_id);

    // Check if session directory exists
    if !session_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Session not found: {}", query.session_id),
        ));
    }

    let mut all_events: Vec<(u64, u64, serde_json::Value)> = Vec::new();

    // Read all event files in the session directory using std::fs for simplicity
    let entries = std::fs::read_dir(&session_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for entry in entries {
        let entry = entry.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }

        // Parse byte range from filename: {timestamp}-chat-events-{start}-{end}.jsonl
        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        if let Some(offsets) = filename
            .strip_prefix("T")
            .or_else(|| filename.strip_prefix("202"))
        {
            // Extract start and end offsets from filename
            let parts: Vec<&str> = offsets.split('-').collect();
            if parts.len() >= 4 {
                let start: u64 = parts[parts.len() - 2].parse().unwrap_or(0);
                let end: u64 = parts[parts.len() - 1]
                    .trim_end_matches(".jsonl")
                    .parse()
                    .unwrap_or(0);

                // Filter by byte range if specified
                if let Some(query_start) = query.start_offset
                    && end <= query_start
                {
                    continue;
                }
                if let Some(query_end) = query.end_offset
                    && start >= query_end
                {
                    continue;
                }

                // Read and parse events from this file
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                for line in content.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Ok(event) = serde_json::from_str::<serde_json::Value>(line) {
                        // Filter by tool name if specified
                        if let Some(ref tool_filter) = query.tool_name {
                            if let Some(event_tool) =
                                event.get("tool_name").and_then(|v| v.as_str())
                            {
                                if event_tool != tool_filter {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }
                        all_events.push((start, end, event));
                    }
                }
            }
        }
    }

    // Sort by start offset
    all_events.sort_by_key(|(s, _, _)| *s);

    // Apply limit
    let limit = query.limit.unwrap_or(1000).min(10000);
    let events: Vec<_> = all_events
        .into_iter()
        .take(limit)
        .map(|(_, _, e)| e)
        .collect();

    Ok(Json(events))
}

/// Get session event files metadata (for audit index)
async fn list_session_event_files(
    Query(query): Query<ReplayQuery>,
) -> Result<Json<Vec<EventFileMeta>>, (StatusCode, String)> {
    use std::path::PathBuf;

    let base_dir = std::env::var("CODETETHER_EVENT_STREAM_PATH")
        .map(PathBuf::from)
        .ok()
        .ok_or_else(|| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Event stream not configured. Set CODETETHER_EVENT_STREAM_PATH.".to_string(),
            )
        })?;

    let session_dir = base_dir.join(&query.session_id);
    if !session_dir.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Session not found: {}", query.session_id),
        ));
    }

    let mut files: Vec<EventFileMeta> = Vec::new();

    // Use std::fs for simplicity
    let entries = std::fs::read_dir(&session_dir)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for entry in entries {
        let entry = entry.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("jsonl") {
            continue;
        }

        let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        // Parse byte range from filename
        if let Some(offsets) = filename
            .strip_prefix("T")
            .or_else(|| filename.strip_prefix("202"))
        {
            let parts: Vec<&str> = offsets.split('-').collect();
            if parts.len() >= 4 {
                let start: u64 = parts[parts.len() - 2].parse().unwrap_or(0);
                let end: u64 = parts[parts.len() - 1]
                    .trim_end_matches(".jsonl")
                    .parse()
                    .unwrap_or(0);

                let metadata = std::fs::metadata(&path)
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                files.push(EventFileMeta {
                    filename: filename.to_string(),
                    start_offset: start,
                    end_offset: end,
                    size_bytes: metadata.len(),
                });
            }
        }
    }

    files.sort_by_key(|f| f.start_offset);
    Ok(Json(files))
}

#[derive(Serialize)]
struct EventFileMeta {
    filename: String,
    start_offset: u64,
    end_offset: u64,
    size_bytes: u64,
}

// ── K8s self-deployment endpoints ──

async fn get_k8s_status(
    State(state): State<AppState>,
) -> Result<Json<crate::k8s::K8sStatus>, (StatusCode, String)> {
    Ok(Json(state.k8s.status().await))
}

#[derive(Deserialize)]
struct ScaleRequest {
    replicas: i32,
}

async fn k8s_scale(
    State(state): State<AppState>,
    Json(req): Json<ScaleRequest>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    if req.replicas < 0 || req.replicas > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Replicas must be between 0 and 100".to_string(),
        ));
    }

    state
        .audit_log
        .log(
            AuditCategory::K8s,
            format!("scale:{}", req.replicas),
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

    state
        .k8s
        .scale(req.replicas)
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn k8s_restart(
    State(state): State<AppState>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    state
        .audit_log
        .log(
            AuditCategory::K8s,
            "rolling_restart",
            AuditOutcome::Success,
            None,
            None,
        )
        .await;

    state
        .k8s
        .rolling_restart()
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn k8s_list_pods(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::k8s::PodInfo>>, (StatusCode, String)> {
    state
        .k8s
        .list_pods()
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn k8s_actions(
    State(state): State<AppState>,
) -> Result<Json<Vec<crate::k8s::DeployAction>>, (StatusCode, String)> {
    Ok(Json(state.k8s.recent_actions(100).await))
}

#[derive(Deserialize)]
struct SpawnSubagentRequest {
    subagent_id: String,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    env_vars: std::collections::HashMap<String, String>,
    #[serde(default)]
    labels: std::collections::HashMap<String, String>,
    #[serde(default)]
    command: Option<Vec<String>>,
    #[serde(default)]
    args: Option<Vec<String>>,
}

async fn k8s_spawn_subagent(
    State(state): State<AppState>,
    Json(req): Json<SpawnSubagentRequest>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    state
        .k8s
        .spawn_subagent_pod_with_spec(
            &req.subagent_id,
            crate::k8s::SubagentPodSpec {
                image: req.image,
                env_vars: req.env_vars,
                labels: req.labels,
                command: req.command,
                args: req.args,
            },
        )
        .await
        .map(Json)
        .map_err(internal_error)
}

async fn k8s_delete_subagent(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<crate::k8s::DeployAction>, (StatusCode, String)> {
    state
        .k8s
        .delete_subagent_pod(&id)
        .await
        .map(Json)
        .map_err(internal_error)
}

/// Request to register a tool
#[derive(Debug, Deserialize)]
pub struct RegisterToolRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub parameters: serde_json::Value,
}

/// Response from registering a tool
#[derive(Serialize)]
struct RegisterToolResponse {
    tool: RegisteredTool,
    message: String,
}

/// List all registered tools (active, non-expired)
async fn list_tools(State(state): State<AppState>) -> Json<Vec<RegisteredTool>> {
    Json(state.tool_registry.list().await)
}

/// Register a new tool
async fn register_tool(
    State(state): State<AppState>,
    Json(req): Json<RegisterToolRequest>,
) -> Result<Json<RegisterToolResponse>, (StatusCode, String)> {
    let now = chrono::Utc::now();
    let tool = RegisteredTool {
        id: req.id.clone(),
        name: req.name,
        description: req.description,
        version: req.version,
        endpoint: req.endpoint,
        capabilities: req.capabilities,
        parameters: req.parameters,
        registered_at: now,
        last_heartbeat: now,
        expires_at: now + Duration::from_secs(90),
    };

    state.tool_registry.register(tool.clone()).await;

    tracing::info!(tool_id = %tool.id, "Tool registered");

    Ok(Json(RegisterToolResponse {
        tool,
        message: "Tool registered successfully. Heartbeat required every 30s.".to_string(),
    }))
}

/// Heartbeat endpoint to extend tool TTL
async fn tool_heartbeat(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<RegisteredTool>, (StatusCode, String)> {
    state
        .tool_registry
        .heartbeat(&id)
        .await
        .map(|tool| {
            tracing::info!(tool_id = %id, "Tool heartbeat received");
            Json(tool)
        })
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Tool {} not found", id)))
}

/// List registered plugins.
async fn list_plugins(State(_state): State<AppState>) -> Json<PluginListResponse> {
    let server_fingerprint = hash_bytes(env!("CARGO_PKG_VERSION").as_bytes());
    let signing_key = SigningKey::from_env();
    let test_sig = signing_key.sign("_probe", "0.0.0", &server_fingerprint);
    Json(PluginListResponse {
        server_fingerprint,
        signing_available: !test_sig.is_empty(),
        plugins: Vec::<PluginManifest>::new(),
    })
}

#[derive(Serialize)]
struct PluginListResponse {
    server_fingerprint: String,
    signing_available: bool,
    plugins: Vec<PluginManifest>,
}

// ── Agent task compatibility surface ─────────────────────────────────────
// These handlers provide the /v1/agent/tasks/* surface the marketing-site
// dashboard expects, backed by the existing KnativeTaskQueue.

/// List all agent tasks (wraps Knative task queue)
async fn list_agent_tasks(
    State(state): State<AppState>,
    Query(params): Query<ListAgentTasksQuery>,
) -> Json<serde_json::Value> {
    let tasks = state.knative_tasks.list().await;
    let filtered: Vec<_> = tasks
        .into_iter()
        .filter(|t| params.status.as_ref().is_none_or(|s| t.status == *s))
        .filter(|t| {
            params
                .agent_type
                .as_ref()
                .is_none_or(|a| t.agent_type == *a)
        })
        .collect();
    Json(serde_json::json!({ "tasks": filtered, "total": filtered.len() }))
}

#[derive(Deserialize)]
struct ListAgentTasksQuery {
    status: Option<String>,
    agent_type: Option<String>,
}

/// Create a new agent task
async fn create_agent_task(
    State(state): State<AppState>,
    Json(req): Json<CreateAgentTaskRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let task_id = uuid::Uuid::new_v4().to_string();
    let task = KnativeTask {
        task_id: task_id.clone(),
        title: req.title,
        description: req.description,
        agent_type: req.agent_type.unwrap_or_else(|| "build".to_string()),
        priority: req.priority.unwrap_or(0),
        received_at: chrono::Utc::now(),
        status: "pending".to_string(),
    };
    state.knative_tasks.push(task).await;
    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "pending",
    })))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CreateAgentTaskRequest {
    title: String,
    description: String,
    agent_type: Option<String>,
    model: Option<String>,
    priority: Option<i32>,
}

/// Get a single agent task by ID
async fn get_agent_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<KnativeTask>, (StatusCode, String)> {
    state
        .knative_tasks
        .get(&task_id)
        .await
        .map(Json)
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))
}

/// Get task output (returns task details including result)
async fn get_agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let task = state
        .knative_tasks
        .get(&task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))?;
    Ok(Json(serde_json::json!({
        "task_id": task.task_id,
        "status": task.status,
        "title": task.title,
        "output": null,
    })))
}

/// Receive task output from worker and broadcast via bus for SSE streaming
#[derive(Deserialize)]
struct TaskOutputPayload {
    #[allow(dead_code)]
    #[serde(default)]
    worker_id: Option<String>,
    #[serde(default)]
    output: Option<String>,
}

async fn agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(payload): Json<TaskOutputPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Verify task exists
    let task_exists = state.knative_tasks.get(&task_id).await.is_some();
    if !task_exists {
        return Err((StatusCode::NOT_FOUND, format!("Task {} not found", task_id)));
    }

    // Update task status to Working if it's still pending
    let _ = state.knative_tasks.update_status(&task_id, "working").await;

    // Broadcast output via bus for SSE subscribers
    if let Some(ref output) = payload.output {
        let bus_handle = state.bus.handle("task-output");
        let _ = bus_handle.send(
            format!("task.{}", task_id),
            crate::bus::BusMessage::TaskUpdate {
                task_id: task_id.clone(),
                state: crate::a2a::types::TaskState::Working,
                message: Some(output.clone()),
            },
        );
    }

    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "received",
    })))
}

/// Stream task output as SSE events
async fn stream_agent_task_output(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Sse<impl stream::Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    // Verify task exists
    state
        .knative_tasks
        .get(&task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {} not found", task_id)))?;

    let bus_handle = state.bus.handle("task-stream");
    let rx = bus_handle.into_receiver();
    let topic_prefix = format!("task.{task_id}");

    let stream = async_stream::try_stream! {
        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(envelope) => {
                    if !envelope.topic.starts_with(&topic_prefix) {
                        continue;
                    }
                    let data = serde_json::to_string(&envelope.message).unwrap_or_default();
                    yield Event::default().event("output").data(data);
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

// ── Worker connectivity ─────────────────────────────────────────────────

/// List connected workers (K8s pods with agent label)
async fn list_connected_workers(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Use K8s pod listing as proxy for connected workers
    let pods = state.k8s.list_pods().await.unwrap_or_default();
    let workers: Vec<serde_json::Value> = pods
        .iter()
        .filter(|p| p.name.contains("codetether") || p.name.contains("worker"))
        .map(|p| {
            serde_json::json!({
                "worker_id": p.name,
                "name": p.name,
                "status": p.phase,
                "is_sse_connected": p.phase == "Running",
                "last_seen": chrono::Utc::now().to_rfc3339(),
            })
        })
        .collect();
    Ok(Json(serde_json::json!({ "workers": workers })))
}

// ── Task dispatch ───────────────────────────────────────────────────────

/// Dispatch a task (creates a Knative task and returns immediately)
async fn dispatch_task(
    State(state): State<AppState>,
    Json(req): Json<DispatchTaskRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let task_id = uuid::Uuid::new_v4().to_string();
    let task = KnativeTask {
        task_id: task_id.clone(),
        title: req.title,
        description: req.description,
        agent_type: req.agent_type.unwrap_or_else(|| "build".to_string()),
        priority: req.priority.unwrap_or(0),
        received_at: chrono::Utc::now(),
        status: "pending".to_string(),
    };

    // Publish task event to agent bus for connected workers
    let handle = state.bus.handle("task-dispatch");
    handle.send(
        format!("task.{}", task_id),
        crate::bus::BusMessage::TaskUpdate {
            task_id: task_id.clone(),
            state: crate::a2a::types::TaskState::Submitted,
            message: Some(format!("Task dispatched: {}", task.title)),
        },
    );

    state.knative_tasks.push(task).await;

    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "pending",
        "dispatched_via_knative": true,
    })))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct DispatchTaskRequest {
    title: String,
    description: String,
    agent_type: Option<String>,
    model: Option<String>,
    priority: Option<i32>,
    metadata: Option<serde_json::Value>,
}

// ── Voice REST bridge ───────────────────────────────────────────────────
// Bridges the REST /v1/voice/* surface the dashboard expects to the
// existing gRPC VoiceService implementation via the AgentBus.

/// Create a voice session (REST bridge for VoiceService::CreateVoiceSession)
async fn create_voice_session_rest(
    State(state): State<AppState>,
    Json(req): Json<CreateVoiceSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let voice_id = req.voice.unwrap_or_else(|| "960f89fc".to_string());
    let room_name = format!("voice-{}", uuid::Uuid::new_v4());

    // Publish session started event to bus
    let handle = state.bus.handle("voice-rest");
    handle.send_voice_session_started(&room_name, &voice_id);

    let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or_default();

    Ok(Json(serde_json::json!({
        "room_name": room_name,
        "voice": voice_id,
        "mode": req.mode.unwrap_or_else(|| "chat".to_string()),
        "access_token": "",
        "livekit_url": livekit_url,
        "expires_at": (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339(),
    })))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct CreateVoiceSessionRequest {
    voice: Option<String>,
    mode: Option<String>,
    codebase_id: Option<String>,
    user_id: Option<String>,
}

/// Get a voice session (REST bridge)
async fn get_voice_session_rest(Path(room_name): Path<String>) -> Json<serde_json::Value> {
    let livekit_url = std::env::var("LIVEKIT_URL").unwrap_or_default();
    Json(serde_json::json!({
        "room_name": room_name,
        "state": "active",
        "agent_state": "listening",
        "access_token": "",
        "livekit_url": livekit_url,
    }))
}

/// Delete a voice session (REST bridge)
async fn delete_voice_session_rest(
    State(state): State<AppState>,
    Path(room_name): Path<String>,
) -> Json<serde_json::Value> {
    let handle = state.bus.handle("voice-rest");
    handle.send_voice_session_ended(&room_name, "user_ended");
    Json(serde_json::json!({ "status": "deleted" }))
}

/// List available voices (REST bridge)
async fn list_voices_rest() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "voices": [
            { "voice_id": "960f89fc", "name": "Riley", "language": "english" },
            { "voice_id": "puck", "name": "Puck", "language": "english" },
            { "voice_id": "charon", "name": "Charon", "language": "english" },
            { "voice_id": "kore", "name": "Kore", "language": "english" },
            { "voice_id": "fenrir", "name": "Fenrir", "language": "english" },
            { "voice_id": "aoede", "name": "Aoede", "language": "english" },
        ]
    }))
}

// ── Session resume ──────────────────────────────────────────────────────

/// Resume a codebase-scoped session (what the dashboard session resume flow calls)
async fn resume_codebase_session(
    Path((_codebase_id, session_id)): Path<(String, String)>,
    Json(req): Json<ResumeSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Load the session if it exists, or create a new one
    let mut session = match crate::session::Session::load(&session_id).await {
        Ok(s) => s,
        Err(_) => {
            // Create a new session for this codebase
            let mut s = crate::session::Session::new()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            if let Some(agent) = &req.agent {
                s.agent = agent.clone();
            }
            s.save()
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            s
        }
    };

    // If there's a prompt, execute it as a task
    if let Some(prompt) = &req.prompt
        && !prompt.is_empty()
    {
        match session.prompt(prompt).await {
            Ok(result) => {
                session.save().await.ok();
                return Ok(Json(serde_json::json!({
                    "session_id": session.id,
                    "active_session_id": session.id,
                    "status": "completed",
                    "result": result.text,
                })));
            }
            Err(e) => {
                return Ok(Json(serde_json::json!({
                    "session_id": session.id,
                    "active_session_id": session.id,
                    "status": "failed",
                    "error": e.to_string(),
                })));
            }
        }
    }

    // No prompt = just resume/check session
    Ok(Json(serde_json::json!({
        "session_id": session.id,
        "active_session_id": session.id,
        "status": "ready",
    })))
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ResumeSessionRequest {
    prompt: Option<String>,
    agent: Option<String>,
    model: Option<String>,
}

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

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|v| match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::{match_policy_rule, normalize_model_reference};

    #[test]
    fn policy_prompt_session_requires_execute_permission() {
        let permission = match_policy_rule("/api/session/abc123/prompt", "POST");
        assert_eq!(permission, Some("agent:execute"));
    }

    #[test]
    fn policy_create_session_keeps_sessions_write_permission() {
        let permission = match_policy_rule("/api/session", "POST");
        assert_eq!(permission, Some("sessions:write"));
    }

    #[test]
    fn policy_proposal_approval_requires_execute_permission() {
        let permission = match_policy_rule("/v1/cognition/proposals/p1/approve", "POST");
        assert_eq!(permission, Some("agent:execute"));
    }

    #[test]
    fn policy_openai_models_requires_read_permission() {
        let permission = match_policy_rule("/v1/models", "GET");
        assert_eq!(permission, Some("agent:read"));
    }

    #[test]
    fn policy_openai_chat_completions_requires_execute_permission() {
        let permission = match_policy_rule("/v1/chat/completions", "POST");
        assert_eq!(permission, Some("agent:execute"));
    }

    #[test]
    fn normalize_model_reference_accepts_colon_format() {
        assert_eq!(
            normalize_model_reference("openai:gpt-4o"),
            "openai/gpt-4o".to_string()
        );
    }
}
