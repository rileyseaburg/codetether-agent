//! Input handed to the independent verifier agent.

use crate::session::tasks::{Goal, GoalStatus};

/// Everything the verifier needs to judge a proposed goal transition.
///
/// The requirements come from the persisted goal, not from the worker, so
/// the worker cannot narrow the scope it is judged against.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::VerificationRequest;
///
/// let request = VerificationRequest {
///     objective: "Ship the export command".into(),
///     success_criteria: vec!["cargo test export passes".into()],
///     forbidden: vec!["editing Cargo.lock".into()],
///     claimed: GoalStatus::Complete,
///     evidence: "tests pass".into(),
/// };
/// assert_eq!(request.claimed, GoalStatus::Complete);
/// ```
#[derive(Clone, Debug)]
pub struct VerificationRequest {
    /// User-authored objective the work must satisfy.
    pub objective: String,
    /// Recorded evidence requirements that prove completion.
    pub success_criteria: Vec<String>,
    /// Actions the worker was prohibited from taking.
    pub forbidden: Vec<String>,
    /// Terminal status the worker is asking for.
    pub claimed: GoalStatus,
    /// The worker's own account of why the claim holds; unverified.
    pub evidence: String,
}

impl VerificationRequest {
    /// Build a request from the persisted goal and the worker's claim.
    ///
    /// # Arguments
    ///
    /// * `goal` — The persisted goal whose requirements are authoritative.
    /// * `claimed` — The terminal status the worker requested.
    /// * `evidence` — The worker's supporting account.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use codetether_agent::session::tasks::{Goal, GoalStatus};
    /// use codetether_agent::tool::goal::verify::VerificationRequest;
    ///
    /// let now = Utc::now();
    /// let goal = Goal {
    ///     id: "g1".into(), objective: "Ship".into(),
    ///     success_criteria: vec!["ci green".into()], forbidden: vec![],
    ///     status: GoalStatus::Active, token_budget: None, tokens_used: 0,
    ///     time_used_seconds: 0, turns_used: 1, set_at: now,
    ///     last_updated_at: now, last_reaffirmed_at: now,
    /// };
    /// let request = VerificationRequest::from_goal(&goal, GoalStatus::Complete, "done");
    /// assert_eq!(request.success_criteria, vec!["ci green".to_string()]);
    /// ```
    pub fn from_goal(goal: &Goal, claimed: GoalStatus, evidence: &str) -> Self {
        Self {
            objective: goal.objective.clone(),
            success_criteria: goal.success_criteria.clone(),
            forbidden: goal.forbidden.clone(),
            claimed,
            evidence: evidence.to_string(),
        }
    }
}
