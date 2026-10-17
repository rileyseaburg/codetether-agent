//! Decode and normalize structured extraction output into belief records.
use super::response::ExtractionResponse;
use crate::beliefs::{Belief, BeliefStatus};
use chrono::{Duration, Utc};
use uuid::Uuid;

/// Decode a model response into new belief records.
///
/// # Arguments
/// * `persona_id` - Identity recorded as the asserting persona.
/// * `output_text` - JSON response, optionally wrapped in Markdown fences.
///
/// # Returns
/// Valid nonblank claims with normalized keys, clamped confidence and timestamps.
/// Malformed responses return an empty vector.
///
/// # Examples
/// ```
/// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
/// assert!(parse_extracted_beliefs("p1", "invalid JSON").is_empty());
/// ```
pub fn parse_extracted_beliefs(persona_id: &str, output_text: &str) -> Vec<Belief> {
    // Strip markdown code fences if present
    let text = output_text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let parsed: ExtractionResponse = match serde_json::from_str(text) {
        Ok(parsed) => parsed,
        Err(_) => return Vec::new(),
    };

    let now = Utc::now();
    parsed
        .claims
        .into_iter()
        .filter(|c| !c.claim.trim().is_empty() && !c.belief_key.trim().is_empty())
        .map(|c| {
            let confidence = c.confidence.clamp(0.05, 0.95);
            let contradicts = c
                .contest_target
                .into_iter()
                .filter(|t| !t.is_empty())
                .collect();
            Belief {
                id: Uuid::new_v4().to_string(),
                belief_key: c.belief_key.to_lowercase().replace(' ', "-"),
                claim: c.claim,
                confidence,
                evidence_refs: c.evidence_refs,
                asserted_by: persona_id.to_string(),
                confirmed_by: Vec::new(),
                contested_by: Vec::new(),
                contradicts,
                created_at: now,
                updated_at: now,
                review_after: now + Duration::hours(1),
                status: BeliefStatus::Active,
            }
        })
        .collect()
}
