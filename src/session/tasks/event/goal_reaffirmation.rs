//! Progress-note payload, preserving the existing goal-reaffirmed JSON shape.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A progress note reaffirming the current goal.
///
/// # Examples
/// ```
/// use codetether_agent::session::tasks::GoalReaffirmation;
/// let note = GoalReaffirmation { at: chrono::Utc::now(), progress_note: "Tests running".into() };
/// assert!(!note.progress_note.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalReaffirmation {
    /// When the progress note was recorded.
    pub at: DateTime<Utc>,
    /// Progress note, with no authority to change a goal's status.
    pub progress_note: String,
}
