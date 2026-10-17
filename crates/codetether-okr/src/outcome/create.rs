//! KrOutcome create behavior.

use crate::KrOutcome;
use crate::KrOutcomeType;
use chrono::Utc;
use uuid::Uuid;
impl KrOutcome {
    /// Create a new outcome
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KrOutcome};
    /// let value = KrOutcome::new(uuid::Uuid::nil(), "Passing checks");
    /// assert_eq!(value.description, "Passing checks");
    /// ```
    pub fn new(kr_id: Uuid, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            kr_id,
            run_id: None,
            description: description.into(),
            outcome_type: KrOutcomeType::Evidence,
            value: None,
            evidence: Vec::new(),
            source: String::new(),
            created_at: Utc::now(),
        }
    }
}
