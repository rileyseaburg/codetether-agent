//! Optional belief query filters; omitted fields leave that dimension unrestricted.
use serde::Deserialize;
/// Filters matched against serialized status and exact asserting persona.
/// # Examples
/// ```
/// let filter = codetether_cognition_api::BeliefFilter { status: None, persona: None };
/// assert!(filter.status.is_none());
/// ```

// ── Belief, Attention, Governance, Workspace handlers ──

#[derive(Deserialize)]
pub struct BeliefFilter {
    pub status: Option<String>,
    pub persona: Option<String>,
}
