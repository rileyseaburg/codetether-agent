//! Runtime-injected arguments for a terminal goal transition.
use serde::Deserialize;

/// Arguments for `update_goal`, including runtime-injected context.
#[derive(Deserialize)]
/// # Examples
/// ```
/// let args = codetether_goal_tools::UpdateArgs { status: "complete".into(), evidence: "review evidence".into(), session_id: None, current_model: None, workspace: None };
/// assert_eq!(args.status, "complete");
/// ```
pub struct Args {
    /// Requested terminal status: `complete` or `blocked`.
    pub status: String,
    /// Worker's account of why the claim holds, handed to the verifier.
    #[serde(default)]
    pub evidence: String,
    #[serde(default, rename = "__ct_session_id")]
    pub session_id: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    pub current_model: Option<String>,
    #[serde(default, rename = "__ct_parent_workspace")]
    pub workspace: Option<String>,
}
