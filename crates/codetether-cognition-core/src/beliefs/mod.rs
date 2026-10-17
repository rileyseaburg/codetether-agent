//! Structured belief contracts and confidence lifecycle.
//!
//! [Belief] records one claim and its provenance; [BeliefStatus] tracks validity.
//! [extraction] builds model prompts and decodes replies without invoking a model.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
pub mod extraction;
mod lifecycle;
mod status;
pub use status::BeliefStatus;

/// A structured belief in the shared store.
///
/// # Examples
/// ```
/// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
/// let mut belief = parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service reachable","belief_key":"service-reachable","confidence":0.6}]}"#).remove(0);
/// assert_eq!(belief.belief_key, "service-reachable");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    /// Unique record identifier.
    pub id: String,
    /// Normalized canonical key — prevents duplicate beliefs.
    pub belief_key: String,
    /// Natural-language factual claim.
    pub claim: String,
    /// Runtime-bounded confidence in [0.05, 0.95].
    pub confidence: f32,
    /// Event IDs that serve as evidence.
    pub evidence_refs: Vec<String>,
    /// Persona that first asserted this belief.
    pub asserted_by: String,
    /// Identities that confirmed this belief.
    pub confirmed_by: Vec<String>,
    /// Identities that contested this belief.
    pub contested_by: Vec<String>,
    /// IDs of beliefs this contradicts.
    pub contradicts: Vec<String>,
    /// Record creation time.
    pub created_at: DateTime<Utc>,
    /// Time of the last lifecycle update.
    pub updated_at: DateTime<Utc>,
    /// Staleness expiry — default: created_at + 1h.
    pub review_after: DateTime<Utc>,
    /// Current validity state.
    pub status: BeliefStatus,
}

#[cfg(test)]
mod tests;

/// Duplicate confirmation, merge, contradiction and decay updates.
pub mod update;
