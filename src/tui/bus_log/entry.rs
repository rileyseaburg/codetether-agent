//! Render-ready protocol bus log entries.

use ratatui::style::Color;

use crate::bus::BusEnvelope;

/// A single render-ready row in the protocol bus log.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::bus_log::BusLogEntry;
/// use ratatui::style::Color;
///
/// let entry = BusLogEntry {
///     timestamp: "00:00:00.000".to_string(),
///     topic: "broadcast".to_string(),
///     sender_id: "agent".to_string(),
///     kind: "MSG".to_string(),
///     summary: "ready".to_string(),
///     detail: "ready".to_string(),
///     kind_color: Color::Cyan,
/// };
/// assert_eq!(entry.kind, "MSG");
/// ```
#[derive(Debug, Clone)]
pub struct BusLogEntry {
    /// Local display timestamp.
    pub timestamp: String,
    /// Envelope topic used for filtering.
    pub topic: String,
    /// Sender identifier used for filtering.
    pub sender_id: String,
    /// Short message kind label.
    pub kind: String,
    /// One-line row summary.
    pub summary: String,
    /// Full detail text shown in detail mode.
    pub detail: String,
    /// Color used for the kind label.
    pub kind_color: Color,
}

impl BusLogEntry {
    /// Converts a bus envelope into a render-ready log entry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use codetether_agent::{
    ///     bus::{BusEnvelope, BusMessage},
    ///     tui::bus_log::BusLogEntry,
    /// };
    ///
    /// let env = BusEnvelope {
    ///     id: "1".to_string(),
    ///     topic: "broadcast".to_string(),
    ///     sender_id: "agent".to_string(),
    ///     correlation_id: None,
    ///     timestamp: Utc::now(),
    ///     message: BusMessage::AgentShutdown {
    ///         agent_id: "agent".to_string(),
    ///     },
    /// };
    /// let entry = BusLogEntry::from_envelope(&env);
    /// assert_eq!(entry.kind, "SHUTDOWN");
    /// ```
    pub fn from_envelope(env: &BusEnvelope) -> Self {
        let parts = super::entry_router::entry_parts(&env.message);
        Self {
            timestamp: env.timestamp.format("%H:%M:%S%.3f").to_string(),
            topic: env.topic.clone(),
            sender_id: env.sender_id.clone(),
            kind: parts.kind,
            summary: parts.summary,
            detail: parts.detail,
            kind_color: parts.kind_color,
        }
    }
}
