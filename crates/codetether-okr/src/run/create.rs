//! OkrRun create behavior.

use crate::OkrRun;
use crate::OkrRunStatus;
use chrono::Utc;
use uuid::Uuid;
impl OkrRun {
    /// Create a new OKR run
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// assert_eq!(value.name, "Quarterly run");
    /// ```
    pub fn new(okr_id: Uuid, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            okr_id,
            name: name.into(),
            status: OkrRunStatus::Draft,
            correlation_id: None,
            relay_checkpoint_id: None,
            session_id: None,
            kr_progress: std::collections::HashMap::new(),
            approval: None,
            outcomes: Vec::new(),
            iterations: 0,
            started_at: now,
            completed_at: None,
            updated_at: now,
        }
    }
}
