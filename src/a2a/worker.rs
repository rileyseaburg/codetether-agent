//! A2A Worker - connects to an A2A server to process tasks

use crate::cli::A2aArgs;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::swarm_view::SwarmEvent;
use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;
use tokio::time::Instant;

/// Worker status for heartbeat
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerStatus {
    Idle,
    Processing,
}

impl WorkerStatus {
    fn as_str(&self) -> &'static str {
        match self {
            WorkerStatus::Idle => "idle",
            WorkerStatus::Processing => "processing",
        }
    }
}

/// Heartbeat state shared between the heartbeat task and the main worker
#[derive(Clone)]
struct HeartbeatState {
    worker_id: String,
    agent_name: String,
    status: Arc<Mutex<WorkerStatus>>,
    active_task_count: Arc<Mutex<usize>>,
}

impl HeartbeatState {
    fn new(worker_id: String, agent_name: String) -> Self {
        Self {
            worker_id,
            agent_name,
            status: Arc::new(Mutex::new(WorkerStatus::Idle)),
            active_task_count: Arc::new(Mutex::new(0)),
        }
    }

    async fn set_status(&self, status: WorkerStatus) {
        *self.status.lock().await = status;
    }

    async fn set_task_count(&self, count: usize) {
        *self.active_task_count.lock().await = count;
    }
}

#[derive(Clone, Debug)]
struct CognitionHeartbeatConfig {
    enabled: bool,
    source_base_url: String,
    include_thought_summary: bool,
    summary_max_chars: usize,
    request_timeout_ms: u64,
}

impl CognitionHeartbeatConfig {
    fn from_env() -> Self {
        let source_base_url = std::env::var("CODETETHER_WORKER_COGNITION_SOURCE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:4096".to_string())
            .trim_end_matches('/')
            .to_string();

        Self {
            enabled: env_bool("CODETETHER_WORKER_COGNITION_SHARE_ENABLED", true),
            source_base_url,
            include_thought_summary: env_bool("CODETETHER_WORKER_COGNITION_INCLUDE_THOUGHTS", true),
            summary_max_chars: env_usize("CODETETHER_WORKER_COGNITION_THOUGHT_MAX_CHARS", 480)
                .max(120),
            request_timeout_ms: env_u64("CODETETHER_WORKER_COGNITION_TIMEOUT_MS", 2_500).max(250),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CognitionStatusSnapshot {
    running: bool,
    #[serde(default)]
    last_tick_at: Option<String>,
    #[serde(default)]
    active_persona_count: usize,
    #[serde(default)]
    events_buffered: usize,
    #[serde(default)]
    snapshots_buffered: usize,
    #[serde(default)]
    loop_interval_ms: u64,
}

#[derive(Debug, Deserialize)]
struct CognitionLatestSnapshot {
    generated_at: String,
    summary: String,
    #[serde(default)]
    metadata: HashMap<String, serde_json::Value>,
}

/// Run the A2A worker
pub async fn run(args: A2aArgs) -> Result<()> {
    let server = args.server.trim_end_matches('/');
    let name = args
        .name
        .unwrap_or_else(|| format!("codetether-{}", std::process::id()));
    let worker_id = generate_worker_id();

    let codebases: Vec<String> = args
        .codebases
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()]);

    tracing::info!("Starting A2A worker: {} ({})", name, worker_id);
    tracing::info!("Server: {}", server);
    tracing::info!("Codebases: {:?}", codebases);

    let client = Client::new();
    let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
    let cognition_heartbeat = CognitionHeartbeatConfig::from_env();
    if cognition_heartbeat.enabled {
        tracing::info!(
            source = %cognition_heartbeat.source_base_url,
            include_thoughts = cognition_heartbeat.include_thought_summary,
            max_chars = cognition_heartbeat.summary_max_chars,
            timeout_ms = cognition_heartbeat.request_timeout_ms,
            "Cognition heartbeat sharing enabled (set CODETETHER_WORKER_COGNITION_SHARE_ENABLED=false to disable)"
        );
    } else {
        tracing::warn!(
            "Cognition heartbeat sharing disabled; worker thought state will not be shared upstream"
        );
    }

    let auto_approve = match args.auto_approve.as_str() {
        "all" => AutoApprove::All,
        "safe" => AutoApprove::Safe,
        _ => AutoApprove::None,
    };

    // Create heartbeat state
    let heartbeat_state = HeartbeatState::new(worker_id.clone(), name.clone());

    // Register worker
    register_worker(&client, server, &args.token, &worker_id, &name, &codebases).await?;

    // Fetch pending tasks
    fetch_pending_tasks(
        &client,
        server,
        &args.token,
        &worker_id,
        &processing,
        &auto_approve,
    )
    .await?;

    // Connect to SSE stream
    loop {
        // Re-register worker on each reconnection to report updated models/capabilities
        if let Err(e) =
            register_worker(&client, server, &args.token, &worker_id, &name, &codebases).await
        {
            tracing::warn!("Failed to re-register worker on reconnection: {}", e);
        }

        // Start heartbeat task for this connection
        let heartbeat_handle = start_heartbeat(
            client.clone(),
            server.to_string(),
            args.token.clone(),
            heartbeat_state.clone(),
            processing.clone(),
            cognition_heartbeat.clone(),
        );

        match connect_stream(
            &client,
            server,
            &args.token,
            &worker_id,
            &name,
            &codebases,
            &processing,
            &auto_approve,
        )
        .await
        {
            Ok(()) => {
                tracing::warn!("Stream ended, reconnecting...");
            }
            Err(e) => {
                tracing::error!("Stream error: {}, reconnecting...", e);
            }
        }

        // Cancel heartbeat on disconnection
        heartbeat_handle.abort();
        tracing::debug!("Heartbeat cancelled for reconnection");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

fn generate_worker_id() -> String {
    format!(
        "wrk_{}_{:x}",
        chrono::Utc::now().timestamp(),
        rand::random::<u64>()
    )
}

#[derive(Debug, Clone, Copy)]
enum AutoApprove {
    All,
    Safe,
    None,
}

/// Capabilities of the codetether-agent worker
const WORKER_CAPABILITIES: &[&str] = &["ralph", "swarm", "rlm", "a2a", "mcp"];

fn task_value<'a>(task: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    task.get("task")
        .and_then(|t| t.get(key))
        .or_else(|| task.get(key))
}

fn task_str<'a>(task: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    task_value(task, key).and_then(|v| v.as_str())
}

fn task_metadata(task: &serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    task_value(task, "metadata")
        .and_then(|m| m.as_object())
        .cloned()
        .unwrap_or_default()
}

fn model_ref_to_provider_model(model: &str) -> String {
    // Convert "provider:model" to "provider/model" format, but only if
    // there is no '/' already present. Model IDs like "amazon.nova-micro-v1:0"
    // contain colons as version separators and must NOT be converted.
    if !model.contains('/') && model.contains(':') {
        model.replacen(':', "/", 1)
    } else {
        model.to_string()
    }
}

fn provider_preferences_for_tier(model_tier: Option<&str>) -> &'static [&'static str] {
    match model_tier.unwrap_or("balanced") {
        "fast" | "quick" => &[
            "openai",
            "github-copilot",
            "moonshotai",
            "zhipuai",
            "openrouter",
            "novita",
            "google",
            "anthropic",
        ],
        "heavy" | "deep" => &[
            "anthropic",
            "openai",
            "github-copilot",
            "moonshotai",
            "zhipuai",
            "openrouter",
            "novita",
            "google",
        ],
        _ => &[
            "openai",
            "github-copilot",
            "anthropic",
            "moonshotai",
            "zhipuai",
            "openrouter",
            "novita",
            "google",
        ],
    }
}

fn choose_provider_for_tier<'a>(providers: &'a [&'a str], model_tier: Option<&str>) -> &'a str {
    for preferred in provider_preferences_for_tier(model_tier) {
        if let Some(found) = providers.iter().copied().find(|p| *p == *preferred) {
            return found;
        }
    }
    if let Some(found) = providers.iter().copied().find(|p| *p == "zhipuai") {
        return found;
    }
    providers[0]
}

fn default_model_for_provider(provider: &str, model_tier: Option<&str>) -> String {
    match model_tier.unwrap_or("balanced") {
        "fast" | "quick" => match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-haiku-4-5".to_string(),
            "openai" => "gpt-4o-mini".to_string(),
            "google" => "gemini-2.5-flash".to_string(),
            "zhipuai" => "glm-4.7".to_string(),
            "openrouter" => "z-ai/glm-4.7".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            _ => "glm-4.7".to_string(),
        },
        "heavy" | "deep" => match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "openai" => "o3".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "zhipuai" => "glm-4.7".to_string(),
            "openrouter" => "z-ai/glm-4.7".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            _ => "glm-4.7".to_string(),
        },
        _ => match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "zhipuai" => "glm-4.7".to_string(),
            "openrouter" => "z-ai/glm-4.7".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            _ => "glm-4.7".to_string(),
        },
    }
}

fn is_swarm_agent(agent_type: &str) -> bool {
    matches!(
        agent_type.trim().to_ascii_lowercase().as_str(),
        "swarm" | "parallel" | "multi-agent"
    )
}

fn metadata_lookup<'a>(
    metadata: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<&'a serde_json::Value> {
    metadata
        .get(key)
        .or_else(|| {
            metadata
                .get("routing")
                .and_then(|v| v.as_object())
                .and_then(|obj| obj.get(key))
        })
        .or_else(|| {
            metadata
                .get("swarm")
                .and_then(|v| v.as_object())
                .and_then(|obj| obj.get(key))
        })
}

fn metadata_str(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<String> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key).and_then(|v| v.as_str()) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn metadata_usize(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<usize> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_u64() {
                return usize::try_from(v).ok();
            }
            if let Some(v) = value.as_i64() {
                if v >= 0 {
                    return usize::try_from(v as u64).ok();
                }
            }
            if let Some(v) = value.as_str() {
                if let Ok(parsed) = v.trim().parse::<usize>() {
                    return Some(parsed);
                }
            }
        }
    }
    None
}

fn metadata_u64(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<u64> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_u64() {
                return Some(v);
            }
            if let Some(v) = value.as_i64() {
                if v >= 0 {
                    return Some(v as u64);
                }
            }
            if let Some(v) = value.as_str() {
                if let Ok(parsed) = v.trim().parse::<u64>() {
                    return Some(parsed);
                }
            }
        }
    }
    None
}

fn metadata_bool(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<bool> {
    for key in keys {
        if let Some(value) = metadata_lookup(metadata, key) {
            if let Some(v) = value.as_bool() {
                return Some(v);
            }
            if let Some(v) = value.as_str() {
                match v.trim().to_ascii_lowercase().as_str() {
                    "1" | "true" | "yes" | "on" => return Some(true),
                    "0" | "false" | "no" | "off" => return Some(false),
                    _ => {}
                }
            }
        }
    }
    None
}

fn parse_swarm_strategy(
    metadata: &serde_json::Map<String, serde_json::Value>,
) -> DecompositionStrategy {
    match metadata_str(
        metadata,
        &[
            "decomposition_strategy",
            "swarm_strategy",
            "strategy",
            "swarm_decomposition",
        ],
    )
    .as_deref()
    .map(|s| s.to_ascii_lowercase())
    .as_deref()
    {
        Some("none") | Some("single") => DecompositionStrategy::None,
        Some("domain") | Some("by_domain") => DecompositionStrategy::ByDomain,
        Some("data") | Some("by_data") => DecompositionStrategy::ByData,
        Some("stage") | Some("by_stage") => DecompositionStrategy::ByStage,
        _ => DecompositionStrategy::Automatic,
    }
}

async fn resolve_swarm_model(
    explicit_model: Option<String>,
    model_tier: Option<&str>,
) -> Option<String> {
    if let Some(model) = explicit_model {
        if !model.trim().is_empty() {
            return Some(model);
        }
    }

    let registry = ProviderRegistry::from_vault().await.ok()?;
    let providers = registry.list();
    if providers.is_empty() {
        return None;
    }
    let provider = choose_provider_for_tier(providers.as_slice(), model_tier);
    let model = default_model_for_provider(provider, model_tier);
    Some(format!("{}/{}", provider, model))
}

fn format_swarm_event_for_output(event: &SwarmEvent) -> Option<String> {
    match event {
        SwarmEvent::Started {
            task,
            total_subtasks,
        } => Some(format!(
            "[swarm] started task={} planned_subtasks={}",
            task, total_subtasks
        )),
        SwarmEvent::StageComplete {
            stage,
            completed,
            failed,
        } => Some(format!(
            "[swarm] stage={} completed={} failed={}",
            stage, completed, failed
        )),
        SwarmEvent::SubTaskUpdate { id, status, .. } => Some(format!(
            "[swarm] subtask id={} status={}",
            &id.chars().take(8).collect::<String>(),
            format!("{status:?}").to_ascii_lowercase()
        )),
        SwarmEvent::AgentToolCall {
            subtask_id,
            tool_name,
        } => Some(format!(
            "[swarm] subtask id={} tool={}",
            &subtask_id.chars().take(8).collect::<String>(),
            tool_name
        )),
        SwarmEvent::AgentError { subtask_id, error } => Some(format!(
            "[swarm] subtask id={} error={}",
            &subtask_id.chars().take(8).collect::<String>(),
            error
        )),
        SwarmEvent::Complete { success, stats } => Some(format!(
            "[swarm] complete success={} subtasks={} speedup={:.2}",
            success,
            stats.subagents_completed + stats.subagents_failed,
            stats.speedup_factor
        )),
        SwarmEvent::Error(err) => Some(format!("[swarm] error message={}", err)),
        _ => None,
    }
}

async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
) -> Result<()> {
    // Load ProviderRegistry and collect available models
    let models = match load_provider_models().await {
        Ok(m) => m,
        Err(e) => {
            tracing::warn!(
                "Failed to load provider models: {}, proceeding without model info",
                e
            );
            HashMap::new()
        }
    };

    // Register via the workers/register endpoint
    let mut req = client.post(format!("{}/v1/opencode/workers/register", server));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    // Flatten models HashMap into array of model objects with pricing data
    // matching the format expected by the A2A server's /models and /workers endpoints
    let models_array: Vec<serde_json::Value> = models
        .iter()
        .flat_map(|(provider, model_infos)| {
            model_infos.iter().map(move |m| {
                let mut obj = serde_json::json!({
                    "id": format!("{}/{}", provider, m.id),
                    "name": &m.id,
                    "provider": provider,
                    "provider_id": provider,
                });
                if let Some(input_cost) = m.input_cost_per_million {
                    obj["input_cost_per_million"] = serde_json::json!(input_cost);
                }
                if let Some(output_cost) = m.output_cost_per_million {
                    obj["output_cost_per_million"] = serde_json::json!(output_cost);
                }
                obj
            })
        })
        .collect();

    tracing::info!(
        "Registering worker with {} models from {} providers",
        models_array.len(),
        models.len()
    );

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    let res = req
        .json(&serde_json::json!({
            "worker_id": worker_id,
            "name": name,
            "capabilities": WORKER_CAPABILITIES,
            "hostname": hostname,
            "models": models_array,
            "codebases": codebases,
        }))
        .send()
        .await?;

    if res.status().is_success() {
        tracing::info!("Worker registered successfully");
    } else {
        tracing::warn!("Failed to register worker: {}", res.status());
    }

    Ok(())
}

/// Load ProviderRegistry and collect all available models grouped by provider.
/// Tries Vault first, then falls back to config/env vars if Vault is unreachable.
/// Returns ModelInfo structs (with pricing data when available).
async fn load_provider_models() -> Result<HashMap<String, Vec<crate::provider::ModelInfo>>> {
    // Try Vault first
    let registry = match ProviderRegistry::from_vault().await {
        Ok(r) if !r.list().is_empty() => {
            tracing::info!("Loaded {} providers from Vault", r.list().len());
            r
        }
        Ok(_) => {
            tracing::warn!("Vault returned 0 providers, falling back to config/env vars");
            fallback_registry().await?
        }
        Err(e) => {
            tracing::warn!("Vault unreachable ({}), falling back to config/env vars", e);
            fallback_registry().await?
        }
    };

    // Fetch the models.dev catalog for pricing data enrichment
    let catalog = crate::provider::models::ModelCatalog::fetch().await.ok();

    // Map provider IDs to their catalog equivalents (some differ)
    let catalog_alias = |pid: &str| -> String {
        match pid {
            "bedrock" => "amazon-bedrock".to_string(),
            "novita" => "novita-ai".to_string(),
            _ => pid.to_string(),
        }
    };

    let mut models_by_provider: HashMap<String, Vec<crate::provider::ModelInfo>> = HashMap::new();

    for provider_name in registry.list() {
        if let Some(provider) = registry.get(provider_name) {
            match provider.list_models().await {
                Ok(models) => {
                    let enriched: Vec<crate::provider::ModelInfo> = models.into_iter().map(|mut m| {
                        // Enrich with catalog pricing if the provider didn't set it
                        if m.input_cost_per_million.is_none() || m.output_cost_per_million.is_none() {
                            if let Some(ref cat) = catalog {
                                let cat_pid = catalog_alias(provider_name);
                                if let Some(prov_info) = cat.get_provider(&cat_pid) {
                                    if let Some(model_info) = prov_info.models.get(&m.id) {
                                        if let Some(ref cost) = model_info.cost {
                                            if m.input_cost_per_million.is_none() {
                                                m.input_cost_per_million = Some(cost.input);
                                            }
                                            if m.output_cost_per_million.is_none() {
                                                m.output_cost_per_million = Some(cost.output);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        m
                    }).collect();
                    if !enriched.is_empty() {
                        tracing::debug!("Provider {}: {} models", provider_name, enriched.len());
                        models_by_provider.insert(provider_name.to_string(), enriched);
                    }
                }
                Err(e) => {
                    tracing::debug!("Failed to list models for {}: {}", provider_name, e);
                }
            }
        }
    }

    // If we still have 0, try the models.dev catalog as last resort
    // NOTE: We list ALL models from the catalog (not just ones with verified API keys)
    // because the worker should advertise what it can handle. The server handles routing.
    if models_by_provider.is_empty() {
        tracing::info!(
            "No authenticated providers found, fetching models.dev catalog (all providers)"
        );
        if let Ok(cat) = crate::provider::models::ModelCatalog::fetch().await {
            // Use all_providers_with_models() to get every provider+model from catalog
            // regardless of API key availability (Vault may be down)
            for (provider_id, provider_info) in cat.all_providers() {
                let model_infos: Vec<crate::provider::ModelInfo> = provider_info
                    .models
                    .values()
                    .map(|m| cat.to_model_info(m, provider_id))
                    .collect();
                if !model_infos.is_empty() {
                    tracing::debug!(
                        "Catalog provider {}: {} models",
                        provider_id,
                        model_infos.len()
                    );
                    models_by_provider.insert(provider_id.clone(), model_infos);
                }
            }
            tracing::info!(
                "Loaded {} providers with {} total models from catalog",
                models_by_provider.len(),
                models_by_provider.values().map(|v| v.len()).sum::<usize>()
            );
        }
    }

    Ok(models_by_provider)
}

/// Fallback: build a ProviderRegistry from config file + environment variables
async fn fallback_registry() -> Result<ProviderRegistry> {
    let config = crate::config::Config::load().await.unwrap_or_default();
    ProviderRegistry::from_config(&config).await
}

async fn fetch_pending_tasks(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) -> Result<()> {
    tracing::info!("Checking for pending tasks...");

    let mut req = client.get(format!("{}/v1/opencode/tasks?status=pending", server));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;
    // Handle both plain array response and {tasks: [...]} wrapper
    let tasks = if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        data["tasks"].as_array().cloned().unwrap_or_default()
    };

    tracing::info!("Found {} pending task(s)", tasks.len());

    for task in tasks {
        if let Some(id) = task["id"].as_str() {
            let mut proc = processing.lock().await;
            if !proc.contains(id) {
                proc.insert(id.to_string());
                drop(proc);

                let task_id = id.to_string();
                let client = client.clone();
                let server = server.to_string();
                let token = token.clone();
                let worker_id = worker_id.to_string();
                let auto_approve = *auto_approve;
                let processing = processing.clone();

                tokio::spawn(async move {
                    if let Err(e) =
                        handle_task(&client, &server, &token, &worker_id, &task, auto_approve).await
                    {
                        tracing::error!("Task {} failed: {}", task_id, e);
                    }
                    processing.lock().await.remove(&task_id);
                });
            }
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn connect_stream(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) -> Result<()> {
    let url = format!(
        "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
        server,
        urlencoding::encode(name),
        urlencoding::encode(worker_id)
    );

    let mut req = client
        .get(&url)
        .header("Accept", "text/event-stream")
        .header("X-Worker-ID", worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        anyhow::bail!("Failed to connect: {}", res.status());
    }

    tracing::info!("Connected to A2A server");

    let mut stream = res.bytes_stream();
    let mut buffer = String::new();
    let mut poll_interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    poll_interval.tick().await; // consume the initial immediate tick

    loop {
        tokio::select! {
            chunk = stream.next() => {
                match chunk {
                    Some(Ok(chunk)) => {
                        buffer.push_str(&String::from_utf8_lossy(&chunk));

                        // Process SSE events
                        while let Some(pos) = buffer.find("\n\n") {
                            let event_str = buffer[..pos].to_string();
                            buffer = buffer[pos + 2..].to_string();

                            if let Some(data_line) = event_str.lines().find(|l| l.starts_with("data:")) {
                                let data = data_line.trim_start_matches("data:").trim();
                                if data == "[DONE]" || data.is_empty() {
                                    continue;
                                }

                                if let Ok(task) = serde_json::from_str::<serde_json::Value>(data) {
                                    spawn_task_handler(
                                        &task, client, server, token, worker_id,
                                        processing, auto_approve,
                                    ).await;
                                }
                            }
                        }
                    }
                    Some(Err(e)) => {
                        return Err(e.into());
                    }
                    None => {
                        // Stream ended
                        return Ok(());
                    }
                }
            }
            _ = poll_interval.tick() => {
                // Periodic poll for pending tasks the SSE stream may have missed
                if let Err(e) = poll_pending_tasks(
                    client, server, token, worker_id, processing, auto_approve,
                ).await {
                    tracing::warn!("Periodic task poll failed: {}", e);
                }
            }
        }
    }
}

async fn spawn_task_handler(
    task: &serde_json::Value,
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) {
    if let Some(id) = task
        .get("task")
        .and_then(|t| t["id"].as_str())
        .or_else(|| task["id"].as_str())
    {
        let mut proc = processing.lock().await;
        if !proc.contains(id) {
            proc.insert(id.to_string());
            drop(proc);

            let task_id = id.to_string();
            let task = task.clone();
            let client = client.clone();
            let server = server.to_string();
            let token = token.clone();
            let worker_id = worker_id.to_string();
            let auto_approve = *auto_approve;
            let processing_clone = processing.clone();

            tokio::spawn(async move {
                if let Err(e) =
                    handle_task(&client, &server, &token, &worker_id, &task, auto_approve).await
                {
                    tracing::error!("Task {} failed: {}", task_id, e);
                }
                processing_clone.lock().await.remove(&task_id);
            });
        }
    }
}

async fn poll_pending_tasks(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) -> Result<()> {
    let mut req = client.get(format!("{}/v1/opencode/tasks?status=pending", server));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;
    let tasks = if let Some(arr) = data.as_array() {
        arr.clone()
    } else {
        data["tasks"].as_array().cloned().unwrap_or_default()
    };

    if !tasks.is_empty() {
        tracing::debug!("Poll found {} pending task(s)", tasks.len());
    }

    for task in &tasks {
        spawn_task_handler(
            task,
            client,
            server,
            token,
            worker_id,
            processing,
            auto_approve,
        )
        .await;
    }

    Ok(())
}

async fn handle_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task: &serde_json::Value,
    auto_approve: AutoApprove,
) -> Result<()> {
    let task_id = task_str(task, "id").ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task_str(task, "title").unwrap_or("Untitled");

    tracing::info!("Handling task: {} ({})", title, task_id);

    // Claim the task
    let mut req = client
        .post(format!("{}/v1/worker/tasks/claim", server))
        .header("X-Worker-ID", worker_id);
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req
        .json(&serde_json::json!({ "task_id": task_id }))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await?;
        if status == reqwest::StatusCode::CONFLICT {
            tracing::debug!(
                task_id,
                "Task already claimed by another worker, skipping"
            );
        } else {
            tracing::warn!(task_id, %status, "Failed to claim task: {}", text);
        }
        return Ok(());
    }

    tracing::info!("Claimed task: {}", task_id);

    let metadata = task_metadata(task);
    let resume_session_id = metadata
        .get("resume_session_id")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let complexity_hint = metadata_str(&metadata, &["complexity"]);
    let model_tier = metadata_str(&metadata, &["model_tier", "tier"])
        .map(|s| s.to_ascii_lowercase())
        .or_else(|| {
            complexity_hint.as_ref().map(|complexity| {
                match complexity.to_ascii_lowercase().as_str() {
                    "quick" => "fast".to_string(),
                    "deep" => "heavy".to_string(),
                    _ => "balanced".to_string(),
                }
            })
        });
    let worker_personality = metadata_str(
        &metadata,
        &["worker_personality", "personality", "agent_personality"],
    );
    let target_agent_name = metadata_str(&metadata, &["target_agent_name", "agent_name"]);
    let raw_model = task_str(task, "model_ref")
        .or_else(|| metadata_lookup(&metadata, "model_ref").and_then(|v| v.as_str()))
        .or_else(|| task_str(task, "model"))
        .or_else(|| metadata_lookup(&metadata, "model").and_then(|v| v.as_str()));
    let selected_model = raw_model.map(model_ref_to_provider_model);

    // Resume existing session when requested; fall back to a fresh session if missing.
    let mut session = if let Some(ref sid) = resume_session_id {
        match Session::load(sid).await {
            Ok(existing) => {
                tracing::info!("Resuming session {} for task {}", sid, task_id);
                existing
            }
            Err(e) => {
                tracing::warn!(
                    "Could not load session {} for task {} ({}), starting a new session",
                    sid,
                    task_id,
                    e
                );
                Session::new().await?
            }
        }
    } else {
        Session::new().await?
    };

    let agent_type = task_str(task, "agent_type")
        .or_else(|| task_str(task, "agent"))
        .unwrap_or("build");
    session.agent = agent_type.to_string();

    if let Some(model) = selected_model.clone() {
        session.metadata.model = Some(model);
    }

    let prompt = task_str(task, "prompt")
        .or_else(|| task_str(task, "description"))
        .unwrap_or(title);

    tracing::info!("Executing prompt: {}", prompt);

    // Set up output streaming to forward progress to the server
    let stream_client = client.clone();
    let stream_server = server.to_string();
    let stream_token = token.clone();
    let stream_worker_id = worker_id.to_string();
    let stream_task_id = task_id.to_string();

    let output_callback = move |output: String| {
        let c = stream_client.clone();
        let s = stream_server.clone();
        let t = stream_token.clone();
        let w = stream_worker_id.clone();
        let tid = stream_task_id.clone();
        tokio::spawn(async move {
            let mut req = c
                .post(format!("{}/v1/opencode/tasks/{}/output", s, tid))
                .header("X-Worker-ID", &w);
            if let Some(tok) = &t {
                req = req.bearer_auth(tok);
            }
            let _ = req
                .json(&serde_json::json!({
                    "worker_id": w,
                    "output": output,
                }))
                .send()
                .await;
        });
    };

    // Execute swarm tasks via SwarmExecutor; all other agents use the standard session loop.
    let (status, result, error, session_id) = if is_swarm_agent(agent_type) {
        match execute_swarm_with_policy(
            &mut session,
            prompt,
            model_tier.as_deref(),
            selected_model,
            &metadata,
            complexity_hint.as_deref(),
            worker_personality.as_deref(),
            target_agent_name.as_deref(),
            Some(&output_callback),
        )
        .await
        {
            Ok((session_result, true)) => {
                tracing::info!("Swarm task completed successfully: {}", task_id);
                (
                    "completed",
                    Some(session_result.text),
                    None,
                    Some(session_result.session_id),
                )
            }
            Ok((session_result, false)) => {
                tracing::warn!("Swarm task completed with failures: {}", task_id);
                (
                    "failed",
                    Some(session_result.text),
                    Some("Swarm execution completed with failures".to_string()),
                    Some(session_result.session_id),
                )
            }
            Err(e) => {
                tracing::error!("Swarm task failed: {} - {}", task_id, e);
                ("failed", None, Some(format!("Error: {}", e)), None)
            }
        }
    } else {
        match execute_session_with_policy(
            &mut session,
            prompt,
            auto_approve,
            model_tier.as_deref(),
            Some(&output_callback),
        )
        .await
        {
            Ok(session_result) => {
                tracing::info!("Task completed successfully: {}", task_id);
                (
                    "completed",
                    Some(session_result.text),
                    None,
                    Some(session_result.session_id),
                )
            }
            Err(e) => {
                tracing::error!("Task failed: {} - {}", task_id, e);
                ("failed", None, Some(format!("Error: {}", e)), None)
            }
        }
    };

    // Release the task with full details
    let mut req = client
        .post(format!("{}/v1/worker/tasks/release", server))
        .header("X-Worker-ID", worker_id);
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    req.json(&serde_json::json!({
        "task_id": task_id,
        "status": status,
        "result": result,
        "error": error,
        "session_id": session_id.unwrap_or_else(|| session.id.clone()),
    }))
    .send()
    .await?;

    tracing::info!("Task released: {} with status: {}", task_id, status);
    Ok(())
}

async fn execute_swarm_with_policy<F>(
    session: &mut Session,
    prompt: &str,
    model_tier: Option<&str>,
    explicit_model: Option<String>,
    metadata: &serde_json::Map<String, serde_json::Value>,
    complexity_hint: Option<&str>,
    worker_personality: Option<&str>,
    target_agent_name: Option<&str>,
    output_callback: Option<&F>,
) -> Result<(crate::session::SessionResult, bool)>
where
    F: Fn(String),
{
    use crate::provider::{ContentPart, Message, Role};

    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });

    if session.title.is_none() {
        session.generate_title().await?;
    }

    let strategy = parse_swarm_strategy(metadata);
    let max_subagents = metadata_usize(
        metadata,
        &["swarm_max_subagents", "max_subagents", "subagents"],
    )
    .unwrap_or(10)
    .clamp(1, 100);
    let max_steps_per_subagent = metadata_usize(
        metadata,
        &[
            "swarm_max_steps_per_subagent",
            "max_steps_per_subagent",
            "max_steps",
        ],
    )
    .unwrap_or(50)
    .clamp(1, 200);
    let timeout_secs = metadata_u64(metadata, &["swarm_timeout_secs", "timeout_secs", "timeout"])
        .unwrap_or(600)
        .clamp(30, 3600);
    let parallel_enabled =
        metadata_bool(metadata, &["swarm_parallel_enabled", "parallel_enabled"]).unwrap_or(true);

    let model = resolve_swarm_model(explicit_model, model_tier).await;
    if let Some(ref selected_model) = model {
        session.metadata.model = Some(selected_model.clone());
    }

    if let Some(cb) = output_callback {
        cb(format!(
            "[swarm] routing complexity={} tier={} personality={} target_agent={}",
            complexity_hint.unwrap_or("standard"),
            model_tier.unwrap_or("balanced"),
            worker_personality.unwrap_or("auto"),
            target_agent_name.unwrap_or("auto")
        ));
        cb(format!(
            "[swarm] config strategy={:?} max_subagents={} max_steps={} timeout={}s tier={}",
            strategy,
            max_subagents,
            max_steps_per_subagent,
            timeout_secs,
            model_tier.unwrap_or("balanced")
        ));
    }

    let swarm_config = SwarmConfig {
        max_subagents,
        max_steps_per_subagent,
        subagent_timeout_secs: timeout_secs,
        parallel_enabled,
        model,
        working_dir: session
            .metadata
            .directory
            .as_ref()
            .map(|p| p.to_string_lossy().to_string()),
        ..Default::default()
    };

    let swarm_result = if output_callback.is_some() {
        let (event_tx, mut event_rx) = mpsc::channel(256);
        let executor = SwarmExecutor::new(swarm_config).with_event_tx(event_tx);
        let prompt_owned = prompt.to_string();
        let mut exec_handle =
            tokio::spawn(async move { executor.execute(&prompt_owned, strategy).await });

        let mut final_result: Option<crate::swarm::SwarmResult> = None;

        while final_result.is_none() {
            tokio::select! {
                maybe_event = event_rx.recv() => {
                    if let Some(event) = maybe_event {
                        if let Some(cb) = output_callback {
                            if let Some(line) = format_swarm_event_for_output(&event) {
                                cb(line);
                            }
                        }
                    }
                }
                join_result = &mut exec_handle => {
                    let joined = join_result.map_err(|e| anyhow::anyhow!("Swarm join failure: {}", e))?;
                    final_result = Some(joined?);
                }
            }
        }

        while let Ok(event) = event_rx.try_recv() {
            if let Some(cb) = output_callback {
                if let Some(line) = format_swarm_event_for_output(&event) {
                    cb(line);
                }
            }
        }

        final_result.ok_or_else(|| anyhow::anyhow!("Swarm execution returned no result"))?
    } else {
        SwarmExecutor::new(swarm_config)
            .execute(prompt, strategy)
            .await?
    };

    let final_text = if swarm_result.result.trim().is_empty() {
        if swarm_result.success {
            "Swarm completed without textual output.".to_string()
        } else {
            "Swarm finished with failures and no textual output.".to_string()
        }
    } else {
        swarm_result.result.clone()
    };

    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: final_text.clone(),
        }],
    });
    session.save().await?;

    Ok((
        crate::session::SessionResult {
            text: final_text,
            session_id: session.id.clone(),
        },
        swarm_result.success,
    ))
}

/// Execute a session with the given auto-approve policy
/// Optionally streams output chunks via the callback
async fn execute_session_with_policy<F>(
    session: &mut Session,
    prompt: &str,
    auto_approve: AutoApprove,
    model_tier: Option<&str>,
    output_callback: Option<&F>,
) -> Result<crate::session::SessionResult>
where
    F: Fn(String),
{
    use crate::provider::{
        CompletionRequest, ContentPart, Message, ProviderRegistry, Role, parse_model_string,
    };
    use std::sync::Arc;

    // Load provider registry from Vault
    let registry = ProviderRegistry::from_vault().await?;
    let providers = registry.list();
    tracing::info!("Available providers: {:?}", providers);

    if providers.is_empty() {
        anyhow::bail!("No providers available. Configure API keys in HashiCorp Vault.");
    }

    // Parse model string
    let (provider_name, model_id) = if let Some(ref model_str) = session.metadata.model {
        let (prov, model) = parse_model_string(model_str);
        if prov.is_some() {
            (prov.map(|s| s.to_string()), model.to_string())
        } else if providers.contains(&model) {
            (Some(model.to_string()), String::new())
        } else {
            (None, model.to_string())
        }
    } else {
        (None, String::new())
    };

    let provider_slice = providers.as_slice();
    let provider_requested_but_unavailable = provider_name
        .as_deref()
        .map(|p| !providers.contains(&p))
        .unwrap_or(false);

    // Determine which provider to use, preferring explicit request first, then model tier.
    let selected_provider = provider_name
        .as_deref()
        .filter(|p| providers.contains(p))
        .unwrap_or_else(|| choose_provider_for_tier(provider_slice, model_tier));

    let provider = registry
        .get(selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

    // Add user message
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: prompt.to_string(),
        }],
    });

    // Generate title
    if session.title.is_none() {
        session.generate_title().await?;
    }

    // Determine model. If a specific provider was requested but not available,
    // ignore that model id and fall back to the tier-based default model.
    let model = if !model_id.is_empty() && !provider_requested_but_unavailable {
        model_id
    } else {
        default_model_for_provider(selected_provider, model_tier)
    };

    // Create tool registry with filtering based on auto-approve policy
    let tool_registry =
        create_filtered_registry(Arc::clone(&provider), model.clone(), auto_approve);
    let tool_definitions = tool_registry.definitions();

    let temperature = if model.starts_with("kimi-k2") {
        Some(1.0)
    } else {
        Some(0.7)
    };

    tracing::info!(
        "Using model: {} via provider: {} (tier: {:?})",
        model,
        selected_provider,
        model_tier
    );
    tracing::info!(
        "Available tools: {} (auto_approve: {:?})",
        tool_definitions.len(),
        auto_approve
    );

    // Build system prompt
    let cwd = std::env::var("PWD")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

    let mut final_output = String::new();
    let max_steps = 50;

    for step in 1..=max_steps {
        tracing::info!(step = step, "Agent step starting");

        // Build messages with system prompt first
        let mut messages = vec![Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: system_prompt.clone(),
            }],
        }];
        messages.extend(session.messages.clone());

        let request = CompletionRequest {
            messages,
            tools: tool_definitions.clone(),
            model: model.clone(),
            temperature,
            top_p: None,
            max_tokens: Some(8192),
            stop: Vec::new(),
        };

        let response = provider.complete(request).await?;

        crate::telemetry::TOKEN_USAGE.record_model_usage(
            &model,
            response.usage.prompt_tokens as u64,
            response.usage.completion_tokens as u64,
        );

        // Extract tool calls
        let tool_calls: Vec<(String, String, serde_json::Value)> = response
            .message
            .content
            .iter()
            .filter_map(|part| {
                if let ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                } = part
                {
                    let args: serde_json::Value =
                        serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                    Some((id.clone(), name.clone(), args))
                } else {
                    None
                }
            })
            .collect();

        // Collect text output and stream it
        for part in &response.message.content {
            if let ContentPart::Text { text } = part {
                if !text.is_empty() {
                    final_output.push_str(text);
                    final_output.push('\n');
                    if let Some(cb) = output_callback {
                        cb(text.clone());
                    }
                }
            }
        }

        // If no tool calls, we're done
        if tool_calls.is_empty() {
            session.add_message(response.message.clone());
            break;
        }

        session.add_message(response.message.clone());

        tracing::info!(
            step = step,
            num_tools = tool_calls.len(),
            "Executing tool calls"
        );

        // Execute each tool call
        for (tool_id, tool_name, tool_input) in tool_calls {
            tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

            // Stream tool start event
            if let Some(cb) = output_callback {
                cb(format!("[tool:start:{}]", tool_name));
            }

            // Check if tool is allowed based on auto-approve policy
            if !is_tool_allowed(&tool_name, auto_approve) {
                let msg = format!(
                    "Tool '{}' requires approval but auto-approve policy is {:?}",
                    tool_name, auto_approve
                );
                tracing::warn!(tool = %tool_name, "Tool blocked by auto-approve policy");
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content: msg,
                    }],
                });
                continue;
            }

            let content = if let Some(tool) = tool_registry.get(&tool_name) {
                let exec_result: Result<crate::tool::ToolResult> =
                    tool.execute(tool_input.clone()).await;
                match exec_result {
                    Ok(result) => {
                        tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                        if let Some(cb) = output_callback {
                            let status = if result.success { "ok" } else { "err" };
                            cb(format!(
                                "[tool:{}:{}] {}",
                                tool_name,
                                status,
                                &result.output[..result.output.len().min(500)]
                            ));
                        }
                        result.output
                    }
                    Err(e) => {
                        tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                        if let Some(cb) = output_callback {
                            cb(format!("[tool:{}:err] {}", tool_name, e));
                        }
                        format!("Error: {}", e)
                    }
                }
            } else {
                tracing::warn!(tool = %tool_name, "Tool not found");
                format!("Error: Unknown tool '{}'", tool_name)
            };

            session.add_message(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: tool_id,
                    content,
                }],
            });
        }
    }

    session.save().await?;

    Ok(crate::session::SessionResult {
        text: final_output.trim().to_string(),
        session_id: session.id.clone(),
    })
}

/// Check if a tool is allowed based on the auto-approve policy
fn is_tool_allowed(tool_name: &str, auto_approve: AutoApprove) -> bool {
    match auto_approve {
        AutoApprove::All => true,
        AutoApprove::Safe | AutoApprove::None => is_safe_tool(tool_name),
    }
}

/// Check if a tool is considered "safe" (read-only)
fn is_safe_tool(tool_name: &str) -> bool {
    let safe_tools = [
        "read",
        "list",
        "glob",
        "grep",
        "codesearch",
        "lsp",
        "webfetch",
        "websearch",
        "todo_read",
        "skill",
    ];
    safe_tools.contains(&tool_name)
}

/// Create a filtered tool registry based on the auto-approve policy
fn create_filtered_registry(
    provider: Arc<dyn crate::provider::Provider>,
    model: String,
    auto_approve: AutoApprove,
) -> crate::tool::ToolRegistry {
    use crate::tool::*;

    let mut registry = ToolRegistry::new();

    // Always add safe tools
    registry.register(Arc::new(file::ReadTool::new()));
    registry.register(Arc::new(file::ListTool::new()));
    registry.register(Arc::new(file::GlobTool::new()));
    registry.register(Arc::new(search::GrepTool::new()));
    registry.register(Arc::new(lsp::LspTool::new()));
    registry.register(Arc::new(webfetch::WebFetchTool::new()));
    registry.register(Arc::new(websearch::WebSearchTool::new()));
    registry.register(Arc::new(codesearch::CodeSearchTool::new()));
    registry.register(Arc::new(todo::TodoReadTool::new()));
    registry.register(Arc::new(skill::SkillTool::new()));

    // Add potentially dangerous tools only if auto_approve is All
    if matches!(auto_approve, AutoApprove::All) {
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(edit::EditTool::new()));
        registry.register(Arc::new(bash::BashTool::new()));
        registry.register(Arc::new(multiedit::MultiEditTool::new()));
        registry.register(Arc::new(patch::ApplyPatchTool::new()));
        registry.register(Arc::new(todo::TodoWriteTool::new()));
        registry.register(Arc::new(task::TaskTool::new()));
        registry.register(Arc::new(plan::PlanEnterTool::new()));
        registry.register(Arc::new(plan::PlanExitTool::new()));
        registry.register(Arc::new(rlm::RlmTool::new()));
        registry.register(Arc::new(ralph::RalphTool::with_provider(provider, model)));
        registry.register(Arc::new(prd::PrdTool::new()));
        registry.register(Arc::new(confirm_edit::ConfirmEditTool::new()));
        registry.register(Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()));
        registry.register(Arc::new(undo::UndoTool));
    }

    registry.register(Arc::new(invalid::InvalidTool::new()));

    registry
}

/// Start the heartbeat background task
/// Returns a JoinHandle that can be used to cancel the heartbeat
fn start_heartbeat(
    client: Client,
    server: String,
    token: Option<String>,
    heartbeat_state: HeartbeatState,
    processing: Arc<Mutex<HashSet<String>>>,
    cognition_config: CognitionHeartbeatConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut consecutive_failures = 0u32;
        const MAX_FAILURES: u32 = 3;
        const HEARTBEAT_INTERVAL_SECS: u64 = 30;
        const COGNITION_RETRY_COOLDOWN_SECS: u64 = 300;
        let mut cognition_payload_disabled_until: Option<Instant> = None;

        let mut interval =
            tokio::time::interval(tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            // Update task count from processing set
            let active_count = processing.lock().await.len();
            heartbeat_state.set_task_count(active_count).await;

            // Determine status based on active tasks
            let status = if active_count > 0 {
                WorkerStatus::Processing
            } else {
                WorkerStatus::Idle
            };
            heartbeat_state.set_status(status).await;

            // Send heartbeat
            let url = format!(
                "{}/v1/opencode/workers/{}/heartbeat",
                server, heartbeat_state.worker_id
            );
            let mut req = client.post(&url);

            if let Some(ref t) = token {
                req = req.bearer_auth(t);
            }

            let status_str = heartbeat_state.status.lock().await.as_str().to_string();
            let base_payload = serde_json::json!({
                "worker_id": &heartbeat_state.worker_id,
                "agent_name": &heartbeat_state.agent_name,
                "status": status_str,
                "active_task_count": active_count,
            });
            let mut payload = base_payload.clone();
            let mut included_cognition_payload = false;
            let cognition_payload_allowed = cognition_payload_disabled_until
                .map(|until| Instant::now() >= until)
                .unwrap_or(true);

            if cognition_config.enabled
                && cognition_payload_allowed
                && let Some(cognition_payload) =
                    fetch_cognition_heartbeat_payload(&client, &cognition_config).await
                && let Some(obj) = payload.as_object_mut()
            {
                obj.insert("cognition".to_string(), cognition_payload);
                included_cognition_payload = true;
            }

            match req.json(&payload).send().await {
                Ok(res) => {
                    if res.status().is_success() {
                        consecutive_failures = 0;
                        tracing::debug!(
                            worker_id = %heartbeat_state.worker_id,
                            status = status_str,
                            active_tasks = active_count,
                            "Heartbeat sent successfully"
                        );
                    } else if included_cognition_payload && res.status().is_client_error() {
                        tracing::warn!(
                            worker_id = %heartbeat_state.worker_id,
                            status = %res.status(),
                            "Heartbeat cognition payload rejected, retrying without cognition payload"
                        );

                        let mut retry_req = client.post(&url);
                        if let Some(ref t) = token {
                            retry_req = retry_req.bearer_auth(t);
                        }

                        match retry_req.json(&base_payload).send().await {
                            Ok(retry_res) if retry_res.status().is_success() => {
                                cognition_payload_disabled_until = Some(
                                    Instant::now()
                                        + Duration::from_secs(COGNITION_RETRY_COOLDOWN_SECS),
                                );
                                consecutive_failures = 0;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    retry_after_secs = COGNITION_RETRY_COOLDOWN_SECS,
                                    "Paused cognition heartbeat payload after schema rejection"
                                );
                            }
                            Ok(retry_res) => {
                                consecutive_failures += 1;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    status = %retry_res.status(),
                                    failures = consecutive_failures,
                                    "Heartbeat failed even after retry without cognition payload"
                                );
                            }
                            Err(e) => {
                                consecutive_failures += 1;
                                tracing::warn!(
                                    worker_id = %heartbeat_state.worker_id,
                                    error = %e,
                                    failures = consecutive_failures,
                                    "Heartbeat retry without cognition payload failed"
                                );
                            }
                        }
                    } else {
                        consecutive_failures += 1;
                        tracing::warn!(
                            worker_id = %heartbeat_state.worker_id,
                            status = %res.status(),
                            failures = consecutive_failures,
                            "Heartbeat failed"
                        );
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    tracing::warn!(
                        worker_id = %heartbeat_state.worker_id,
                        error = %e,
                        failures = consecutive_failures,
                        "Heartbeat request failed"
                    );
                }
            }

            // Log error after 3 consecutive failures but do not terminate
            if consecutive_failures >= MAX_FAILURES {
                tracing::error!(
                    worker_id = %heartbeat_state.worker_id,
                    failures = consecutive_failures,
                    "Heartbeat failed {} consecutive times - worker will continue running and attempt reconnection via SSE loop",
                    MAX_FAILURES
                );
                // Reset counter to avoid spamming error logs
                consecutive_failures = 0;
            }
        }
    })
}

async fn fetch_cognition_heartbeat_payload(
    client: &Client,
    config: &CognitionHeartbeatConfig,
) -> Option<serde_json::Value> {
    let status_url = format!("{}/v1/cognition/status", config.source_base_url);
    let status_res = tokio::time::timeout(
        Duration::from_millis(config.request_timeout_ms),
        client.get(status_url).send(),
    )
    .await
    .ok()?
    .ok()?;

    if !status_res.status().is_success() {
        return None;
    }

    let status: CognitionStatusSnapshot = status_res.json().await.ok()?;
    let mut payload = serde_json::json!({
        "running": status.running,
        "last_tick_at": status.last_tick_at,
        "active_persona_count": status.active_persona_count,
        "events_buffered": status.events_buffered,
        "snapshots_buffered": status.snapshots_buffered,
        "loop_interval_ms": status.loop_interval_ms,
    });

    if config.include_thought_summary {
        let snapshot_url = format!("{}/v1/cognition/snapshots/latest", config.source_base_url);
        let snapshot_res = tokio::time::timeout(
            Duration::from_millis(config.request_timeout_ms),
            client.get(snapshot_url).send(),
        )
        .await
        .ok()
        .and_then(Result::ok);

        if let Some(snapshot_res) = snapshot_res
            && snapshot_res.status().is_success()
            && let Ok(snapshot) = snapshot_res.json::<CognitionLatestSnapshot>().await
            && let Some(obj) = payload.as_object_mut()
        {
            obj.insert(
                "latest_snapshot_at".to_string(),
                serde_json::Value::String(snapshot.generated_at),
            );
            obj.insert(
                "latest_thought".to_string(),
                serde_json::Value::String(trim_for_heartbeat(
                    &snapshot.summary,
                    config.summary_max_chars,
                )),
            );
            if let Some(model) = snapshot
                .metadata
                .get("model")
                .and_then(serde_json::Value::as_str)
            {
                obj.insert(
                    "latest_thought_model".to_string(),
                    serde_json::Value::String(model.to_string()),
                );
            }
            if let Some(source) = snapshot
                .metadata
                .get("source")
                .and_then(serde_json::Value::as_str)
            {
                obj.insert(
                    "latest_thought_source".to_string(),
                    serde_json::Value::String(source.to_string()),
                );
            }
        }
    }

    Some(payload)
}

fn trim_for_heartbeat(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }

    let mut trimmed = String::with_capacity(max_chars + 3);
    for ch in input.chars().take(max_chars) {
        trimmed.push(ch);
    }
    trimmed.push_str("...");
    trimmed.trim().to_string()
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

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}
