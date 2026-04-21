//! `session_recall` tool: RLM-powered recall over the agent's own
//! persisted session history in `.codetether-agent/sessions/`.
//!
//! When the model "forgets" something that happened earlier in this
//! workspace (an earlier prompt, a tool result, a decision), it can
//! call this tool with a natural-language query. The tool loads
//! one-or-more recent sessions for the current workspace, flattens
//! their messages through [`messages_to_rlm_context`], and runs
//! [`RlmRouter::auto_process`] against the flattened transcript to
//! produce a focused answer.
//!
//! This is distinct from the automatic compaction in
//! [`crate::session::helper::compression`]: that compresses the
//! *active* session's in-memory messages to fit the context window.
//! This tool recalls from the *persisted* on-disk history, across
//! sessions, on demand.

use super::{Tool, ToolResult};
use crate::provider::Provider;
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmConfig, RlmRouter};
use crate::session::Session;
use crate::session::helper::error::messages_to_rlm_context;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;

/// Default number of recent sessions scanned when no `session_id` is
/// supplied. Higher values produce more complete recall at the cost of
/// a larger RLM input (which the router will still chunk + summarise).
const DEFAULT_SESSION_LIMIT: usize = 3;

/// RLM-backed recall tool over persisted session history.
pub struct SessionRecallTool {
    provider: Arc<dyn Provider>,
    model: String,
    config: RlmConfig,
}

impl SessionRecallTool {
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            provider,
            model,
            config,
        }
    }
}

#[async_trait]
impl Tool for SessionRecallTool {
    fn id(&self) -> &str {
        "session_recall"
    }

    fn name(&self) -> &str {
        "SessionRecall"
    }

    fn description(&self) -> &str {
        "RECALL FROM YOUR OWN PAST SESSIONS. Call this whenever:\n\
         - You see `[AUTO CONTEXT COMPRESSION]` in the transcript and need \
           specifics the summary dropped (exact paths, prior tool output, \
           earlier user instructions, numbers).\n\
         - The user references something from earlier (\"like I said before\", \
           \"the task I gave you yesterday\", \"that file we edited\") and you \
           cannot find it in your active context.\n\
         - You feel uncertain about prior decisions in this workspace.\n\
         Do NOT ask the user to repeat themselves — call this tool first. \
         It runs RLM over `.codetether-agent/sessions/` for the current \
         workspace. Pass a natural-language `query` describing what you \
         need to remember. Optionally pin `session_id` or widen `limit` \
         (recent sessions to include, default 3). Distinct from the \
         `memory` tool: `memory` is curated notes you wrote; \
         `session_recall` is the raw conversation archive."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Natural-language question about past session content"
                },
                "session_id": {
                    "type": "string",
                    "description": "Specific session UUID to recall from. \
                                    When omitted, searches the most recent \
                                    sessions for the current workspace."
                },
                "limit": {
                    "type": "integer",
                    "description": "How many recent sessions to include when \
                                    session_id is not set (default 3).",
                    "default": 3
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let query = match args["query"].as_str() {
            Some(q) if !q.trim().is_empty() => q.to_string(),
            _ => return Ok(ToolResult::error("query is required")),
        };
        let session_id = args["session_id"].as_str().map(str::to_string);
        let limit = args["limit"]
            .as_u64()
            .map(|n| n as usize)
            .unwrap_or(DEFAULT_SESSION_LIMIT)
            .max(1);

        let (context, sources) = match build_recall_context(session_id, limit).await {
            Ok(ok) => ok,
            Err(e) => return Ok(ToolResult::error(format!("recall load failed: {e}"))),
        };

        if context.trim().is_empty() {
            return Ok(ToolResult::success(
                "No prior session history found for this workspace.".to_string(),
            ));
        }

        run_recall(
            &context,
            &sources,
            &query,
            Arc::clone(&self.provider),
            &self.model,
            &self.config,
        )
        .await
    }
}

/// Load session transcripts and flatten them into an RLM-ready string.
///
/// Returns the concatenated context plus a vector of human-readable
/// source labels (id + title) for the final tool output.
async fn build_recall_context(
    session_id: Option<String>,
    limit: usize,
) -> Result<(String, Vec<String>)> {
    let sessions = match session_id {
        Some(id) => vec![Session::load(&id).await?],
        None => load_recent_for_cwd(limit).await?,
    };

    let mut ctx = String::new();
    let mut sources = Vec::with_capacity(sessions.len());
    for s in &sessions {
        let label = format!(
            "{} ({})",
            s.title.as_deref().unwrap_or("<untitled>"),
            &s.id
        );
        sources.push(label.clone());
        ctx.push_str(&format!(
            "\n===== SESSION {label} — updated {} =====\n",
            s.updated_at
        ));
        ctx.push_str(&messages_to_rlm_context(&s.messages));
    }
    Ok((ctx, sources))
}

/// Load the most recent `limit` sessions scoped to the current working
/// directory. Falls back to a global scan when the cwd is unavailable.
async fn load_recent_for_cwd(limit: usize) -> Result<Vec<Session>> {
    let cwd = std::env::current_dir().ok();
    let summaries = match cwd.as_deref() {
        Some(dir) => crate::session::listing::list_sessions_for_directory(dir).await?,
        None => crate::session::list_sessions().await?,
    };

    let mut loaded = Vec::new();
    for s in summaries.into_iter().take(limit) {
        match Session::load(&s.id).await {
            Ok(sess) => loaded.push(sess),
            Err(e) => tracing::warn!(session_id = %s.id, error = %e, "session_recall: load failed"),
        }
    }
    Ok(loaded)
}

/// Invoke [`RlmRouter::auto_process`] against the flattened transcript
/// and format the result for the tool caller.
async fn run_recall(
    context: &str,
    sources: &[String],
    query: &str,
    provider: Arc<dyn Provider>,
    model: &str,
    config: &RlmConfig,
) -> Result<ToolResult> {
    let auto_ctx = AutoProcessContext {
        tool_id: "session_recall",
        tool_args: json!({ "query": query }),
        session_id: "session-recall-tool",
        abort: None,
        on_progress: None,
        provider,
        model: model.to_string(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    };

    // Prepend the query so the router's summarisation stays focused on
    // what the caller is actually trying to recall, instead of producing
    // a generic summary of the transcript.
    let framed = format!(
        "Recall task: {query}\n\n\
         Use the session transcript below to answer the recall task. \
         Quote short passages verbatim when useful; otherwise summarise.\n\n\
         {context}"
    );

    match RlmRouter::auto_process(&framed, auto_ctx, config).await {
        Ok(result) => Ok(ToolResult::success(format!(
            "Recalled from {} session(s): {}\n\
             (RLM: {} → {} tokens, {} iterations)\n\n{}",
            sources.len(),
            sources.join(", "),
            result.stats.input_tokens,
            result.stats.output_tokens,
            result.stats.iterations,
            result.processed
        ))),
        Err(e) => Ok(ToolResult::error(format!("RLM recall failed: {e}"))),
    }
}
