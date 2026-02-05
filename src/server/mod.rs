//! HTTP Server
//!
//! Main API server for the CodeTether Agent

use crate::a2a;
use crate::cli::ServeArgs;
use crate::config::Config;
use anyhow::Result;
use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Server state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
}

/// Start the HTTP server
pub async fn serve(args: ServeArgs) -> Result<()> {
    let config = Config::load().await?;
    let state = AppState {
        config: Arc::new(config),
    };

    let addr = format!("{}:{}", args.hostname, args.port);

    // Build the agent card
    let agent_card = a2a::server::A2AServer::default_card(&format!("http://{}", addr));
    let a2a_server = a2a::server::A2AServer::new(agent_card);

    // Build A2A router separately
    let a2a_router = a2a_server.router();

    let app = Router::new()
        // Health check
        .route("/health", get(health))
        // API routes
        .route("/api/version", get(get_version))
        .route("/api/session", get(list_sessions).post(create_session))
        .route("/api/session/:id", get(get_session))
        .route("/api/session/:id/prompt", post(prompt_session))
        .route("/api/config", get(get_config))
        .route("/api/provider", get(list_providers))
        .route("/api/agent", get(list_agents))
        .with_state(state)
        // A2A routes (nested to work with different state type)
        .nest("/a2a", a2a_router)
        // Middleware
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check response
async fn health() -> &'static str {
    "ok"
}

/// Version info
#[derive(Serialize)]
struct VersionInfo {
    version: &'static str,
    name: &'static str,
}

async fn get_version() -> Json<VersionInfo> {
    Json(VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        name: env!("CARGO_PKG_NAME"),
    })
}

/// List sessions
#[derive(Deserialize)]
struct ListSessionsQuery {
    limit: Option<usize>,
}

async fn list_sessions(
    Query(query): Query<ListSessionsQuery>,
) -> Result<Json<Vec<crate::session::SessionSummary>>, (StatusCode, String)> {
    let sessions = crate::session::list_sessions()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let limit = query.limit.unwrap_or(50);
    Ok(Json(sessions.into_iter().take(limit).collect()))
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
