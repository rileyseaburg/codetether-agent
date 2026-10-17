//! Shared forage opportunity data.
use chrono::{DateTime, Utc};
use codetether_okr::OkrStatus;
use serde::Serialize;
use uuid::Uuid;

/// Ranked, unfinished key-result work with its execution prompt.
///
/// Fields retain the originating objective, progress, deadline and rubric evidence.
#[derive(Debug, Clone, Serialize)]
pub struct ForageOpportunity {
    pub score: f64,
    pub okr_id: Uuid,
    pub okr_title: String,
    pub okr_status: OkrStatus,
    pub key_result_id: Uuid,
    pub key_result_title: String,
    pub progress: f64,
    pub remaining: f64,
    pub target_date: Option<DateTime<Utc>>,
    pub moonshot_alignment: f64,
    pub moonshot_hits: Vec<String>,
    pub prompt: String,
}

impl ForageOpportunity {
    /// Return the stable lowercase label used in CLI output.
    pub fn okr_status_label(&self) -> &'static str {
        match self.okr_status {
            OkrStatus::Draft => "draft",
            OkrStatus::Active => "active",
            OkrStatus::Completed => "completed",
            OkrStatus::Cancelled => "cancelled",
            OkrStatus::OnHold => "on_hold",
        }
    }
}
