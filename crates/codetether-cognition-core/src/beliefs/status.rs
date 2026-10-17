//! Validity states for a stored belief.
use serde::{Deserialize, Serialize};

/// Status of a belief in the store.
///
/// States are Active, Stale, Invalidated and Archived.
///
/// # Examples
/// ```
/// use codetether_cognition_core::beliefs::BeliefStatus;
/// match BeliefStatus::Active {
///     BeliefStatus::Active => {},
///     BeliefStatus::Stale | BeliefStatus::Invalidated | BeliefStatus::Archived => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BeliefStatus {
    /// Current belief eligible for active context.
    Active,
    /// Belief awaiting revalidation.
    Stale,
    /// Belief rejected by evidence.
    Invalidated,
    /// Belief retained outside active use.
    Archived,
}
