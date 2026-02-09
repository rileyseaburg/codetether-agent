//! Shared Belief Store — structured extraction, canonical keys, decay, and confidence policy.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::thinker::ThinkerClient;

/// Status of a belief in the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BeliefStatus {
    Active,
    Stale,
    Invalidated,
    Archived,
}

/// A structured belief in the shared store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub id: String,
    /// Normalized canonical key — prevents duplicate beliefs.
    pub belief_key: String,
    pub claim: String,
    /// Runtime-bounded confidence in [0.05, 0.95].
    pub confidence: f32,
    /// Event IDs that serve as evidence.
    pub evidence_refs: Vec<String>,
    pub asserted_by: String,
    pub confirmed_by: Vec<String>,
    pub contested_by: Vec<String>,
    /// IDs of beliefs this contradicts.
    pub contradicts: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Staleness expiry — default: created_at + 1h.
    pub review_after: DateTime<Utc>,
    pub status: BeliefStatus,
}

impl Belief {
    /// Clamp confidence to the valid range [0.05, 0.95].
    pub fn clamp_confidence(&mut self) {
        self.confidence = self.confidence.clamp(0.05, 0.95);
    }

    /// Apply revalidation success: increase confidence by 0.15, capped at 0.95.
    pub fn revalidation_success(&mut self) {
        self.confidence = (self.confidence + 0.15).min(0.95);
        self.status = BeliefStatus::Active;
        self.updated_at = Utc::now();
        self.review_after = Utc::now() + Duration::hours(1);
    }

    /// Apply revalidation failure: decrease confidence by 0.25, min 0.05.
    pub fn revalidation_failure(&mut self) {
        self.confidence = (self.confidence - 0.25).max(0.05);
        self.updated_at = Utc::now();
        if self.confidence < 0.5 {
            self.status = BeliefStatus::Stale;
        }
    }

    /// Apply staleness decay: multiply confidence by 0.98.
    pub fn decay(&mut self) {
        self.confidence *= 0.98;
        self.confidence = self.confidence.max(0.05);
        self.updated_at = Utc::now();
    }
}

/// Raw claim extracted from LLM structured output.
#[derive(Debug, Clone, Deserialize)]
struct ExtractedClaim {
    claim: String,
    belief_key: String,
    confidence: f32,
    #[serde(default)]
    evidence_refs: Vec<String>,
    #[serde(default)]
    contest_target: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    uncertainties: Vec<String>,
}

/// Wrapper for the structured extraction response.
#[derive(Debug, Deserialize)]
struct ExtractionResponse {
    claims: Vec<ExtractedClaim>,
}

/// Extract beliefs from a thought using structured LLM extraction.
///
/// Returns a Vec of new Belief structs. The caller handles duplicate detection
/// and insertion into the belief store.
pub async fn extract_beliefs_from_thought(
    thinker: Option<&ThinkerClient>,
    persona_id: &str,
    thought_text: &str,
) -> Vec<Belief> {
    let Some(client) = thinker else {
        return Vec::new();
    };

    let system_prompt = "You are a structured belief extractor. \
Given a thought, extract concrete factual claims as structured JSON. \
Return ONLY valid JSON, no markdown fences. \
If no concrete claims exist, return {\"claims\":[]}."
        .to_string();

    let user_prompt = format!(
        "Extract beliefs from this thought:\n\n{thought}\n\n\
Return JSON only: {{ \"claims\": [{{ \"claim\": \"...\", \"belief_key\": \"lowercase-normalized-key\", \
\"confidence\": 0.0-1.0, \"evidence_refs\": [], \"contest_target\": null, \
\"uncertainties\": [] }}] }}",
        thought = thought_text
    );

    let output = match client.think(&system_prompt, &user_prompt).await {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    // Strip markdown code fences if present
    let text = output
        .text
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_clamping() {
        let mut belief = Belief {
            id: "b1".to_string(),
            belief_key: "test-key".to_string(),
            claim: "test claim".to_string(),
            confidence: 1.5,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        };
        belief.clamp_confidence();
        assert!((belief.confidence - 0.95).abs() < f32::EPSILON);

        belief.confidence = -0.5;
        belief.clamp_confidence();
        assert!((belief.confidence - 0.05).abs() < f32::EPSILON);
    }

    #[test]
    fn revalidation_success_increases_confidence() {
        let mut belief = Belief {
            id: "b1".to_string(),
            belief_key: "test-key".to_string(),
            claim: "test".to_string(),
            confidence: 0.6,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Stale,
        };
        belief.revalidation_success();
        assert!((belief.confidence - 0.75).abs() < f32::EPSILON);
        assert_eq!(belief.status, BeliefStatus::Active);
    }

    #[test]
    fn revalidation_failure_decreases_confidence() {
        let mut belief = Belief {
            id: "b1".to_string(),
            belief_key: "test-key".to_string(),
            claim: "test".to_string(),
            confidence: 0.6,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        };
        belief.revalidation_failure();
        assert!((belief.confidence - 0.35).abs() < f32::EPSILON);
        assert_eq!(belief.status, BeliefStatus::Stale);
    }

    #[test]
    fn decay_reduces_confidence() {
        let mut belief = Belief {
            id: "b1".to_string(),
            belief_key: "test-key".to_string(),
            claim: "test".to_string(),
            confidence: 0.5,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        };
        belief.decay();
        assert!((belief.confidence - 0.49).abs() < f32::EPSILON);
    }

    #[test]
    fn duplicate_detection_by_belief_key() {
        use std::collections::HashMap;

        let mut store: HashMap<String, Belief> = HashMap::new();
        let belief = Belief {
            id: "b1".to_string(),
            belief_key: "api-latency-high".to_string(),
            claim: "API latency is above SLA".to_string(),
            confidence: 0.7,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        };
        store.insert(belief.id.clone(), belief);

        // Check duplicate detection
        let existing = store
            .values()
            .find(|b| b.belief_key == "api-latency-high" && b.status != BeliefStatus::Invalidated);
        assert!(existing.is_some());
    }

    #[test]
    fn contradiction_tracking() {
        let mut b1 = Belief {
            id: "b1".to_string(),
            belief_key: "api-stable".to_string(),
            claim: "API is stable".to_string(),
            confidence: 0.8,
            evidence_refs: Vec::new(),
            asserted_by: "p1".to_string(),
            confirmed_by: Vec::new(),
            contested_by: Vec::new(),
            contradicts: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            review_after: Utc::now() + Duration::hours(1),
            status: BeliefStatus::Active,
        };

        // Contest reduces confidence
        b1.confidence = (b1.confidence - 0.1).max(0.05);
        b1.contested_by.push("b2".to_string());
        assert!((b1.confidence - 0.7).abs() < f32::EPSILON);
        assert_eq!(b1.contested_by, vec!["b2".to_string()]);
    }

    #[tokio::test]
    async fn extract_beliefs_no_thinker_returns_empty() {
        let result = extract_beliefs_from_thought(None, "p1", "some thought").await;
        assert!(result.is_empty());
    }
}
