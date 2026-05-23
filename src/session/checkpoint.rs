//! Resumable run checkpoint data persisted beside session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A structured checkpoint capturing the state of a `codetether run` that
/// exhausted its step budget, enabling automatic resume.
///
/// All fields are populated from runtime session data by the
/// [`RunCheckpoint::from_session`] constructor.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunCheckpoint {
    pub reason: CheckpointReason,
    pub original_objective: String,
    pub max_step_budget: usize,
    pub session_id: String,
    pub workspace: Option<PathBuf>,
    pub message_count: usize,
    /// Last known browser URL extracted from tool calls in the session.
    pub current_browser_url: Option<String>,
    /// Tool call names that completed successfully before checkpoint.
    pub completed_actions: Vec<String>,
    /// Errors or blockers encountered before checkpoint.
    pub blockers: Vec<String>,
    /// Inferred next action based on the last assistant message.
    pub next_intended_action: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReason {
    MaxStepsExhausted,
}

impl RunCheckpoint {
    /// Build a checkpoint for a max-step-exhausted run when message history is
    /// not available to inspect. Runtime code should prefer
    /// [`RunCheckpoint::from_session_messages`] so browser URLs, completed
    /// actions, blockers, and next actions are extracted from real session
    /// state.
    pub fn exhausted(
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
        message_count: usize,
    ) -> Self {
        Self {
            reason: CheckpointReason::MaxStepsExhausted,
            original_objective: objective.into(),
            max_step_budget: max_steps,
            session_id: session_id.into(),
            workspace,
            message_count,
            current_browser_url: None,
            completed_actions: Vec::new(),
            blockers: Vec::new(),
            next_intended_action:
                "Continue from the last assistant/tool state toward the original objective."
                    .to_string(),
            created_at: Utc::now(),
        }
    }

    /// Build a checkpoint by extracting real runtime state from session
    /// messages.
    ///
    /// * `objective` — the original user prompt / objective string.
    /// * `max_steps` — the step budget that was exhausted.
    /// * `session_id` — the session UUID.
    /// * `workspace` — optional workspace path.
    /// * `message_count` — total messages at checkpoint time.
    /// * `messages` — the session message slice to extract state from.
    pub fn from_session_messages(
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
        message_count: usize,
        messages: &[crate::provider::Message],
    ) -> Self {
        let extracted = extract_checkpoint_state(messages);
        Self {
            reason: CheckpointReason::MaxStepsExhausted,
            original_objective: objective.into(),
            max_step_budget: max_steps,
            session_id: session_id.into(),
            workspace,
            message_count,
            current_browser_url: extracted.browser_url,
            completed_actions: extracted.completed_actions,
            blockers: extracted.blockers,
            next_intended_action: extracted.next_action,
            created_at: Utc::now(),
        }
    }
}

/// Intermediate state extracted from session messages.
struct ExtractedState {
    browser_url: Option<String>,
    completed_actions: Vec<String>,
    blockers: Vec<String>,
    next_action: String,
}

fn extract_checkpoint_state(messages: &[crate::provider::Message]) -> ExtractedState {
    use crate::provider::{ContentPart, Role};

    let mut browser_url: Option<String> = None;
    let mut completed_actions: Vec<String> = Vec::new();
    let mut blockers: Vec<String> = Vec::new();
    let mut last_assistant_text: Option<String> = None;

    for msg in messages {
        match msg.role {
            Role::Tool => {
                for part in &msg.content {
                    if let ContentPart::ToolResult { content, .. } = part {
                        extract_browser_url_from_text(content, &mut browser_url);
                        if content.contains("error") || content.contains("Error") {
                            blockers.push(truncate_to_line(content, 200));
                        }
                    }
                }
            }
            Role::Assistant => {
                for part in &msg.content {
                    match part {
                        ContentPart::ToolCall { name, .. } => {
                            completed_actions.push(name.clone());
                        }
                        ContentPart::Text { text } if !text.is_empty() => {
                            last_assistant_text = Some(truncate_to_line(text, 500));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let next_action = last_assistant_text.unwrap_or_else(|| {
        "Continue from the last assistant/tool state toward the original objective.".to_string()
    });

    ExtractedState {
        browser_url,
        completed_actions,
        blockers,
        next_action,
    }
}

/// Scan text for the last URL that looks like a browser page.
fn extract_browser_url_from_text(text: &str, url: &mut Option<String>) {
    for line in text.lines().rev() {
        if let Some(found) = line.split_whitespace().find(|s| s.starts_with("https://")) {
            *url = Some(
                found
                    .trim_end_matches(',')
                    .trim_end_matches('.')
                    .to_string(),
            );
            return;
        }
        if let Some(found) = line.split_whitespace().find(|s| s.starts_with("http://")) {
            *url = Some(
                found
                    .trim_end_matches(',')
                    .trim_end_matches('.')
                    .to_string(),
            );
            return;
        }
    }
}

/// Truncate text to the first non-empty line, capped at `max` chars.
fn truncate_to_line(text: &str, max: usize) -> String {
    let line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    if line.len() <= max {
        line.to_string()
    } else {
        format!("{}...", &line[..max.saturating_sub(3)])
    }
}
