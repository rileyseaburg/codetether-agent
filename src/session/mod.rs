//! Session management
//!
//! Sessions track the conversation history and state for agent interactions.

use crate::agent::ToolUse;
use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use crate::event_stream::ChatEvent;
use crate::event_stream::s3_sink::S3Sink;
use crate::provider::{ContentPart, Message, Role, ToolDefinition, Usage};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter, RoutingContext};
use crate::tool::ToolRegistry;
use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::fs;
use uuid::Uuid;

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

/// An image attachment to include with a message (from clipboard paste, etc.)
#[derive(Debug, Clone)]
pub struct ImageAttachment {
    /// Base64-encoded data URL (e.g., "data:image/png;base64,...")
    pub data_url: String,
    /// MIME type (e.g., "image/png")
    pub mime_type: Option<String>,
}

fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}

fn enrich_tool_input_with_runtime_context(
    tool_input: &Value,
    current_model: Option<&str>,
    session_id: &str,
    agent_name: &str,
) -> Value {
    let mut enriched = tool_input.clone();
    if let Value::Object(ref mut obj) = enriched {
        if let Some(model) = current_model {
            obj.entry("__ct_current_model".to_string())
                .or_insert_with(|| json!(model));
        }
        obj.entry("__ct_session_id".to_string())
            .or_insert_with(|| json!(session_id));
        obj.entry("__ct_agent_name".to_string())
            .or_insert_with(|| json!(agent_name));
    }
    enriched
}

const BUILD_MODE_TOOL_FIRST_NUDGE: &str = "Build mode policy reminder: execute directly. \
Start by calling at least one appropriate tool now (or emit <tool_call> markup for non-native \
tool providers). Do not ask for permission and do not provide a plan-only response.";
const BUILD_MODE_TOOL_FIRST_MAX_RETRIES: u8 = 2;
const MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES: u32 = 5;
const CODESEARCH_THRASH_NUDGE: &str = "Stop brute-force codesearch variant retries. \
You already got repeated \"No matches found\" results. Do not try punctuation/casing/underscore \
variants of the same token again. Either switch to a broader strategy (e.g., inspect likely files \
directly) or conclude the identifier is absent and continue with the best available evidence.";

fn is_build_agent(agent_name: &str) -> bool {
    agent_name.eq_ignore_ascii_case("build")
}

fn extract_text_content(parts: &[ContentPart]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_codesearch_no_match_output(tool_name: &str, success: bool, output: &str) -> bool {
    tool_name == "codesearch"
        && success
        && output
            .to_ascii_lowercase()
            .contains("no matches found for pattern:")
}

fn looks_like_build_execution_request(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let keywords = [
        "fix",
        "patch",
        "implement",
        "add",
        "update",
        "edit",
        "change",
        "refactor",
        "debug",
        "investigate",
        "run",
        "test",
        "build",
        "compile",
        "create",
        "remove",
        "rename",
        "wire",
        "hook up",
    ];
    keywords.iter().any(|k| lower.contains(k))
}

fn is_affirmative_build_followup(text: &str) -> bool {
    let lower = text.trim().to_ascii_lowercase();
    let markers = [
        "yes",
        "yep",
        "yeah",
        "do it",
        "go ahead",
        "proceed",
        "use the edit",
        "use edit",
        "apply it",
        "ship it",
        "fix it",
    ];
    markers
        .iter()
        .any(|m| lower == *m || lower.starts_with(&format!("{m} ")))
}

fn looks_like_proposed_change(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "use this exact block",
        "now uses",
        "apply",
        "replace",
        "patch",
        "edit",
        "change",
        "update",
        "fix",
    ];
    markers.iter().any(|m| lower.contains(m))
}

fn assistant_offered_next_step(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let offer_markers = [
        "if you want",
        "want me to",
        "should i",
        "next i can",
        "i can also",
        "i'm ready to",
        "i am ready to",
    ];
    let action_markers = [
        "patch",
        "add",
        "update",
        "edit",
        "change",
        "fix",
        "implement",
        "style",
        "tighten",
        "apply",
        "refactor",
    ];
    offer_markers.iter().any(|m| lower.contains(m))
        && action_markers.iter().any(|m| lower.contains(m))
}

fn build_request_requires_tool(session_messages: &[Message], workspace_dir: &Path) -> bool {
    let Some(text) = latest_user_text(session_messages) else {
        return false;
    };

    if looks_like_build_execution_request(&text)
        && !extract_candidate_file_paths(&text, workspace_dir, 1).is_empty()
    {
        return true;
    }

    if !is_affirmative_build_followup(&text) {
        return false;
    }

    let mut skipped_latest_user = false;
    for msg in session_messages.iter().rev() {
        let msg_text = extract_text_content(&msg.content);
        if msg_text.trim().is_empty() {
            continue;
        }

        if matches!(msg.role, Role::User) && !skipped_latest_user {
            skipped_latest_user = true;
            continue;
        }

        if matches!(msg.role, Role::Assistant) && assistant_offered_next_step(&msg_text) {
            return true;
        }

        if (looks_like_build_execution_request(&msg_text) || looks_like_proposed_change(&msg_text))
            && !extract_candidate_file_paths(&msg_text, workspace_dir, 1).is_empty()
        {
            return true;
        }
    }

    false
}

fn should_force_build_tool_first_retry(
    agent_name: &str,
    retry_count: u8,
    tool_definitions: &[ToolDefinition],
    session_messages: &[Message],
    workspace_dir: &Path,
    assistant_text: &str,
    has_tool_calls: bool,
) -> bool {
    if retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
        || !is_build_agent(agent_name)
        || tool_definitions.is_empty()
        || has_tool_calls
    {
        return false;
    }

    if assistant_text.trim().is_empty() {
        return false;
    }

    if !build_request_requires_tool(session_messages, workspace_dir) {
        return false;
    }

    true
}

fn choose_default_provider<'a>(providers: &'a [&'a str]) -> Option<&'a str> {
    // Keep Google as an explicit option, but don't default to it first because
    // some environments expose API keys that are not valid for ChatCompletions.
    let preferred = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "minimax",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];
    for name in preferred {
        if let Some(found) = providers.iter().copied().find(|p| *p == name) {
            return Some(found);
        }
    }
    providers.first().copied()
}

fn resolve_provider_for_session_request<'a>(
    providers: &'a [&'a str],
    explicit_provider: Option<&str>,
) -> Result<&'a str> {
    if let Some(explicit) = explicit_provider {
        if let Some(found) = providers.iter().copied().find(|p| *p == explicit) {
            return Ok(found);
        }
        anyhow::bail!(
            "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
            explicit,
            providers.join(", ")
        );
    }

    choose_default_provider(providers).ok_or_else(|| anyhow::anyhow!("No providers available"))
}

fn prefers_temperature_one(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("kimi-k2") || normalized.contains("glm-") || normalized.contains("minimax")
}

/// Return the context window size (in tokens) for known models.
fn context_window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();
    if m.contains("kimi-k2") {
        256_000
    } else if m.contains("glm-5") || m.contains("glm5") {
        200_000
    } else if m.contains("gpt-4o") {
        128_000
    } else if m.contains("gpt-5") {
        256_000
    } else if m.contains("claude") {
        200_000
    } else if m.contains("gemini") {
        1_000_000
    } else if m.contains("minimax") || m.contains("m2.5") {
        256_000
    } else if m.contains("qwen") {
        131_072
    } else {
        128_000 // conservative default
    }
}

fn session_completion_max_tokens() -> usize {
    std::env::var("CODETETHER_SESSION_MAX_TOKENS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(8192)
}

fn estimate_tokens_for_part(part: &ContentPart) -> usize {
    match part {
        ContentPart::Text { text } => RlmChunker::estimate_tokens(text),
        ContentPart::ToolResult { content, .. } => RlmChunker::estimate_tokens(content),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            thought_signature,
            ..
        } => {
            let mut s = String::new();
            s.push_str(id);
            s.push(' ');
            s.push_str(name);
            s.push(' ');
            s.push_str(arguments);
            if let Some(sig) = thought_signature {
                s.push(' ');
                s.push_str(sig);
            }
            RlmChunker::estimate_tokens(&s)
        }
        ContentPart::Thinking { text } => RlmChunker::estimate_tokens(text),
        ContentPart::Image { .. } => {
            // Image parts are encoded/handled provider-side and do not map
            // cleanly to text tokens. Use a conservative fixed estimate.
            2000
        }
        ContentPart::File { path, mime_type } => {
            let mut s = String::new();
            s.push_str(path);
            if let Some(mt) = mime_type {
                s.push(' ');
                s.push_str(mt);
            }
            RlmChunker::estimate_tokens(&s)
        }
    }
}

fn estimate_tokens_for_messages(messages: &[Message]) -> usize {
    messages
        .iter()
        .map(|m| {
            m.content
                .iter()
                .map(estimate_tokens_for_part)
                .sum::<usize>()
        })
        .sum()
}

fn estimate_tokens_for_tools(tools: &[ToolDefinition]) -> usize {
    // Tool definitions are serialized and sent to providers. Count them as part
    // of the prompt budget to avoid unexpected overflows.
    serde_json::to_string(tools)
        .ok()
        .map(|s| RlmChunker::estimate_tokens(&s))
        .unwrap_or(0)
}

fn estimate_request_tokens(
    system_prompt: &str,
    messages: &[Message],
    tools: &[ToolDefinition],
) -> usize {
    RlmChunker::estimate_tokens(system_prompt)
        + estimate_tokens_for_messages(messages)
        + estimate_tokens_for_tools(tools)
}

fn role_label(role: Role) -> &'static str {
    match role {
        Role::System => "System",
        Role::User => "User",
        Role::Assistant => "Assistant",
        Role::Tool => "Tool",
    }
}

/// Build tool-calling instructions to inject into the system prompt for models
/// that don't support native structured tool calls (e.g. gemini-web).
/// Instructs the model to output `<tool_call>` XML blocks with JSON payloads.
fn inject_tool_prompt(base_prompt: &str, tools: &[ToolDefinition]) -> String {
    let tool_lines: String = tools
        .iter()
        .map(|t| format!("- {}: {}", t.name, t.description))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{base_prompt}\n\n\
         # Tool Use\n\
         You have access to the following tools. To use a tool, output a \
         <tool_call> XML block with a JSON object containing \"name\" and \
         \"arguments\" fields. You may output multiple tool calls in one response.\n\n\
         Example:\n\
         <tool_call>\n\
         {{\"name\": \"bash\", \"arguments\": {{\"command\": \"ls /tmp\"}}}}\n\
         </tool_call>\n\n\
         Available tools:\n{tool_lines}\n\n\
         RULES:\n\
         1. To use a tool, output <tool_call> blocks. Do NOT describe or \
         simulate tool usage in plain text. Do NOT fabricate tool output.\n\
         2. After a tool result is returned, review the result and provide \
         your final answer in plain text WITHOUT any <tool_call> blocks. \
         Only call another tool if the result was insufficient.\n\
         3. Do NOT call the same tool with the same arguments more than once.\n\
         4. Prefer the lsp tool for code intelligence (symbols/definitions/references). \
         Prefer the bash tool for shell commands.\n\
         5. During refactors, NEVER create placeholder/stub implementations \
         (e.g., TODO, FIXME, \"not implemented\", \"fallback\"). Always preserve \
         concrete behavior."
    )
}

fn stub_marker_in_text(text: &str) -> Option<&'static str> {
    let lower = text.to_ascii_lowercase();
    let markers = [
        "todo",
        "fixme",
        "placeholder implementation",
        "<placeholder>",
        "[placeholder]",
        "{placeholder}",
        "not implemented",
        "fallback",
        "stub",
        "coming soon",
        "unimplemented!(",
        "todo!(",
        "throw new error(\"not implemented",
    ];
    markers.into_iter().find(|m| lower.contains(m))
}

fn detect_stub_in_tool_input(tool_name: &str, tool_input: &Value) -> Option<String> {
    let check = |label: &str, text: &str| {
        stub_marker_in_text(text).map(|marker| format!("{label} contains stub marker \"{marker}\""))
    };

    match tool_name {
        "write" => tool_input
            .get("content")
            .and_then(Value::as_str)
            .and_then(|text| check("content", text)),
        "edit" | "confirm_edit" => tool_input
            .get("new_string")
            .or_else(|| tool_input.get("newString"))
            .and_then(Value::as_str)
            .and_then(|text| check("new_string", text)),
        "advanced_edit" => tool_input
            .get("newString")
            .and_then(Value::as_str)
            .and_then(|text| check("newString", text)),
        "multiedit" | "confirm_multiedit" => {
            let edits = tool_input.get("edits").and_then(Value::as_array)?;
            for (idx, edit) in edits.iter().enumerate() {
                if let Some(reason) = edit
                    .get("new_string")
                    .or_else(|| edit.get("newString"))
                    .and_then(Value::as_str)
                    .and_then(|text| check(&format!("edits[{idx}].new_string"), text))
                {
                    return Some(reason);
                }
            }
            None
        }
        "patch" => {
            let patch = tool_input.get("patch").and_then(Value::as_str)?;
            let added_lines = patch
                .lines()
                .filter(|line| line.starts_with('+') && !line.starts_with("+++"))
                .map(|line| line.trim_start_matches('+'))
                .collect::<Vec<_>>()
                .join("\n");
            if added_lines.is_empty() {
                None
            } else {
                check("patch additions", &added_lines)
            }
        }
        _ => None,
    }
}

fn latest_user_text(messages: &[Message]) -> Option<String> {
    messages.iter().rev().find_map(|m| {
        if m.role != Role::User {
            return None;
        }
        let text = m
            .content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if text.trim().is_empty() {
            None
        } else {
            Some(text)
        }
    })
}

fn extract_candidate_file_paths(text: &str, cwd: &Path, max_files: usize) -> Vec<String> {
    static FILE_PATH_RE: OnceLock<Regex> = OnceLock::new();
    let re = FILE_PATH_RE.get_or_init(|| {
        Regex::new(
            r"(?x)
            (?P<path>
              (?:\.?/)?(?:[A-Za-z0-9_.\-]+/)*
              [A-Za-z0-9_.\-]+\.
              (?:rs|ts|tsx|js|jsx|mjs|cjs|py|go|java|kt|swift|c|cc|cpp|h|hpp|cs|php|rb|scala|sql|json|ya?ml|toml)
            )",
        )
        .unwrap()
    });

    let mut out = Vec::new();
    for cap in re.captures_iter(text) {
        let Some(raw) = cap.name("path").map(|m| m.as_str()) else {
            continue;
        };
        let path = raw
            .trim_matches(|c: char| {
                matches!(
                    c,
                    '`' | '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':'
                )
            })
            .to_string();
        if path.is_empty() || out.iter().any(|p| p == &path) {
            continue;
        }
        if cwd.join(&path).exists() {
            out.push(path);
        }
        if out.len() >= max_files {
            break;
        }
    }
    out
}

async fn build_proactive_lsp_context_message(
    selected_provider: &str,
    step: usize,
    tool_registry: &ToolRegistry,
    session_messages: &[Message],
    workspace_dir: &Path,
) -> Option<Message> {
    // Keep this narrowly scoped to Gemini Web where the user requested
    // proactive LSP context before tool-calling decisions.
    if selected_provider != "gemini-web" || step != 1 {
        return None;
    }

    let Some(lsp_tool) = tool_registry.get("lsp") else {
        return None;
    };

    let Some(user_text) = latest_user_text(session_messages) else {
        return None;
    };

    let max_files = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_FILES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(1);

    let max_chars = std::env::var("CODETETHER_PROACTIVE_LSP_MAX_CHARS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(600);

    let paths = extract_candidate_file_paths(&user_text, workspace_dir, max_files);
    if paths.is_empty() {
        return None;
    }

    let mut sections: Vec<String> = Vec::new();
    for path in paths {
        let args = json!({
            "action": "documentSymbol",
            "file_path": path,
            "line": 1,
            "column": 1
        });

        match lsp_tool.execute(args).await {
            Ok(result) if result.success => {
                sections.push(format!(
                    "File: {}\n{}",
                    path,
                    truncate_with_ellipsis(&result.output, max_chars)
                ));
            }
            Ok(result) => {
                tracing::debug!(
                    file = %path,
                    output = %truncate_with_ellipsis(&result.output, 200),
                    "Proactive LSP context skipped file due to unsuccessful result"
                );
            }
            Err(e) => {
                tracing::debug!(file = %path, error = %e, "Proactive LSP prefetch failed");
            }
        }
    }

    if sections.is_empty() {
        return None;
    }

    Some(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: format!(
                "Proactive LSP context (prefetched). Use this to reason about file structure before issuing more tools.\n\n{}",
                sections.join("\n\n---\n\n")
            ),
        }],
    })
}

fn messages_to_rlm_context(messages: &[Message]) -> String {
    // Render a lossless-ish text representation of the session suitable for RLM.
    // Avoid embedding base64 image data URLs (they can be enormous and not useful
    // for summarizing conversational state).
    let mut out = String::new();
    for (idx, m) in messages.iter().enumerate() {
        out.push_str(&format!("[{} {}]\n", idx, role_label(m.role)));

        for part in &m.content {
            match part {
                ContentPart::Text { text } => {
                    out.push_str(text);
                    out.push('\n');
                }
                ContentPart::Thinking { text } => {
                    if !text.trim().is_empty() {
                        out.push_str("[Thinking]\n");
                        out.push_str(text);
                        out.push('\n');
                    }
                }
                ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } => {
                    out.push_str(&format!(
                        "[ToolCall id={id} name={name}]\nargs: {arguments}\n"
                    ));
                }
                ContentPart::ToolResult {
                    tool_call_id,
                    content,
                } => {
                    out.push_str(&format!("[ToolResult id={tool_call_id}]\n"));
                    out.push_str(content);
                    out.push('\n');
                }
                ContentPart::Image { mime_type, url } => {
                    out.push_str(&format!(
                        "[Image mime_type={} url_len={}]\n",
                        mime_type.clone().unwrap_or_else(|| "unknown".to_string()),
                        url.len()
                    ));
                }
                ContentPart::File { path, mime_type } => {
                    out.push_str(&format!(
                        "[File path={} mime_type={}]\n",
                        path,
                        mime_type.clone().unwrap_or_else(|| "unknown".to_string())
                    ));
                }
            }
        }

        out.push_str("\n---\n\n");
    }
    out
}

fn is_prompt_too_long_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    msg.contains("prompt is too long")
        || msg.contains("context length")
        || msg.contains("maximum context")
        || (msg.contains("tokens") && msg.contains("maximum") && msg.contains("prompt"))
}

/// A conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub tool_uses: Vec<ToolUse>,
    pub usage: Usage,
    pub agent: String,
    pub metadata: SessionMetadata,
    /// Optional bus for publishing agent thinking/reasoning to training pipeline
    #[serde(skip)]
    pub bus: Option<Arc<crate::bus::AgentBus>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub directory: Option<PathBuf>,
    pub model: Option<String>,
    pub shared: bool,
    pub share_url: Option<String>,
}

impl Session {
    fn default_model_for_provider(provider: &str) -> String {
        match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "minimax" => "MiniMax-M2.5".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "local_cuda" => std::env::var("LOCAL_CUDA_MODEL")
                .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
                .unwrap_or_else(|_| "qwen3-coder-next".to_string()),
            "zhipuai" | "zai" => "glm-5".to_string(),
            // OpenRouter uses model IDs like "z-ai/glm-5".
            "openrouter" => "z-ai/glm-5".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
            _ => "glm-5".to_string(),
        }
    }

    /// Create a new session
    pub async fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        Ok(Self {
            id,
            title: None,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            tool_uses: Vec::new(),
            usage: Usage::default(),
            agent: "build".to_string(),
            metadata: SessionMetadata {
                directory: Some(std::env::current_dir()?),
                ..Default::default()
            },
            bus: None,
        })
    }

    /// Attach a bus for publishing agent thinking/reasoning
    pub fn with_bus(mut self, bus: Arc<crate::bus::AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Load an existing session
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let content = fs::read_to_string(&path).await?;
        let session: Session = serde_json::from_str(&content)?;
        Ok(session)
    }

    /// Load the last session, optionally scoped to a workspace directory
    ///
    /// When `workspace` is Some, only considers sessions created in that directory.
    /// When None, returns the most recent session globally (legacy behavior).
    pub async fn last_for_directory(workspace: Option<&std::path::Path>) -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;

        if !sessions_dir.exists() {
            anyhow::bail!("No sessions found");
        }

        let mut entries: Vec<tokio::fs::DirEntry> = Vec::new();
        let mut read_dir = fs::read_dir(&sessions_dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            entries.push(entry);
        }

        if entries.is_empty() {
            anyhow::bail!("No sessions found");
        }

        // Sort by modification time (most recent first)
        // Use std::fs::metadata since we can't await in sort_by_key
        entries.sort_by_key(|e| {
            std::cmp::Reverse(
                std::fs::metadata(e.path())
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            )
        });

        let canonical_workspace =
            workspace.map(|w| w.canonicalize().unwrap_or_else(|_| w.to_path_buf()));

        for entry in &entries {
            let content: String = fs::read_to_string(entry.path()).await?;
            if let Ok(session) = serde_json::from_str::<Session>(&content) {
                // If workspace scoping requested, filter by directory
                if let Some(ref ws) = canonical_workspace {
                    if let Some(ref dir) = session.metadata.directory {
                        let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
                        if &canonical_dir == ws {
                            return Ok(session);
                        }
                    }
                    continue;
                }
                return Ok(session);
            }
        }

        anyhow::bail!("No sessions found")
    }

    /// Load the last session (global, unscoped — legacy compatibility)
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Save the session to disk
    pub async fn save(&self) -> Result<()> {
        let path = Self::session_path(&self.id)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;

        Ok(())
    }

    /// Add a message to the session
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    async fn compress_history_keep_last(
        &mut self,
        provider: Arc<dyn crate::provider::Provider>,
        model: &str,
        keep_last: usize,
        reason: &str,
    ) -> Result<bool> {
        if self.messages.len() <= keep_last {
            return Ok(false);
        }

        let split_idx = self.messages.len().saturating_sub(keep_last);
        let tail = self.messages.split_off(split_idx);
        let prefix = std::mem::take(&mut self.messages);

        let context = messages_to_rlm_context(&prefix);
        let ctx_window = context_window_for_model(model);

        // We call auto_process directly for session context. It internally
        // compresses very large inputs before hitting the model.
        let rlm_config = RlmConfig::default();
        let auto_ctx = AutoProcessContext {
            tool_id: "session_context",
            tool_args: serde_json::json!({"reason": reason}),
            session_id: &self.id,
            abort: None,
            on_progress: None,
            provider,
            model: model.to_string(),
        };

        let summary = match RlmRouter::auto_process(&context, auto_ctx, &rlm_config).await {
            Ok(result) => {
                tracing::info!(
                    reason,
                    input_tokens = result.stats.input_tokens,
                    output_tokens = result.stats.output_tokens,
                    compression_ratio = result.stats.compression_ratio,
                    "RLM: Compressed session history"
                );
                result.processed
            }
            Err(e) => {
                tracing::warn!(reason, error = %e, "RLM: Failed to compress session history; falling back to chunk compression");
                // Fallback: keep a smaller, semantically chunked excerpt.
                RlmChunker::compress(&context, (ctx_window as f64 * 0.25) as usize, None)
            }
        };

        let summary_msg = Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: format!(
                    "[AUTO CONTEXT COMPRESSION]\nOlder conversation + tool output was compressed to fit the model context window.\n\n{}",
                    summary
                ),
            }],
        };

        let mut new_messages = Vec::with_capacity(1 + tail.len());
        new_messages.push(summary_msg);
        new_messages.extend(tail);
        self.messages = new_messages;
        self.updated_at = Utc::now();

        Ok(true)
    }

    async fn enforce_context_window(
        &mut self,
        provider: Arc<dyn crate::provider::Provider>,
        model: &str,
        system_prompt: &str,
        tools: &[ToolDefinition],
    ) -> Result<()> {
        let ctx_window = context_window_for_model(model);

        // Reserve response tokens + a small fixed overhead for tool schemas,
        // protocol framing, and provider-specific wrappers.
        let reserve = session_completion_max_tokens().saturating_add(2048);
        let budget = ctx_window.saturating_sub(reserve);
        let safety_budget = (budget as f64 * 0.90) as usize;

        // Try progressively more aggressive compression.
        let keep_last_candidates = [16usize, 12, 8, 6];
        for keep_last in keep_last_candidates {
            let est = estimate_request_tokens(system_prompt, &self.messages, tools);
            if est <= safety_budget {
                return Ok(());
            }

            tracing::info!(
                est_tokens = est,
                ctx_window,
                safety_budget,
                keep_last,
                "Context window approaching limit; compressing older session history"
            );

            let did = self
                .compress_history_keep_last(
                    Arc::clone(&provider),
                    model,
                    keep_last,
                    "context_budget",
                )
                .await?;

            if !did {
                // Nothing left to compress.
                break;
            }
        }

        Ok(())
    }

    /// Execute a prompt and get the result
    pub async fn prompt(&mut self, message: &str) -> Result<SessionResult> {
        use crate::provider::{
            CompletionRequest, ContentPart, ProviderRegistry, Role, parse_model_string,
        };

        // Load providers from Vault
        let registry = ProviderRegistry::from_vault().await?;

        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!(
                "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
            );
        }

        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
            if prov.is_some() {
                // Format: provider/model
                (prov.map(|s| s.to_string()), model.to_string())
            } else if providers.contains(&model) {
                // Format: just provider name (e.g., "novita")
                (Some(model.to_string()), String::new())
            } else {
                // Format: just model name
                (None, model.to_string())
            }
        } else {
            (None, String::new())
        };

        let selected_provider =
            resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?;

        let provider = registry
            .get(selected_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

        // Add user message to session using add_message
        self.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: message.to_string(),
            }],
        });

        // Generate title if this is the first user message and no title exists
        if self.title.is_none() {
            self.generate_title().await?;
        }

        // Determine model to use
        let model = if !model_id.is_empty() {
            model_id
        } else {
            Self::default_model_for_provider(selected_provider)
        };

        // Compress oversized user message via RLM if it exceeds the context threshold
        {
            let ctx_window = context_window_for_model(&model);
            let msg_tokens = RlmChunker::estimate_tokens(message);
            let threshold = (ctx_window as f64 * 0.35) as usize;
            if msg_tokens > threshold {
                tracing::info!(
                    msg_tokens,
                    threshold,
                    ctx_window,
                    "RLM: User message exceeds context threshold, compressing"
                );
                let auto_ctx = AutoProcessContext {
                    tool_id: "session_context",
                    tool_args: serde_json::json!({}),
                    session_id: &self.id,
                    abort: None,
                    on_progress: None,
                    provider: Arc::clone(&provider),
                    model: model.clone(),
                };
                let rlm_config = RlmConfig::default();
                match RlmRouter::auto_process(message, auto_ctx, &rlm_config).await {
                    Ok(result) => {
                        tracing::info!(
                            input_tokens = result.stats.input_tokens,
                            output_tokens = result.stats.output_tokens,
                            "RLM: User message compressed"
                        );
                        // Replace the last message (user message we just added)
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text {
                                text: format!(
                                    "[Original message: {} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                                    msg_tokens,
                                    result.processed,
                                    message.chars().take(500).collect::<String>()
                                ),
                            }];
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "RLM: Failed to compress user message, using truncation");
                        let max_chars = threshold * 4;
                        let truncated = RlmChunker::compress(message, max_chars / 4, None);
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text { text: truncated }];
                        }
                    }
                }
            }
        }

        // Create tool registry with all available tools
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        // Some models behave best with temperature=1.0.
        // - Kimi K2.x requires temperature=1.0
        // - GLM (Z.AI) defaults to temperature 1.0 for coding workflows
        // Use contains() to match both short aliases and provider-qualified IDs.
        let temperature = if prefers_temperature_one(&model) {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // Most providers support native tool calling.  For providers that
        // don't (e.g. gemini-web), the FunctionGemma tool-call router will
        // convert text-only responses into structured tool calls.
        let model_supports_tools = !matches!(
            selected_provider,
            "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
        );

        // Build system prompt with AGENTS.md
        let cwd = self
            .metadata
            .directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

        // For models that don't support native tool calling, inject tool
        // definitions into the system prompt so the model outputs <tool_call>
        // XML blocks that the router can parse directly.
        let system_prompt = if !model_supports_tools && !tool_definitions.is_empty() {
            inject_tool_prompt(&system_prompt, &tool_definitions)
        } else {
            system_prompt
        };

        // Run agentic loop with tool execution
        let max_steps = 50;
        let mut final_output = String::new();

        // Track consecutive identical tool calls to detect infinite loops.
        let mut last_tool_sig: Option<String> = None;
        let mut consecutive_same_tool: u32 = 0;
        let mut consecutive_codesearch_no_matches: u32 = 0;
        let mut build_mode_tool_retry_count: u8 = 0;
        const MAX_CONSECUTIVE_SAME_TOOL: u32 = 3;

        // Initialise the FunctionGemma tool-call router (feature-gated, opt-in).

        let tool_router: Option<ToolCallRouter> = {
            let cfg = ToolRouterConfig::from_env();
            match ToolCallRouter::from_config(&cfg) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(error = %e, "FunctionGemma tool router init failed; disabled");
                    None
                }
            }
        };

        for step in 1..=max_steps {
            tracing::info!(step = step, "Agent step starting");

            // Proactively keep the prompt within the model's context window.
            self.enforce_context_window(
                Arc::clone(&provider),
                &model,
                &system_prompt,
                &tool_definitions,
            )
            .await?;

            let proactive_lsp_message = build_proactive_lsp_context_message(
                selected_provider,
                step,
                &tool_registry,
                &self.messages,
                &cwd,
            )
            .await;

            // Call the provider (retry once if the provider rejects due to an
            // unexpected context-length mismatch).
            let mut attempt = 0;
            let response = loop {
                attempt += 1;

                // Build messages with system prompt first
                let mut messages = vec![Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: system_prompt.clone(),
                    }],
                }];
                if let Some(msg) = &proactive_lsp_message {
                    messages.push(msg.clone());
                }
                messages.extend(self.messages.clone());

                // Create completion request with tools
                let request = CompletionRequest {
                    messages,
                    tools: tool_definitions.clone(),
                    model: model.clone(),
                    temperature,
                    top_p: None,
                    max_tokens: Some(session_completion_max_tokens()),
                    stop: Vec::new(),
                };

                match provider.complete(request).await {
                    Ok(r) => break r,
                    Err(e) => {
                        if attempt == 1 && is_prompt_too_long_error(&e) {
                            tracing::warn!(error = %e, "Provider rejected prompt as too long; forcing extra compression and retrying");
                            let _ = self
                                .compress_history_keep_last(
                                    Arc::clone(&provider),
                                    &model,
                                    6,
                                    "prompt_too_long_retry",
                                )
                                .await?;
                            continue;
                        }
                        return Err(e);
                    }
                }
            };

            // Optionally route text-only responses through FunctionGemma to
            // produce structured tool calls.  Skipped when the model natively
            // supports tool calling (which all current providers do).

            let response = if let Some(ref router) = tool_router {
                router
                    .maybe_reformat(response, &tool_definitions, model_supports_tools)
                    .await
            } else {
                response
            };

            // Record token usage
            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &model,
                response.usage.prompt_tokens as u64,
                response.usage.completion_tokens as u64,
            );

            // Extract tool calls from response
            let tool_calls: Vec<(String, String, serde_json::Value)> = response
                .message
                .content
                .iter()
                .filter_map(|part| {
                    if let ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                        ..
                    } = part
                    {
                        // Parse arguments JSON string into Value
                        let args: serde_json::Value =
                            serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                        Some((id.clone(), name.clone(), args))
                    } else {
                        None
                    }
                })
                .collect();

            let assistant_text = extract_text_content(&response.message.content);
            if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
            ) {
                build_mode_tool_retry_count += 1;
                tracing::warn!(
                    step = step,
                    agent = %self.agent,
                    retry = build_mode_tool_retry_count,
                    "Build mode tool-first guard triggered; retrying with execution nudge"
                );
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: BUILD_MODE_TOOL_FIRST_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if !tool_calls.is_empty() {
                build_mode_tool_retry_count = 0;
            } else if is_build_agent(&self.agent)
                && build_request_requires_tool(&self.messages, &cwd)
                && build_mode_tool_retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            {
                return Err(anyhow::anyhow!(
                    "Build mode could not obtain tool calls for an explicit file-change request after {} retries. \
                     Switch to a tool-capable model and try again.",
                    BUILD_MODE_TOOL_FIRST_MAX_RETRIES
                ));
            }

            // Collect text output and publish thinking to bus
            for part in &response.message.content {
                match part {
                    ContentPart::Text { text } if !text.is_empty() => {
                        final_output.push_str(text);
                        final_output.push('\n');
                    }
                    ContentPart::Thinking { text } if !text.is_empty() => {
                        if let Some(ref bus) = self.bus {
                            let handle = bus.handle(&self.agent);
                            handle.send(
                                format!("agent.{}.thinking", self.agent),
                                crate::bus::BusMessage::AgentThinking {
                                    agent_id: self.agent.clone(),
                                    thinking: text.clone(),
                                    step,
                                },
                            );
                        }
                    }
                    _ => {}
                }
            }

            // If no tool calls, we're done
            if tool_calls.is_empty() {
                self.add_message(response.message.clone());
                break;
            }

            // ── Loop detection: break if the same tool+args repeats too many times,
            //    or if a non-native-tool model has used too many consecutive tool steps. ──
            {
                // Signature = sorted list of "name:args" for deterministic comparison.
                let mut sigs: Vec<String> = tool_calls
                    .iter()
                    .map(|(_, name, args)| format!("{name}:{args}"))
                    .collect();
                sigs.sort();
                let sig = sigs.join("|");

                if last_tool_sig.as_deref() == Some(&sig) {
                    consecutive_same_tool += 1;
                } else {
                    consecutive_same_tool = 1;
                    last_tool_sig = Some(sig);
                }

                // For non-native-tool models, also guard against total tool steps.
                // These models tend to always include <tool_call> blocks and never
                // converge to a text-only answer on their own.
                let force_answer = consecutive_same_tool > MAX_CONSECUTIVE_SAME_TOOL
                    || (!model_supports_tools && step >= 3);

                if force_answer {
                    tracing::warn!(
                        step = step,
                        consecutive = consecutive_same_tool,
                        "Breaking agent loop: forcing final answer",
                    );
                    // Strip ToolCall parts from the response so the model
                    // doesn't see dangling calls without results.
                    let mut nudge_msg = response.message.clone();
                    nudge_msg
                        .content
                        .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
                    if !nudge_msg.content.is_empty() {
                        self.add_message(nudge_msg);
                    }
                    self.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: "STOP using tools. Provide your final answer NOW \
                                   in plain text based on the tool results you already \
                                   received. Do NOT output any <tool_call> blocks."
                                .to_string(),
                        }],
                    });
                    continue;
                }
            }

            // Add assistant message with tool calls
            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call
            let mut codesearch_thrash_guard_triggered = false;
            for (tool_id, tool_name, tool_input) in tool_calls {
                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                // Publish tool request to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.request", self.agent),
                        crate::bus::BusMessage::ToolRequest {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            arguments: tool_input.clone(),
                        },
                    );
                }

                if is_interactive_tool(&tool_name) {
                    tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content: "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string(),
                        }],
                    });
                    continue;
                }

                if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input) {
                    tracing::warn!(tool = %tool_name, reason = %reason, "Blocking suspected stubbed edit");
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content: format!(
                                "Error: Refactor guard rejected this edit: {reason}. \
                                 Provide concrete, behavior-preserving implementation (no placeholders/stubs)."
                            ),
                        }],
                    });
                    continue;
                }

                // Get and execute the tool
                let exec_start = std::time::Instant::now();
                let exec_input = enrich_tool_input_with_runtime_context(
                    &tool_input,
                    self.metadata.model.as_deref(),
                    &self.id,
                    &self.agent,
                );
                let content = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(exec_input).await {
                        Ok(result) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        if result.success { AuditOutcome::Success } else { AuditOutcome::Failure },
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "output_len": result.output.len() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            result.output
                        }
                        Err(e) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        AuditOutcome::Failure,
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "error": e.to_string() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            format!("Error: {}", e)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    if let Some(audit) = try_audit_log() {
                        audit
                            .log_with_correlation(
                                AuditCategory::ToolExecution,
                                format!("tool:{}", tool_name),
                                AuditOutcome::Failure,
                                None,
                                Some(json!({ "error": "unknown_tool" })),
                                None,                  // okr_id
                                None,                  // okr_run_id
                                None,                  // relay_id
                                Some(self.id.clone()), // session_id
                            )
                            .await;
                    }
                    format!("Error: Unknown tool '{}'", tool_name)
                };

                // Calculate duration for event stream
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                let success = !content.starts_with("Error:");
                let codesearch_no_match =
                    is_codesearch_no_match_output(&tool_name, success, &content);

                // Publish full tool output to bus for training pipeline
                // (before RLM truncation so we capture the complete output)
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: content.clone(),
                            success,
                            step,
                        },
                    );
                }

                // Emit event stream event for audit/compliance (SOC 2, FedRAMP, ATO)
                if let Some(base_dir) = Self::event_stream_path() {
                    let workspace = std::env::var("PWD")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
                    let event = ChatEvent::tool_result(
                        workspace,
                        self.id.clone(),
                        &tool_name,
                        success,
                        duration_ms,
                        &content,
                        self.messages.len() as u64,
                    );
                    let event_json = event.to_json();
                    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
                    let seq = self.messages.len() as u64;
                    let filename = format!(
                        "{}-chat-events-{:020}-{:020}.jsonl",
                        timestamp,
                        seq * 10000,
                        (seq + 1) * 10000
                    );
                    let event_path = base_dir.join(&self.id).join(filename);

                    let event_path_clone = event_path;
                    tokio::spawn(async move {
                        if let Some(parent) = event_path_clone.parent() {
                            let _ = tokio::fs::create_dir_all(parent).await;
                        }
                        if let Ok(mut file) = tokio::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&event_path_clone)
                            .await
                        {
                            use tokio::io::AsyncWriteExt;
                            let _ = file.write_all(event_json.as_bytes()).await;
                            let _ = file.write_all(b"\n").await;
                        }
                    });
                }

                // Route large tool outputs through RLM
                let content = {
                    let ctx_window = context_window_for_model(&model);
                    let current_tokens = estimate_tokens_for_messages(&self.messages);
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing = RlmRouter::should_route(&content, &routing_ctx, &rlm_config);
                    if routing.should_route {
                        tracing::info!(
                            tool = %tool_name,
                            reason = %routing.reason,
                            estimated_tokens = routing.estimated_tokens,
                            "RLM: Routing large tool output"
                        );
                        let auto_ctx = AutoProcessContext {
                            tool_id: &tool_name,
                            tool_args: tool_input.clone(),
                            session_id: &self.id,
                            abort: None,
                            on_progress: None,
                            provider: Arc::clone(&provider),
                            model: model.clone(),
                        };
                        match RlmRouter::auto_process(&content, auto_ctx, &rlm_config).await {
                            Ok(result) => {
                                tracing::info!(
                                    input_tokens = result.stats.input_tokens,
                                    output_tokens = result.stats.output_tokens,
                                    iterations = result.stats.iterations,
                                    "RLM: Processing complete"
                                );
                                result.processed
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "RLM: auto_process failed, using smart_truncate");
                                let (truncated, _, _) = RlmRouter::smart_truncate(
                                    &content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        content
                    }
                };

                // Add tool result message
                self.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });

                if is_build_agent(&self.agent) {
                    if codesearch_no_match {
                        consecutive_codesearch_no_matches += 1;
                    } else {
                        consecutive_codesearch_no_matches = 0;
                    }

                    if consecutive_codesearch_no_matches >= MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
                        tracing::warn!(
                            step = step,
                            consecutive_codesearch_no_matches = consecutive_codesearch_no_matches,
                            "Detected codesearch no-match thrash; nudging model to stop variant retries",
                        );
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: CODESEARCH_THRASH_NUDGE.to_string(),
                            }],
                        });
                        codesearch_thrash_guard_triggered = true;
                        break;
                    }
                }
            }

            if codesearch_thrash_guard_triggered {
                continue;
            }
        }

        // Save session after each prompt to persist messages
        self.save().await?;

        // Archive event stream to S3/R2 if configured (for compliance: SOC 2, FedRAMP, ATO)
        self.archive_event_stream_to_s3().await;

        Ok(SessionResult {
            text: final_output.trim().to_string(),
            session_id: self.id.clone(),
        })
    }

    /// Archive event stream files to S3/R2 for immutable compliance logging
    async fn archive_event_stream_to_s3(&self) {
        // Check if S3 is configured
        if !S3Sink::is_configured() {
            return;
        }

        let Some(base_dir) = Self::event_stream_path() else {
            return;
        };

        let session_event_dir = base_dir.join(&self.id);
        if !session_event_dir.exists() {
            return;
        }

        // Try to create S3 sink
        let Ok(sink) = S3Sink::from_env().await else {
            tracing::warn!("Failed to create S3 sink for archival");
            return;
        };

        // Upload all event files in the session directory
        let session_id = self.id.clone();
        tokio::spawn(async move {
            if let Ok(mut entries) = tokio::fs::read_dir(&session_event_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                        match sink.upload_file(&path, &session_id).await {
                            Ok(url) => {
                                tracing::info!(url = %url, "Archived event stream to S3/R2");
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "Failed to archive event file to S3");
                            }
                        }
                    }
                }
            }
        });
    }

    /// Process a user message with real-time event streaming for UI updates.
    /// Events are sent through the provided channel as tool calls execute.
    ///
    /// Accepts a pre-loaded `ProviderRegistry` to avoid re-fetching secrets
    /// from Vault on every message (which was the primary TUI performance
    /// bottleneck).
    pub async fn prompt_with_events(
        &mut self,
        message: &str,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: std::sync::Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        self.prompt_with_events_and_images(message, Vec::new(), event_tx, registry)
            .await
    }

    /// Execute a prompt with optional image attachments and stream events.
    ///
    /// This is the full-featured version that supports multimodal input (text + images).
    /// Images should be base64-encoded data URLs.
    pub async fn prompt_with_events_and_images(
        &mut self,
        message: &str,
        images: Vec<ImageAttachment>,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: std::sync::Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        use crate::provider::{CompletionRequest, ContentPart, Role, parse_model_string};

        let _ = event_tx.send(SessionEvent::Thinking).await;

        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!(
                "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
            );
        }
        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
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

        let selected_provider =
            resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?;

        let provider = registry
            .get(selected_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

        // Build user message content parts (text + optional images)
        let mut content_parts = vec![ContentPart::Text {
            text: message.to_string(),
        }];

        // Add image attachments
        for img in &images {
            content_parts.push(ContentPart::Image {
                url: img.data_url.clone(),
                mime_type: img.mime_type.clone(),
            });
        }

        if !images.is_empty() {
            tracing::info!(
                image_count = images.len(),
                "Adding {} image attachment(s) to user message",
                images.len()
            );
        }

        // Add user message
        self.add_message(Message {
            role: Role::User,
            content: content_parts,
        });

        // Generate title if needed
        if self.title.is_none() {
            self.generate_title().await?;
        }

        // Determine model
        let model = if !model_id.is_empty() {
            model_id
        } else {
            Self::default_model_for_provider(selected_provider)
        };

        // Compress oversized user message via RLM if it exceeds the context threshold
        {
            let ctx_window = context_window_for_model(&model);
            let msg_tokens = RlmChunker::estimate_tokens(message);
            let threshold = (ctx_window as f64 * 0.35) as usize;
            if msg_tokens > threshold {
                tracing::info!(
                    msg_tokens,
                    threshold,
                    ctx_window,
                    "RLM: User message exceeds context threshold, compressing"
                );
                let auto_ctx = AutoProcessContext {
                    tool_id: "session_context",
                    tool_args: serde_json::json!({}),
                    session_id: &self.id,
                    abort: None,
                    on_progress: None,
                    provider: Arc::clone(&provider),
                    model: model.clone(),
                };
                let rlm_config = RlmConfig::default();
                match RlmRouter::auto_process(message, auto_ctx, &rlm_config).await {
                    Ok(result) => {
                        tracing::info!(
                            input_tokens = result.stats.input_tokens,
                            output_tokens = result.stats.output_tokens,
                            "RLM: User message compressed"
                        );
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text {
                                text: format!(
                                    "[Original message: {} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                                    msg_tokens,
                                    result.processed,
                                    message.chars().take(500).collect::<String>()
                                ),
                            }];
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "RLM: Failed to compress user message, using truncation");
                        let max_chars = threshold * 4;
                        let truncated = RlmChunker::compress(message, max_chars / 4, None);
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text { text: truncated }];
                        }
                    }
                }
            }
        }

        // Create tool registry
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        let temperature = if prefers_temperature_one(&model) {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // Most providers support native tool calling.  For providers that
        // don't (e.g. gemini-web), the FunctionGemma tool-call router will
        // convert text-only responses into structured tool calls.
        let model_supports_tools = !matches!(
            selected_provider,
            "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
        );

        // Build system prompt
        let cwd = std::env::var("PWD")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
        let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

        // For models that don't support native tool calling, inject tool
        // definitions into the system prompt.
        let system_prompt = if !model_supports_tools && !tool_definitions.is_empty() {
            inject_tool_prompt(&system_prompt, &tool_definitions)
        } else {
            system_prompt
        };

        let mut final_output = String::new();
        let max_steps = 50;

        // Track consecutive identical tool calls to detect infinite loops.
        let mut last_tool_sig: Option<String> = None;
        let mut consecutive_same_tool: u32 = 0;
        let mut consecutive_codesearch_no_matches: u32 = 0;
        let mut build_mode_tool_retry_count: u8 = 0;
        const MAX_CONSECUTIVE_SAME_TOOL: u32 = 3;

        // Initialise the FunctionGemma tool-call router (feature-gated, opt-in).

        let tool_router: Option<ToolCallRouter> = {
            let cfg = ToolRouterConfig::from_env();
            match ToolCallRouter::from_config(&cfg) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(error = %e, "FunctionGemma tool router init failed; disabled");
                    None
                }
            }
        };

        for step in 1..=max_steps {
            tracing::info!(step = step, "Agent step starting");
            let _ = event_tx.send(SessionEvent::Thinking).await;

            // Proactively keep the prompt within the model's context window.
            self.enforce_context_window(
                Arc::clone(&provider),
                &model,
                &system_prompt,
                &tool_definitions,
            )
            .await?;

            let proactive_lsp_message = build_proactive_lsp_context_message(
                selected_provider,
                step,
                &tool_registry,
                &self.messages,
                &cwd,
            )
            .await;

            // Build messages with system prompt first
            let mut messages = vec![Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt.clone(),
                }],
            }];
            if let Some(msg) = &proactive_lsp_message {
                messages.push(msg.clone());
            }
            messages.extend(self.messages.clone());

            let request = CompletionRequest {
                messages,
                tools: tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(session_completion_max_tokens()),
                stop: Vec::new(),
            };

            let llm_start = std::time::Instant::now();
            let mut attempt = 0;
            #[allow(clippy::never_loop)]
            let response = loop {
                attempt += 1;
                match provider.complete(request.clone()).await {
                    Ok(r) => break r,
                    Err(e) => {
                        if attempt == 1 && is_prompt_too_long_error(&e) {
                            tracing::warn!(error = %e, "Provider rejected prompt as too long; forcing extra compression and retrying");
                            let _ = self
                                .compress_history_keep_last(
                                    Arc::clone(&provider),
                                    &model,
                                    6,
                                    "prompt_too_long_retry",
                                )
                                .await?;
                            // Rebuild request with the newly-compressed history.
                            // NOTE: we can't mutate the existing request in place; it contains
                            // cloned message vectors.
                            let mut messages = vec![Message {
                                role: Role::System,
                                content: vec![ContentPart::Text {
                                    text: system_prompt.clone(),
                                }],
                            }];
                            if let Some(msg) = &proactive_lsp_message {
                                messages.push(msg.clone());
                            }
                            messages.extend(self.messages.clone());
                            let new_request = CompletionRequest {
                                messages,
                                tools: tool_definitions.clone(),
                                model: model.clone(),
                                temperature,
                                top_p: None,
                                max_tokens: Some(session_completion_max_tokens()),
                                stop: Vec::new(),
                            };
                            // Shadow request for next attempt.
                            // (We keep the loop structure simple; at most one retry.)
                            match provider.complete(new_request).await {
                                Ok(r2) => break r2,
                                Err(e2) => return Err(e2),
                            }
                        }
                        return Err(e);
                    }
                }
            };
            let llm_duration_ms = llm_start.elapsed().as_millis() as u64;

            // Optionally route text-only responses through FunctionGemma to
            // produce structured tool calls.  Skipped for native tool-calling models.

            let response = if let Some(ref router) = tool_router {
                router
                    .maybe_reformat(response, &tool_definitions, model_supports_tools)
                    .await
            } else {
                response
            };

            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &model,
                response.usage.prompt_tokens as u64,
                response.usage.completion_tokens as u64,
            );

            // Emit usage report for TUI display
            let _ = event_tx
                .send(SessionEvent::UsageReport {
                    prompt_tokens: response.usage.prompt_tokens,
                    completion_tokens: response.usage.completion_tokens,
                    duration_ms: llm_duration_ms,
                    model: model.clone(),
                })
                .await;

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
                        ..
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

            let assistant_text = extract_text_content(&response.message.content);
            if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
            ) {
                build_mode_tool_retry_count += 1;
                tracing::warn!(
                    step = step,
                    agent = %self.agent,
                    retry = build_mode_tool_retry_count,
                    "Build mode tool-first guard triggered; retrying with execution nudge"
                );
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: BUILD_MODE_TOOL_FIRST_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if !tool_calls.is_empty() {
                build_mode_tool_retry_count = 0;
            } else if is_build_agent(&self.agent)
                && build_request_requires_tool(&self.messages, &cwd)
                && build_mode_tool_retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            {
                return Err(anyhow::anyhow!(
                    "Build mode could not obtain tool calls for an explicit file-change request after {} retries. \
                     Switch to a tool-capable model and try again.",
                    BUILD_MODE_TOOL_FIRST_MAX_RETRIES
                ));
            }

            // Collect text output for this step
            // Collect thinking and text output
            let mut thinking_text = String::new();
            let mut step_text = String::new();
            for part in &response.message.content {
                match part {
                    ContentPart::Thinking { text } => {
                        if !text.is_empty() {
                            thinking_text.push_str(text);
                            thinking_text.push('\n');
                        }
                    }
                    ContentPart::Text { text } => {
                        if !text.is_empty() {
                            step_text.push_str(text);
                            step_text.push('\n');
                        }
                    }
                    _ => {}
                }
            }

            // Emit thinking output first
            if !thinking_text.trim().is_empty() {
                let _ = event_tx
                    .send(SessionEvent::ThinkingComplete(
                        thinking_text.trim().to_string(),
                    ))
                    .await;
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.thinking", self.agent),
                        crate::bus::BusMessage::AgentThinking {
                            agent_id: self.agent.clone(),
                            thinking: thinking_text.trim().to_string(),
                            step,
                        },
                    );
                }
            }

            // Emit this step's text BEFORE tool calls so it appears in correct
            // chronological order in the TUI chat display.
            if !step_text.trim().is_empty() {
                let trimmed = step_text.trim().to_string();
                let _ = event_tx
                    .send(SessionEvent::TextChunk(trimmed.clone()))
                    .await;
                let _ = event_tx.send(SessionEvent::TextComplete(trimmed)).await;
                final_output.push_str(&step_text);
            }

            if tool_calls.is_empty() {
                self.add_message(response.message.clone());
                break;
            }

            // ── Loop detection: break if the same tool+args repeats too many times,
            //    or if a non-native-tool model has used too many consecutive tool steps. ──
            {
                let mut sigs: Vec<String> = tool_calls
                    .iter()
                    .map(|(_, name, args)| format!("{name}:{args}"))
                    .collect();
                sigs.sort();
                let sig = sigs.join("|");

                if last_tool_sig.as_deref() == Some(&sig) {
                    consecutive_same_tool += 1;
                } else {
                    consecutive_same_tool = 1;
                    last_tool_sig = Some(sig);
                }

                let force_answer = consecutive_same_tool > MAX_CONSECUTIVE_SAME_TOOL
                    || (!model_supports_tools && step >= 3);

                if force_answer {
                    tracing::warn!(
                        step = step,
                        consecutive = consecutive_same_tool,
                        "Breaking agent loop: forcing final answer",
                    );
                    let mut nudge_msg = response.message.clone();
                    nudge_msg
                        .content
                        .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
                    if !nudge_msg.content.is_empty() {
                        self.add_message(nudge_msg);
                    }
                    self.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: "STOP using tools. Provide your final answer NOW \
                                   in plain text based on the tool results you already \
                                   received. Do NOT output any <tool_call> blocks."
                                .to_string(),
                        }],
                    });
                    continue;
                }
            }

            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call with events
            let mut codesearch_thrash_guard_triggered = false;
            for (tool_id, tool_name, tool_input) in tool_calls {
                let args_str = serde_json::to_string(&tool_input).unwrap_or_default();
                let _ = event_tx
                    .send(SessionEvent::ToolCallStart {
                        name: tool_name.clone(),
                        arguments: args_str,
                    })
                    .await;

                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                // Publish tool request to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.request", self.agent),
                        crate::bus::BusMessage::ToolRequest {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            arguments: tool_input.clone(),
                        },
                    );
                }

                if is_interactive_tool(&tool_name) {
                    tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                    let content = "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string();
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: content.clone(),
                            success: false,
                        })
                        .await;
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content,
                        }],
                    });
                    continue;
                }

                if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input) {
                    tracing::warn!(tool = %tool_name, reason = %reason, "Blocking suspected stubbed edit");
                    let content = format!(
                        "Error: Refactor guard rejected this edit: {reason}. \
                         Provide concrete, behavior-preserving implementation (no placeholders/stubs)."
                    );
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: content.clone(),
                            success: false,
                        })
                        .await;
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content,
                        }],
                    });
                    continue;
                }

                let exec_start = std::time::Instant::now();
                let exec_input = enrich_tool_input_with_runtime_context(
                    &tool_input,
                    self.metadata.model.as_deref(),
                    &self.id,
                    &self.agent,
                );
                let (content, success) = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(exec_input).await {
                        Ok(result) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        if result.success { AuditOutcome::Success } else { AuditOutcome::Failure },
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "output_len": result.output.len() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            (result.output, result.success)
                        }
                        Err(e) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        AuditOutcome::Failure,
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "error": e.to_string() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            (format!("Error: {}", e), false)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    if let Some(audit) = try_audit_log() {
                        audit
                            .log_with_correlation(
                                AuditCategory::ToolExecution,
                                format!("tool:{}", tool_name),
                                AuditOutcome::Failure,
                                None,
                                Some(json!({ "error": "unknown_tool" })),
                                None,                  // okr_id
                                None,                  // okr_run_id
                                None,                  // relay_id
                                Some(self.id.clone()), // session_id
                            )
                            .await;
                    }
                    (format!("Error: Unknown tool '{}'", tool_name), false)
                };

                // Calculate total duration from exec_start (captured from line 772)
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                let codesearch_no_match =
                    is_codesearch_no_match_output(&tool_name, success, &content);

                // Publish full tool output to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: content.clone(),
                            success,
                            step,
                        },
                    );
                }

                // Emit event stream event for audit/compliance (SOC 2, FedRAMP, ATO)
                // This creates the structured JSONL record with byte-range offsets
                // File format: {timestamp}-chat-events-{start_byte}-{end_byte}.jsonl
                if let Some(base_dir) = Self::event_stream_path() {
                    let workspace = std::env::var("PWD")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
                    let event = ChatEvent::tool_result(
                        workspace,
                        self.id.clone(),
                        &tool_name,
                        success,
                        duration_ms,
                        &content,
                        self.messages.len() as u64,
                    );
                    let event_json = event.to_json();
                    let event_size = event_json.len() as u64 + 1; // +1 for newline

                    // Generate filename with byte-range offsets for random access replay
                    // Format: {timestamp}-chat-events-{start_offset}-{end_offset}.jsonl
                    // We use a session-scoped counter stored in metadata for byte tracking
                    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
                    let seq = self.messages.len() as u64;
                    let filename = format!(
                        "{}-chat-events-{:020}-{:020}.jsonl",
                        timestamp,
                        seq * 10000,       // approximated start offset
                        (seq + 1) * 10000  // approximated end offset
                    );
                    let event_path = base_dir.join(&self.id).join(filename);

                    // Fire-and-forget: don't block tool execution on event logging
                    let event_path_clone = event_path;
                    tokio::spawn(async move {
                        if let Some(parent) = event_path_clone.parent() {
                            let _ = tokio::fs::create_dir_all(parent).await;
                        }
                        if let Ok(mut file) = tokio::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&event_path_clone)
                            .await
                        {
                            use tokio::io::AsyncWriteExt;
                            let _ = file.write_all(event_json.as_bytes()).await;
                            let _ = file.write_all(b"\n").await;
                            tracing::debug!(path = %event_path_clone.display(), size = event_size, "Event stream wrote");
                        }
                    });
                }

                let _ = event_tx
                    .send(SessionEvent::ToolCallComplete {
                        name: tool_name.clone(),
                        output: content.clone(),
                        success,
                    })
                    .await;

                // Route large tool outputs through RLM
                let content = {
                    let ctx_window = context_window_for_model(&model);
                    let current_tokens = estimate_tokens_for_messages(&self.messages);
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing = RlmRouter::should_route(&content, &routing_ctx, &rlm_config);
                    if routing.should_route {
                        tracing::info!(
                            tool = %tool_name,
                            reason = %routing.reason,
                            estimated_tokens = routing.estimated_tokens,
                            "RLM: Routing large tool output"
                        );
                        let auto_ctx = AutoProcessContext {
                            tool_id: &tool_name,
                            tool_args: tool_input.clone(),
                            session_id: &self.id,
                            abort: None,
                            on_progress: None,
                            provider: Arc::clone(&provider),
                            model: model.clone(),
                        };
                        match RlmRouter::auto_process(&content, auto_ctx, &rlm_config).await {
                            Ok(result) => {
                                tracing::info!(
                                    input_tokens = result.stats.input_tokens,
                                    output_tokens = result.stats.output_tokens,
                                    iterations = result.stats.iterations,
                                    "RLM: Processing complete"
                                );
                                result.processed
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "RLM: auto_process failed, using smart_truncate");
                                let (truncated, _, _) = RlmRouter::smart_truncate(
                                    &content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        content
                    }
                };

                self.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });

                if is_build_agent(&self.agent) {
                    if codesearch_no_match {
                        consecutive_codesearch_no_matches += 1;
                    } else {
                        consecutive_codesearch_no_matches = 0;
                    }

                    if consecutive_codesearch_no_matches >= MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
                        tracing::warn!(
                            step = step,
                            consecutive_codesearch_no_matches = consecutive_codesearch_no_matches,
                            "Detected codesearch no-match thrash; nudging model to stop variant retries",
                        );
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: CODESEARCH_THRASH_NUDGE.to_string(),
                            }],
                        });
                        codesearch_thrash_guard_triggered = true;
                        break;
                    }
                }
            }

            if codesearch_thrash_guard_triggered {
                continue;
            }
        }

        self.save().await?;

        // Archive event stream to S3/R2 if configured (for compliance: SOC 2, FedRAMP, ATO)
        self.archive_event_stream_to_s3().await;

        // Text was already sent per-step via TextComplete events.
        // Send updated session state so the caller can sync back.
        let _ = event_tx.send(SessionEvent::SessionSync(self.clone())).await;
        let _ = event_tx.send(SessionEvent::Done).await;

        Ok(SessionResult {
            text: final_output.trim().to_string(),
            session_id: self.id.clone(),
        })
    }

    /// Generate a title for the session based on the first message
    /// Only sets title if not already set (for initial title generation)
    pub async fn generate_title(&mut self) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }

        // Get first user message
        let first_message = self
            .messages
            .iter()
            .find(|m| m.role == crate::provider::Role::User);

        if let Some(msg) = first_message {
            let text: String = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    crate::provider::ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");

            // Truncate to reasonable length
            self.title = Some(truncate_with_ellipsis(&text, 47));
        }

        Ok(())
    }

    /// Regenerate the title based on the first message, even if already set
    /// Use this for on-demand title updates or after context changes
    pub async fn regenerate_title(&mut self) -> Result<()> {
        // Get first user message
        let first_message = self
            .messages
            .iter()
            .find(|m| m.role == crate::provider::Role::User);

        if let Some(msg) = first_message {
            let text: String = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    crate::provider::ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");

            // Truncate to reasonable length
            self.title = Some(truncate_with_ellipsis(&text, 47));
        }

        Ok(())
    }

    /// Set a custom title for the session
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
        self.updated_at = Utc::now();
    }

    /// Clear the title, allowing it to be regenerated
    pub fn clear_title(&mut self) {
        self.title = None;
        self.updated_at = Utc::now();
    }

    /// Handle context change - updates metadata and optionally regenerates title
    /// Call this when the session context changes (e.g., directory change, model change)
    pub async fn on_context_change(&mut self, regenerate_title: bool) -> Result<()> {
        self.updated_at = Utc::now();

        if regenerate_title {
            self.regenerate_title().await?;
        }

        Ok(())
    }

    /// Delete a session by ID
    pub async fn delete(id: &str) -> Result<()> {
        let path = Self::session_path(id)?;
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
        }
        Ok(())
    }

    /// Get the sessions directory
    fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|d| d.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }

    /// Get the path for a session file
    fn session_path(id: &str) -> Result<PathBuf> {
        Ok(Self::sessions_dir()?.join(format!("{}.json", id)))
    }

    /// Get the event stream path from environment, if configured.
    /// Returns None if CODETETHER_EVENT_STREAM_PATH is not set.
    fn event_stream_path() -> Option<PathBuf> {
        std::env::var("CODETETHER_EVENT_STREAM_PATH")
            .ok()
            .map(PathBuf::from)
    }
}

/// Result from a session prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub text: String,
    pub session_id: String,
}

/// Events emitted during session processing for real-time UI updates
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// Agent is thinking/processing
    Thinking,
    /// Tool call started
    ToolCallStart { name: String, arguments: String },
    /// Tool call completed with result
    ToolCallComplete {
        name: String,
        output: String,
        success: bool,
    },
    /// Partial text output (for streaming)
    TextChunk(String),
    /// Final text output
    TextComplete(String),
    /// Model thinking/reasoning output
    ThinkingComplete(String),
    /// Token usage for one LLM round-trip
    UsageReport {
        prompt_tokens: usize,
        completion_tokens: usize,
        duration_ms: u64,
        model: String,
    },
    /// Updated session state for caller to sync back
    SessionSync(Session),
    /// Processing complete
    Done,
    /// Error occurred
    Error(String),
}

/// List all sessions
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    let sessions_dir = crate::config::Config::data_dir()
        .map(|d| d.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;

    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false)
            && let Ok(content) = fs::read_to_string(&path).await
            && let Ok(session) = serde_json::from_str::<Session>(&content)
        {
            summaries.push(SessionSummary {
                id: session.id,
                title: session.title,
                created_at: session.created_at,
                updated_at: session.updated_at,
                message_count: session.messages.len(),
                agent: session.agent,
                directory: session.metadata.directory,
            });
        }
    }

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

/// List sessions scoped to a specific directory (workspace)
///
/// Only returns sessions whose `metadata.directory` matches the given path.
/// This prevents sessions from other workspaces "leaking" into the TUI.
pub async fn list_sessions_for_directory(dir: &std::path::Path) -> Result<Vec<SessionSummary>> {
    let all = list_sessions().await?;
    let canonical = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    Ok(all
        .into_iter()
        .filter(|s| {
            s.directory
                .as_ref()
                .map(|d| d.canonicalize().unwrap_or_else(|_| d.clone()) == canonical)
                .unwrap_or(false)
        })
        .collect())
}

/// List sessions for a directory with pagination.
///
/// - `limit`: Maximum number of sessions to return (default: 100)
/// - `offset`: Number of sessions to skip (default: 0)
pub async fn list_sessions_paged(
    dir: &std::path::Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    let mut sessions = list_sessions_for_directory(dir).await?;
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions.into_iter().skip(offset).take(limit).collect())
}

/// Summary of a session for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub agent: String,
    /// The working directory this session was created in
    #[serde(default)]
    pub directory: Option<PathBuf>,
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}

// Async helper for Vec - kept for potential future use
#[allow(dead_code)]
use futures::StreamExt;

#[allow(dead_code)]
trait AsyncCollect<T> {
    async fn collect(self) -> Vec<T>;
}

#[allow(dead_code)]
impl<S, T> AsyncCollect<T> for S
where
    S: futures::Stream<Item = T> + Unpin,
{
    async fn collect(mut self) -> Vec<T> {
        let mut items = Vec::new();
        while let Some(item) = self.next().await {
            items.push(item);
        }
        items
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_request_requires_tool, choose_default_provider, detect_stub_in_tool_input,
        extract_candidate_file_paths, is_codesearch_no_match_output,
        looks_like_build_execution_request, resolve_provider_for_session_request,
        should_force_build_tool_first_retry,
    };
    use crate::provider::{ContentPart, Message, Role, ToolDefinition};
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn explicit_provider_must_not_fallback_when_unavailable() {
        let providers = vec!["zai", "openai"];
        let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
        assert!(result.is_err());
        assert!(
            result
                .err()
                .map(|e| e.to_string())
                .unwrap_or_default()
                .contains("selected explicitly but is unavailable")
        );
    }

    #[test]
    fn explicit_provider_is_used_when_available() {
        let providers = vec!["local_cuda", "zai"];
        let result = resolve_provider_for_session_request(&providers, Some("local_cuda"));
        assert_eq!(result.ok(), Some("local_cuda"));
    }

    #[test]
    fn default_provider_prefers_zai() {
        let providers = vec!["openai", "zai"];
        assert_eq!(choose_default_provider(&providers), Some("zai"));
    }

    #[test]
    fn extract_candidate_file_paths_filters_to_existing_files() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::create_dir_all(root.join("api/src")).expect("mkdirs");
        fs::write(root.join("api/src/index.ts"), "export {};").expect("write file");

        let text = "Check api/src/index.ts and missing/path.ts";
        let paths = extract_candidate_file_paths(text, root, 5);
        assert_eq!(paths, vec!["api/src/index.ts".to_string()]);
    }

    #[test]
    fn extract_candidate_file_paths_dedupes_and_respects_limit() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::create_dir_all(root.join("a")).expect("mkdirs");
        fs::create_dir_all(root.join("b")).expect("mkdirs");
        fs::write(root.join("a/one.ts"), "export {};").expect("write file");
        fs::write(root.join("b/two.tsx"), "export {};").expect("write file");

        let text = "a/one.ts a/one.ts b/two.tsx";
        let paths = extract_candidate_file_paths(text, root, 1);
        assert_eq!(paths, vec!["a/one.ts".to_string()]);
    }

    #[test]
    fn detect_stub_in_tool_input_flags_write_placeholder_content() {
        let args = json!({
            "path": "src/demo.ts",
            "content": "export function run(){ return \"Fallback prompt\"; } // Placeholder"
        });
        let reason = detect_stub_in_tool_input("write", &args);
        assert!(reason.is_some());
    }

    #[test]
    fn detect_stub_in_tool_input_allows_concrete_edit_content() {
        let args = json!({
            "old_string": "return a + b;",
            "new_string": "return sanitize(a) + sanitize(b);"
        });
        let reason = detect_stub_in_tool_input("edit", &args);
        assert!(reason.is_none());
    }

    #[test]
    fn detect_stub_in_tool_input_flags_multiedit_stub_line() {
        let args = json!({
            "edits": [
                {
                    "file_path": "src/a.ts",
                    "old_string": "x",
                    "new_string": "throw new Error(\"Not implemented\")"
                }
            ]
        });
        let reason = detect_stub_in_tool_input("multiedit", &args);
        assert!(reason.is_some());
    }

    #[test]
    fn detect_stub_in_tool_input_allows_html_placeholder_attribute() {
        let args = json!({
            "old_string": "<input type=\"text\" />",
            "new_string": "<input type=\"text\" placeholder=\"Search users\" />"
        });
        let reason = detect_stub_in_tool_input("edit", &args);
        assert!(reason.is_none());
    }

    #[test]
    fn detect_stub_in_tool_input_flags_placeholder_stub_phrase() {
        let args = json!({
            "path": "src/demo.ts",
            "content": "// Placeholder implementation\nexport const run = () => null;"
        });
        let reason = detect_stub_in_tool_input("write", &args);
        assert!(reason.is_some());
    }

    #[test]
    fn looks_like_build_execution_request_detects_fix_prompt() {
        assert!(looks_like_build_execution_request(
            "yes fix it and patch src/tool/lsp.rs"
        ));
        assert!(!looks_like_build_execution_request(
            "what does this module do?"
        ));
    }

    #[test]
    fn codesearch_no_match_detection_matches_expected_format() {
        assert!(is_codesearch_no_match_output(
            "codesearch",
            true,
            "No matches found for pattern: foo_bar"
        ));
    }

    #[test]
    fn codesearch_no_match_detection_matches_prefixed_output() {
        assert!(is_codesearch_no_match_output(
            "codesearch",
            true,
            "build step 1 codesearch: No matches found for pattern: foo_bar"
        ));
    }

    #[test]
    fn codesearch_no_match_detection_ignores_non_codesearch_tools() {
        assert!(!is_codesearch_no_match_output(
            "grep",
            true,
            "No matches found for pattern: foo_bar"
        ));
    }

    #[test]
    fn build_mode_tool_first_retry_triggers_for_deferral_reply() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
        fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

        let tools = vec![ToolDefinition {
            name: "bash".to_string(),
            description: "Run shell commands".to_string(),
            parameters: json!({"type":"object"}),
        }];
        let session_messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "fix src/provider/gemini_web.rs now".to_string(),
            }],
        }];

        let should_retry = should_force_build_tool_first_retry(
            "build",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
        );

        assert!(should_retry);
    }

    #[test]
    fn build_mode_tool_first_retry_does_not_trigger_for_non_build_agent() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
        fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

        let tools = vec![ToolDefinition {
            name: "bash".to_string(),
            description: "Run shell commands".to_string(),
            parameters: json!({"type":"object"}),
        }];
        let session_messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "fix src/provider/gemini_web.rs now".to_string(),
            }],
        }];

        let should_retry = should_force_build_tool_first_retry(
            "plan",
            0,
            &tools,
            &session_messages,
            root,
            "If you want, I can patch this next.",
            false,
        );

        assert!(!should_retry);
    }

    #[test]
    fn build_request_requires_tool_detects_existing_file_edit_prompt() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::create_dir_all(root.join("src/provider")).expect("mkdirs");
        fs::write(root.join("src/provider/gemini_web.rs"), "fn main(){}").expect("write file");

        let session_messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "In build mode, make a concrete change now: in src/provider/gemini_web.rs replace \"A\" with \"B\".".to_string(),
            }],
        }];

        assert!(build_request_requires_tool(&session_messages, root));
    }

    #[test]
    fn build_request_requires_tool_for_affirmative_followup_with_context() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        fs::write(root.join("rspack.config.js"), "module.exports = {};").expect("write file");

        let session_messages = vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "ws error in rspack.config.js".to_string(),
                }],
            },
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: "Done — use this exact block in rspack.config.js".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "yes".to_string(),
                }],
            },
        ];

        assert!(build_request_requires_tool(&session_messages, root));
    }

    #[test]
    fn build_request_requires_tool_for_affirmative_followup_after_assistant_offer() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        let session_messages = vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "the banner is missing".to_string(),
                }],
            },
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: "If you want, next I can tighten the Catalyst alert variant exactly."
                        .to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "yes".to_string(),
                }],
            },
        ];

        assert!(build_request_requires_tool(&session_messages, root));
    }

    #[test]
    fn build_request_requires_tool_false_for_plain_yes_without_context() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        let session_messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "yes".to_string(),
            }],
        }];

        assert!(!build_request_requires_tool(&session_messages, root));
    }
}
