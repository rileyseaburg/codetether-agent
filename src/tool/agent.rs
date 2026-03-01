//! Agent Tool - Spawn and communicate with sub-agents
//!
//! Allows the main agent to create specialized sub-agents, send them messages,
//! and receive their responses. Sub-agents maintain conversation history.

use super::{Tool, ToolResult};
use crate::provider::{ContentPart, Message, ProviderRegistry, Role, parse_model_string};
use crate::session::{Session, SessionEvent};
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, OnceCell};

// ============================================================================
// Agent Store (SRP: pure storage operations)
// ============================================================================

#[derive(Clone)]
struct AgentEntry {
    instructions: String,
    session: Session,
}

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

mod store {
    use super::*;

    pub fn insert(name: String, entry: AgentEntry) { AGENT_STORE.write().insert(name, entry); }
    pub fn remove(name: &str) -> Option<AgentEntry> { AGENT_STORE.write().remove(name) }
    pub fn get(name: &str) -> Option<AgentEntry> { AGENT_STORE.read().get(name).cloned() }
    pub fn contains(name: &str) -> bool { AGENT_STORE.read().contains_key(name) }
    pub fn list() -> Vec<(String, String, usize)> {
        AGENT_STORE.read().iter()
            .map(|(n, e)| (n.clone(), e.instructions.clone(), e.session.messages.len()))
            .collect()
    }
    pub fn update_session(name: &str, session: Session) {
        if let Some(e) = AGENT_STORE.write().get_mut(name) { e.session = session; }
    }
    pub fn snapshots() -> Vec<super::AgentSnapshot> {
        AGENT_STORE.read().iter()
            .map(|(n, e)| super::AgentSnapshot { name: n.clone(), instructions: e.instructions.clone(), session: e.session.clone() })
            .collect()
    }
}

// ============================================================================
// Provider Registry (lazy singleton)
// ============================================================================

static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY
        .get_or_try_init(|| async {
            Ok(Arc::new(ProviderRegistry::from_vault().await?))
        })
        .await
        .cloned()
}

// ============================================================================
// Model Policy (SRP: validation only)
// ============================================================================

fn normalize_model(model: &str) -> String { model.trim().to_ascii_lowercase() }

const SUBSCRIPTION_PROVIDERS: &[&str] = &[
    "openai-codex", "github-copilot", "github-copilot-enterprise", "gemini-web", "local_cuda"
];
fn is_subscription_provider(p: &str) -> bool { SUBSCRIPTION_PROVIDERS.contains(&p) }
fn is_free_model_id(id: &str) -> bool {
    let lower = id.to_ascii_lowercase();
    lower.contains(":free") || lower.ends_with("-free")
}

async fn is_free_or_eligible(model: &str, registry: &ProviderRegistry) -> bool {
    let trimmed = model.trim();
    if trimmed.is_empty() { return false; }

    let (provider_name, model_id) = parse_model_string(trimmed);
    if provider_name.is_none() { return is_free_model_id(trimmed); }

    let pn = provider_name.unwrap();
    if is_subscription_provider(&pn.to_ascii_lowercase()) || is_free_model_id(model_id) { return true; }

    let Some(p) = registry.get(pn) else { return false; };
    match p.list_models().await {
        Ok(ms) => ms.into_iter().any(|m| {
            m.id.eq_ignore_ascii_case(model_id)
                && m.input_cost_per_million.unwrap_or(1.0) <= 0.0
                && m.output_cost_per_million.unwrap_or(1.0) <= 0.0
        }),
        Err(_) => false,
    }
}

// ============================================================================
// Agent Session Helpers
// ============================================================================

fn truncate_preview(input: &str, max_chars: usize) -> String {
    if max_chars == 0 { return String::new(); }
    let preview: String = input.chars().take(max_chars).collect();
    if input.chars().nth(max_chars).is_some() { format!("{preview}...") } else { preview }
}

async fn create_agent_session(name: &str, instructions: &str, model: &str) -> Result<Session> {
    let mut session = Session::new().await.context("Failed to create session")?;
    session.agent = name.to_string();
    session.metadata.model = Some(model.to_string());
    let system_msg = format!(
        "You are @{name}, a specialized sub-agent. {instructions}\n\n\
         You have access to all tools. Be thorough, focused, and concise. Complete the task fully."
    );
    session.add_message(Message { role: Role::System, content: vec![ContentPart::Text { text: system_msg }] });
    Ok(session)
}

// ============================================================================
// Action Handlers (each < 50 lines)
// ============================================================================

async fn handle_spawn(params: &Params) -> Result<ToolResult> {
    let name = params.name.as_ref().context("name required for spawn")?;
    let instructions = params.instructions.as_ref().context("instructions required for spawn")?;
    let requested = params.model.as_ref().context("model required for spawn")?;
    let current = params.current_model.clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .context("cannot determine caller model")?;

    // Policy: different model
    if normalize_model(requested) == normalize_model(&current) {
        return Ok(ToolResult::error(format!("Spawn blocked: '{requested}' must differ from '{current}'.")));
    }

    // Policy: free/subscription eligible
    let registry = get_registry().await?;
    if !is_free_or_eligible(requested, &registry).await {
        return Ok(ToolResult::error(format!("Spawn blocked: '{requested}' not free/subscription-eligible.")));
    }

    if store::contains(name) {
        return Ok(ToolResult::error(format!("Agent @{name} exists. Use kill first.")));
    }

    let session = create_agent_session(name, instructions, requested).await?;
    store::insert(name.clone(), AgentEntry { instructions: instructions.clone(), session });
    tracing::info!(agent = %name, "Sub-agent spawned");
    Ok(ToolResult::success(format!("Spawned @{name} on '{requested}': {instructions}")))
}

async fn handle_message(params: &Params) -> Result<ToolResult> {
    let name = params.name.as_ref().context("name required for message")?.clone();
    let message = params.message.as_ref().context("message required")?.clone();

    let session = store::get(&name).map(|e| e.session)
        .context(format!("Agent @{name} not found"))?;

    let registry = get_registry().await?;
    let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
    let mut session_async = session.clone();

    let handle = tokio::spawn(async move {
        session_async.prompt_with_events(&message, tx, registry).await
    });

    let (response, thinking, tools, error) = run_agent_loop(&mut rx, handle).await;
    store::update_session(&name, session);

    // Build response
    let mut output = json!({ "agent": name, "response": response });
    if !thinking.is_empty() { output["thinking"] = json!(thinking); }
    if !tools.is_empty() { output["tool_calls"] = json!(tools); }

    if let Some(err) = error {
        if response.is_empty() {
            return Ok(ToolResult::error(format!("Agent @{name} failed: {err}")));
        }
        output["warning"] = json!(err);
    }

    Ok(ToolResult::success(serde_json::to_string_pretty(&output).unwrap_or(response)))
}

// SRP: event collection only
async fn run_agent_loop(
    rx: &mut mpsc::Receiver<SessionEvent>,
    handle: tokio::task::JoinHandle<Result<crate::session::SessionResult>>,
) -> (String, String, Vec<Value>, Option<String>) {
    let mut response = String::new();
    let mut thinking = String::new();
    let mut tools = Vec::new();
    let mut error = None;
    let mut done = false;
    let start = std::time::Instant::now();
    let max_wait = std::time::Duration::from_secs(300);

    while !done && start.elapsed() < max_wait {
        tokio::task::yield_now().await;

        match tokio::time::timeout(std::time::Duration::from_millis(20), rx.recv()).await {
            Ok(Some(event)) => match event {
                SessionEvent::TextComplete(t) => response.push_str(&t),
                SessionEvent::ThinkingComplete(t) => thinking.push_str(&t),
                SessionEvent::ToolCallComplete { name, output, success } => {
                    tools.push(json!({ "tool": name, "success": success, "output_preview": truncate_preview(&output, 200) }));
                }
                SessionEvent::Error(e) => { response.push_str(&format!("\n[Error: {e}]")); error = Some(e); }
                SessionEvent::Done => done = true,
                _ => {}
            },
            Ok(None) | Err(_) => { if handle.is_finished() { done = true; } }
        }
    }

    // Task cleanup
    if !handle.is_finished() { handle.abort(); error = Some("Agent timed out after 5 minutes".into()); }
    else if let Ok(Err(e)) = handle.await { error = Some(e.to_string()); }

    (response, thinking, tools, error)
}

fn handle_list() -> ToolResult {
    let agents = store::list();
    if agents.is_empty() { return ToolResult::success("No sub-agents spawned. Use action \"spawn\"."); }

    let list: Vec<Value> = agents.into_iter()
        .map(|(name, instructions, msgs)| json!({ "name": name, "instructions": instructions, "messages": msgs }))
        .collect();
    ToolResult::success(serde_json::to_string_pretty(&list).unwrap_or_default())
}

fn handle_kill(name: &str) -> ToolResult {
    match store::remove(name) {
        Some(_) => { tracing::info!(agent = %name, "Sub-agent killed"); ToolResult::success(format!("Removed @{name}")) }
        None => ToolResult::error(format!("Agent @{name} not found")),
    }
}

// ============================================================================
// Tool Implementation
// ============================================================================

#[derive(Clone)]
pub struct AgentSnapshot {
    pub name: String,
    pub instructions: String,
    pub session: Session,
}

pub fn list_agent_snapshots() -> Vec<AgentSnapshot> { store::snapshots() }
pub fn remove_agent(name: &str) -> bool { store::remove(name).is_some() }

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    instructions: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    current_model: Option<String>,
}

pub struct AgentTool;

impl AgentTool {
    pub fn new() -> Self { Self }
}

impl Default for AgentTool {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl Tool for AgentTool {
    fn id(&self) -> &str { "agent" }
    fn name(&self) -> &str { "Sub-Agent" }
    fn description(&self) -> &str { "Spawn and communicate with specialized sub-agents. Actions: spawn, message, list, kill." }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["spawn", "message", "list", "kill"] },
                "name": { "type": "string", "description": "Agent name (required for spawn, message, kill)" },
                "instructions": { "type": "string", "description": "System instructions (required for spawn)" },
                "message": { "type": "string", "description": "Message to send (required for message)" },
                "model": { "type": "string", "description": "Model for spawned agent (required for spawn)" }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "spawn" => handle_spawn(&p).await,
            "message" => handle_message(&p).await,
            "list" => Ok(handle_list()),
            "kill" => {
                let name = p.name.as_ref().context("name required for kill")?;
                Ok(handle_kill(name))
            }
            _ => Ok(ToolResult::error(format!("Unknown action: {}. Valid: spawn, message, list, kill", p.action))),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{is_free_model_id, normalize_model, truncate_preview};
    use crate::provider::ProviderRegistry;

    #[test] fn truncate_short() { assert_eq!(truncate_preview("hi", 10), "hi"); }
    #[test] fn truncate_long() {
        let out = truncate_preview(&"a".repeat(50), 20);
        assert!(out.ends_with("..."));
        assert!(out.starts_with(&"a".repeat(17)));
    }
    #[test] fn normalize_model_test() { assert_eq!(normalize_model("  ABC  "), "abc"); }
    #[test] fn free_model_detection() {
        assert!(is_free_model_id("z-ai:free"));
        assert!(is_free_model_id("model-free"));
        assert!(!is_free_model_id("gpt-4"));
    }

    #[tokio::test]
    async fn included_provider_eligible() {
        let registry = ProviderRegistry::new();
        assert!(super::is_free_or_eligible("openai-codex/gpt-5-mini", &registry).await);
    }
}
