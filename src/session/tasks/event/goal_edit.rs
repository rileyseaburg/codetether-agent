//! Persisted user goal edits. Replay preserves identity and accounting.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User-requested changes; completion remains a separate verified operation.
///
/// # Examples
/// ```
/// use codetether_agent::session::tasks::GoalEdit;
/// let edit: GoalEdit = serde_json::from_value(serde_json::json!({
///     "goalId": "goal", "updatedAt": "2026-01-01T00:00:00Z",
///     "action": "edit", "tokenBudget": null
/// })).unwrap();
/// assert_eq!(edit.token_budget, Some(None));
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GoalEdit {
    /// Stable native goal identity, never a session title.
    pub goal_id: String,
    /// Exact revision observed before the user began editing.
    pub updated_at: DateTime<Utc>,
    /// Explicit operation initiated by the user.
    pub action: GoalEditAction,
    /// Optional replacement objective; absent preserves the current text.
    #[serde(default)]
    pub objective: Option<String>,
    /// Optional replacement completion requirements.
    #[serde(default)]
    pub success_criteria: Option<Vec<String>>,
    /// Optional replacement list of prohibited actions.
    #[serde(default)]
    pub forbidden: Option<Vec<String>>,
    /// Missing preserves the cap; explicit null removes it.
    #[serde(
        default,
        deserialize_with = "budget",
        skip_serializing_if = "Option::is_none"
    )]
    pub token_budget: Option<Option<i64>>,
}

#[path = "goal_edit_action.rs"]
mod action;
pub use action::GoalEditAction;

/// One append-only edit, including the version the user actually inspected.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoalEdited {
    /// Native append time, also the next accepted goal revision.
    pub at: DateTime<Utc>,
    /// User request with a compare-and-set goal identity and revision.
    pub request: GoalEdit,
}

/// Preserve the distinction between an omitted budget and an explicit null.
fn budget<'de, D>(input: D) -> Result<Option<Option<i64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<i64>::deserialize(input).map(Some)
}
