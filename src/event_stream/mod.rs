//! Event Stream Module
//!
//! Structured JSONL event sourcing for agent sessions. Every agent turn,
//! tool call, and handoff is captured as an append-only event stream with
//! byte-range offsets for efficient random access replay.
//!
//! ## Event Schema
//!
//! ```json
//! {
//!   "recorded_at": "2026-02-13T04:16:56.465006066+00:00",
//!   "workspace": "/home/riley/A2A-Server-MCP",
//!   "session_id": "36d04218-2a47-4fbe-8579-02c21be775bc",
//!   "role": "tool",
//!   "agent_name": null,
//!   "message_type": "tool_result",
//!   "content": "✓ bash",
//!   "tool_name": "bash",
//!   "tool_success": true,
//!   "tool_duration_ms": 22515
//! }
//! ```
//!
//! ## File Naming Convention
//!
//! Files are named with byte-range offsets to enable random access:
//! `{timestamp}-chat-events-{start_byte}-{end_byte}.jsonl`
//!
//! This allows seeking to any point in a session without reading the entire log.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Categories of events in the stream
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventCategory {
    /// User message/turn
    User,
    /// Assistant/agent message
    Assistant,
    /// Tool execution result
    ToolResult,
    /// Agent handoff
    Handoff,
    /// Session lifecycle
    Session,
    /// Error event
    Error,
}

/// A single event in the JSONL event stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    /// When the event was recorded (ISO 8601)
    pub recorded_at: DateTime<Utc>,
    /// Workspace/working directory
    pub workspace: PathBuf,
    /// Session identifier
    pub session_id: String,
    /// Role: user, assistant, tool
    pub role: String,
    /// Agent name if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// Message type: message_start, message_end, tool_call, tool_result, etc.
    pub message_type: String,
    /// Event content (truncated for large outputs)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Tool name if this is a tool event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    /// Whether tool succeeded
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_success: Option<bool>,
    /// Tool execution duration in milliseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_duration_ms: Option<u64>,
    /// Parent event ID for traceability
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<String>,
    /// Sequence number in the session
    pub sequence: u64,
}

impl ChatEvent {
    /// Create a new chat event
    pub fn new(
        workspace: PathBuf,
        session_id: String,
        role: &str,
        message_type: &str,
        sequence: u64,
    ) -> Self {
        Self {
            recorded_at: Utc::now(),
            workspace,
            session_id,
            role: role.to_string(),
            agent_name: None,
            message_type: message_type.to_string(),
            content: None,
            tool_name: None,
            tool_success: None,
            tool_duration_ms: None,
            parent_event_id: None,
            sequence,
        }
    }

    /// Create a tool result event
    pub fn tool_result(
        workspace: PathBuf,
        session_id: String,
        tool_name: &str,
        success: bool,
        duration_ms: u64,
        content: &str,
        sequence: u64,
    ) -> Self {
        // Truncate content if too long (object storage optimization)
        let max_content_len = 10_000;
        let truncated_content = if content.len() > max_content_len {
            format!("{}...[truncated {} bytes]", &content[..max_content_len], content.len())
        } else {
            content.to_string()
        };

        Self {
            recorded_at: Utc::now(),
            workspace,
            session_id,
            role: "tool".to_string(),
            agent_name: None,
            message_type: "tool_result".to_string(),
            content: Some(truncated_content),
            tool_name: Some(tool_name.to_string()),
            tool_success: Some(success),
            tool_duration_ms: Some(duration_ms),
            parent_event_id: None,
            sequence,
        }
    }

    /// Serialize to JSON for JSONL output
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// File metadata for byte-range indexed JSONL
#[derive(Debug, Clone)]
pub struct EventFile {
    /// Path to the JSONL file
    pub path: PathBuf,
    /// Starting byte offset
    pub start_offset: u64,
    /// Ending byte offset
    pub end_offset: u64,
    /// Number of events in this file
    pub event_count: u64,
    /// First event timestamp
    pub first_event_at: DateTime<Utc>,
    /// Last event timestamp
    pub last_event_at: DateTime<Utc>,
}

impl EventFile {
    /// Generate filename with byte-range offsets
    pub fn filename(_session_id: &str, start_offset: u64, end_offset: u64) -> String {
        let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
        format!(
            "{}-chat-events-{:020}-{:020}.jsonl",
            timestamp, start_offset, end_offset
        )
    }
}

/// Event stream writer that manages append-only JSONL with byte-range tracking
pub struct EventStreamWriter {
    session_id: String,
    workspace: PathBuf,
    /// Current file path
    current_path: Option<PathBuf>,
    /// Current byte offset
    current_offset: u64,
    /// Events written to current file
    events_in_file: u64,
    /// Max events per file before rotation
    max_events_per_file: u64,
    /// Max bytes per file before rotation
    max_bytes_per_file: u64,
    /// Sequence counter
    sequence: u64,
}

impl EventStreamWriter {
    /// Create a new event stream writer
    pub fn new(
        session_id: String,
        workspace: PathBuf,
        max_events_per_file: u64,
        max_bytes_per_file: u64,
    ) -> Self {
        Self {
            session_id,
            workspace,
            current_path: None,
            current_offset: 0,
            events_in_file: 0,
            max_events_per_file,
            max_bytes_per_file,
            sequence: 0,
        }
    }

    /// Write an event and return the byte range
    pub async fn write_event(&mut self, event: ChatEvent) -> std::io::Result<(u64, u64)> {
        use tokio::io::AsyncWriteExt;

        let json = event.to_json();
        let event_size = json.len() as u64 + 1; // +1 for newline

        // Check if we need to rotate
        if self.events_in_file >= self.max_events_per_file
            || (self.current_offset + event_size) > self.max_bytes_per_file
        {
            self.rotate().await?;
        }

        // Generate filename if first file
        if self.current_path.is_none() {
            let filename = EventFile::filename(&self.session_id, self.current_offset, self.current_offset + event_size);
            self.current_path = Some(self.workspace.join(filename));
        }

        let start = self.current_offset;

        // Append to file
        if let Some(ref path) = self.current_path {
            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await?;

            file.write_all(json.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }

        let end = self.current_offset + event_size;
        self.current_offset += event_size;
        self.events_in_file += 1;
        self.sequence += 1;

        // Update filename with new end offset
        if let Some(ref path) = self.current_path {
            if let Some(parent) = path.parent() {
                let filename = EventFile::filename(&self.session_id, start, end);
                let new_path = parent.join(filename);
                // Rename file if offset changed significantly
                if new_path != *path {
                    let _ = tokio::fs::rename(path, &new_path).await;
                    self.current_path = Some(new_path);
                }
            }
        }

        Ok((start, end))
    }

    /// Rotate to a new file
    async fn rotate(&mut self) -> std::io::Result<()> {
        self.current_path = None;
        self.events_in_file = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = ChatEvent::tool_result(
            PathBuf::from("/test/workspace"),
            "session-123".to_string(),
            "bash",
            true,
            22515,
            "✓ bash",
            1,
        );

        let json = event.to_json();
        let parsed: ChatEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.session_id, "session-123");
        assert_eq!(parsed.tool_name, Some("bash".to_string()));
        assert_eq!(parsed.tool_success, Some(true));
        assert_eq!(parsed.tool_duration_ms, Some(22515));
    }

    #[test]
    fn test_filename_format() {
        let filename = EventFile::filename("session-123", 1000, 2500);
        assert!(filename.contains("chat-events-"));
        // Filename ends with .jsonl, so we check for the offset followed by .jsonl
        assert!(filename.contains("-00000000000000001000-") || filename.contains("-00000000000000001000."));
        assert!(filename.contains("-00000000000000002500.jsonl"));
    }
}
