//! Confidence clamps, revalidation and time-based decay.
use super::{Belief, BeliefStatus};
use chrono::{Duration, Utc};

impl Belief {
    /// Clamp confidence to the valid range [0.05, 0.95].
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
    /// let mut belief = parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service reachable","belief_key":"service-reachable","confidence":0.6}]}"#).remove(0);
    /// belief.clamp_confidence();
    /// assert!((0.05..=0.95).contains(&belief.confidence));
    /// ```
    pub fn clamp_confidence(&mut self) {
        self.confidence = self.confidence.clamp(0.05, 0.95);
    }

    /// Apply revalidation success: increase confidence by 0.15, capped at 0.95.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
    /// let mut belief = parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service reachable","belief_key":"service-reachable","confidence":0.6}]}"#).remove(0);
    /// belief.revalidation_success();
    /// assert_eq!(belief.status, codetether_cognition_core::beliefs::BeliefStatus::Active);
    /// ```
    pub fn revalidation_success(&mut self) {
        self.confidence = (self.confidence + 0.15).min(0.95);
        self.status = BeliefStatus::Active;
        self.updated_at = Utc::now();
        self.review_after = Utc::now() + Duration::hours(1);
    }

    /// Apply revalidation failure: decrease confidence by 0.25, min 0.05.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
    /// let mut belief = parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service reachable","belief_key":"service-reachable","confidence":0.6}]}"#).remove(0);
    /// belief.revalidation_failure();
    /// assert!(belief.confidence < 0.6);
    /// ```
    pub fn revalidation_failure(&mut self) {
        self.confidence = (self.confidence - 0.25).max(0.05);
        self.updated_at = Utc::now();
        if self.confidence < 0.5 {
            self.status = BeliefStatus::Stale;
        }
    }

    /// Apply staleness decay: multiply confidence by 0.98.
    ///
    /// # Examples
    /// ```
    /// use codetether_cognition_core::beliefs::extraction::parse_extracted_beliefs;
    /// let mut belief = parse_extracted_beliefs("p1", r#"{"claims":[{"claim":"Service reachable","belief_key":"service-reachable","confidence":0.6}]}"#).remove(0);
    /// belief.decay();
    /// assert!(belief.confidence < 0.6);
    /// ```
    pub fn decay(&mut self) {
        self.confidence *= 0.98;
        self.confidence = self.confidence.max(0.05);
        self.updated_at = Utc::now();
    }
}
