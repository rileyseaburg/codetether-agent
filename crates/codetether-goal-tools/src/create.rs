//! Input contract for model-created goals.
use serde::Deserialize;

#[derive(Deserialize)]
/// Goal objective and trusted session context.
/// # Examples
/// ```
/// let args = codetether_goal_tools::CreateArgs { objective: "Finish export".into(), session_id: None };
/// assert!(!args.objective.is_empty());
/// ```
pub struct Args {
    /// User-requested objective.
    pub objective: String,
    /// Session context injected by the runtime.
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
}
