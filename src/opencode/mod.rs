//! OpenCode session integration
//!
//! This module enables CodeTether to read and resume OpenCode sessions
//! by parsing OpenCode's storage format.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Default OpenCode storage directory
pub fn opencode_storage_dir() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|b| b.data_local_dir().join("opencode").join("storage"))
}

/// OpenCode session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeSession {
    pub id: String,
    pub version: String,
    #[serde(rename = "projectID")]
    pub project_id: String,
    pub directory: String,
    pub title: String,
    pub time: OpenCodeTime,
    #[serde(default)]
    pub summary: Option<OpenCodeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeTime {
    pub created: i64,
    pub updated: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenCodeSummary {
    #[serde(default)]
    pub additions: i64,
    #[serde(default)]
    pub deletions: i64,
    #[serde(default)]
    pub files: i64,
}

/// OpenCode message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeMessage {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub role: String,
    pub time: OpenCodeMessageTime,
    #[serde(default)]
    pub summary: Option<OpenCodeMessageSummary>,
    #[serde(rename = "parentID", default)]
    pub parent_id: Option<String>,
    #[serde(rename = "modelID", default)]
    pub model_id: Option<String>,
    #[serde(rename = "providerID", default)]
    pub provider_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub path: Option<OpenCodeMessagePath>,
    #[serde(default)]
    pub cost: Option<f64>,
    #[serde(default)]
    pub tokens: Option<OpenCodeTokens>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub model: Option<OpenCodeModel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeMessageTime {
    pub created: i64,
    #[serde(rename = "completed", default)]
    pub completed: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenCodeMessageSummary {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub diffs: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeMessagePath {
    pub cwd: String,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeTokens {
    #[serde(default)]
    pub input: i64,
    #[serde(default)]
    pub output: i64,
    #[serde(default)]
    pub reasoning: i64,
    #[serde(default)]
    pub cache: Option<OpenCodeCacheTokens>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeCacheTokens {
    #[serde(default)]
    pub read: i64,
    #[serde(default)]
    pub write: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeModel {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
}

/// OpenCode message part (content)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodePart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub part_type: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub time: Option<OpenCodePartTime>,
    // Tool call fields
    #[serde(rename = "toolCallID", default)]
    pub tool_call_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub arguments: Option<String>,
    // Tool result fields
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodePartTime {
    #[serde(default)]
    pub start: Option<i64>,
    #[serde(default)]
    pub end: Option<i64>,
}

/// OpenCode todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCodeTodo {
    pub id: String,
    pub content: String,
    pub status: String,
    pub priority: String,
}

/// Summary of an OpenCode session for listing
#[derive(Debug, Clone)]
pub struct OpenCodeSessionSummary {
    pub id: String,
    pub title: String,
    pub directory: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
}

/// Reader for OpenCode storage
pub struct OpenCodeStorage {
    storage_dir: PathBuf,
}

impl OpenCodeStorage {
    /// Create a new OpenCode storage reader with default location
    pub fn new() -> Option<Self> {
        opencode_storage_dir().map(|dir| Self { storage_dir: dir })
    }

    /// Create a new OpenCode storage reader with custom location
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        Self {
            storage_dir: path.as_ref().to_path_buf(),
        }
    }

    /// Check if OpenCode storage exists
    pub fn exists(&self) -> bool {
        self.storage_dir.exists()
    }

    /// Get the storage directory path
    pub fn path(&self) -> &Path {
        &self.storage_dir
    }

    /// List all available sessions
    pub async fn list_sessions(&self) -> Result<Vec<OpenCodeSessionSummary>> {
        let session_dir = self.storage_dir.join("session");

        if !session_dir.exists() {
            return Ok(Vec::new());
        }

        let mut summaries = Vec::new();
        let mut entries = fs::read_dir(&session_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Ok(session) = serde_json::from_str::<OpenCodeSession>(&content) {
                        let message_count = self.count_messages(&session.id).await.unwrap_or(0);

                        summaries.push(OpenCodeSessionSummary {
                            id: session.id.clone(),
                            title: session.title,
                            directory: session.directory,
                            created_at: DateTime::from_timestamp_millis(session.time.created)
                                .unwrap_or_else(|| Utc::now()),
                            updated_at: DateTime::from_timestamp_millis(session.time.updated)
                                .unwrap_or_else(|| Utc::now()),
                            message_count,
                        });
                    }
                }
            }
        }

        // Sort by updated_at descending
        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(summaries)
    }

    /// List sessions for a specific directory
    pub async fn list_sessions_for_directory(
        &self,
        dir: &Path,
    ) -> Result<Vec<OpenCodeSessionSummary>> {
        let all = self.list_sessions().await?;
        let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());

        Ok(all
            .into_iter()
            .filter(|s| {
                let session_dir = Path::new(&s.directory);
                let canonical_session = session_dir
                    .canonicalize()
                    .unwrap_or_else(|_| session_dir.to_path_buf());
                canonical_session == canonical_dir
            })
            .collect())
    }

    /// Load a session by ID
    pub async fn load_session(&self, session_id: &str) -> Result<OpenCodeSession> {
        let path = self
            .storage_dir
            .join("session")
            .join(format!("{}.json", session_id));
        let content = fs::read_to_string(&path)
            .await
            .with_context(|| format!("Failed to read session file: {}", path.display()))?;
        let session: OpenCodeSession = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse session JSON: {}", session_id))?;
        Ok(session)
    }

    /// Load all messages for a session
    pub async fn load_messages(&self, session_id: &str) -> Result<Vec<OpenCodeMessage>> {
        let message_dir = self.storage_dir.join("message").join(session_id);

        if !message_dir.exists() {
            return Ok(Vec::new());
        }

        let mut messages = Vec::new();
        let mut entries = fs::read_dir(&message_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Ok(message) = serde_json::from_str::<OpenCodeMessage>(&content) {
                        messages.push(message);
                    }
                }
            }
        }

        // Sort by creation time
        messages.sort_by_key(|m| m.time.created);
        Ok(messages)
    }

    /// Load parts for a message
    pub async fn load_parts(&self, message_id: &str) -> Result<Vec<OpenCodePart>> {
        let part_dir = self.storage_dir.join("part").join(message_id);

        if !part_dir.exists() {
            return Ok(Vec::new());
        }

        let mut parts = Vec::new();
        let mut entries = fs::read_dir(&part_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Ok(part) = serde_json::from_str::<OpenCodePart>(&content) {
                        parts.push(part);
                    }
                }
            }
        }

        // Sort by start time if available
        parts.sort_by_key(|p| p.time.as_ref().and_then(|t| t.start).unwrap_or(0));
        Ok(parts)
    }

    /// Load todos for a session
    pub async fn load_todos(&self, session_id: &str) -> Result<Vec<OpenCodeTodo>> {
        let todo_path = self
            .storage_dir
            .join("todo")
            .join(format!("{}.json", session_id));

        if !todo_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&todo_path).await?;
        let todos: Vec<OpenCodeTodo> = serde_json::from_str(&content)?;
        Ok(todos)
    }

    /// Count messages in a session
    async fn count_messages(&self, session_id: &str) -> Result<usize> {
        let message_dir = self.storage_dir.join("message").join(session_id);

        if !message_dir.exists() {
            return Ok(0);
        }

        let mut count = 0;
        let mut entries = fs::read_dir(&message_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Get the most recent session
    pub async fn last_session(&self) -> Result<OpenCodeSession> {
        let sessions = self.list_sessions().await?;

        if let Some(first) = sessions.first() {
            self.load_session(&first.id).await
        } else {
            anyhow::bail!("No OpenCode sessions found")
        }
    }

    /// Get the most recent session for a directory
    pub async fn last_session_for_directory(&self, dir: &Path) -> Result<OpenCodeSession> {
        let sessions = self.list_sessions_for_directory(dir).await?;

        if let Some(first) = sessions.first() {
            self.load_session(&first.id).await
        } else {
            anyhow::bail!(
                "No OpenCode sessions found for directory: {}",
                dir.display()
            )
        }
    }
}

/// Convert OpenCode session to CodeTether session
pub mod convert {
    use super::*;
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::{Session, SessionMetadata};

    /// Convert an OpenCode session and its messages to a CodeTether session
    pub async fn to_codetether_session(
        opencode_session: &OpenCodeSession,
        messages: Vec<(OpenCodeMessage, Vec<OpenCodePart>)>,
    ) -> Result<Session> {
        let mut codetether_messages = Vec::new();

        for (msg, parts) in messages {
            let role = match msg.role.as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                _ => Role::User, // Default to user for unknown roles
            };

            let content = convert_parts_to_content(&parts);

            codetether_messages.push(Message { role, content });
        }

        let session = Session {
            id: format!("opencode_{}", opencode_session.id),
            title: Some(opencode_session.title.clone()),
            created_at: DateTime::from_timestamp_millis(opencode_session.time.created)
                .unwrap_or_else(|| Utc::now()),
            updated_at: DateTime::from_timestamp_millis(opencode_session.time.updated)
                .unwrap_or_else(|| Utc::now()),
            messages: codetether_messages,
            tool_uses: Vec::new(), // OpenCode doesn't store tool uses separately
            usage: Default::default(),
            agent: "build".to_string(), // Default agent
            metadata: SessionMetadata {
                directory: Some(PathBuf::from(&opencode_session.directory)),
                model: None, // Could extract from messages
                shared: false,
                share_url: None,
            },
        };

        Ok(session)
    }

    /// Convert OpenCode parts to CodeTether content parts
    fn convert_parts_to_content(parts: &[OpenCodePart]) -> Vec<ContentPart> {
        let mut content = Vec::new();

        for part in parts {
            match part.part_type.as_str() {
                "text" => {
                    if let Some(text) = &part.text {
                        content.push(ContentPart::Text { text: text.clone() });
                    }
                }
                "tool-call" => {
                    if let (Some(name), Some(args)) = (&part.name, &part.arguments) {
                        content.push(ContentPart::ToolCall {
                            id: part.tool_call_id.clone().unwrap_or_default(),
                            name: name.clone(),
                            arguments: args.clone(),
                        });
                    }
                }
                "tool-result" => {
                    if let Some(tool_content) = &part.content {
                        content.push(ContentPart::ToolResult {
                            tool_call_id: part.tool_call_id.clone().unwrap_or_default(),
                            content: tool_content.clone(),
                        });
                    }
                }
                _ => {
                    // Unknown part type, try to extract text if available
                    if let Some(text) = &part.text {
                        content.push(ContentPart::Text { text: text.clone() });
                    }
                }
            }
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opencode_storage_dir() {
        let dir = opencode_storage_dir();
        assert!(dir.is_some());
        let dir = dir.unwrap();
        assert!(dir.to_string_lossy().contains("opencode"));
        assert!(dir.to_string_lossy().contains("storage"));
    }

    #[tokio::test]
    async fn test_storage_exists() {
        if let Some(storage) = OpenCodeStorage::new() {
            // Just test that we can create the storage reader
            // Don't assume OpenCode is installed
            let _ = storage.exists();
        }
    }
}
