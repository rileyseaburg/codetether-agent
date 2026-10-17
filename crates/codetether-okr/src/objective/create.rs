//! Okr create behavior.

use crate::Okr;
use crate::OkrStatus;
use chrono::Utc;
use uuid::Uuid;
impl Okr {
    /// Create a new OKR with a generated UUID
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{Okr};
    /// let mut value = Okr::new("Reliability", "Reduce failures");
    /// assert_eq!(value.title, "Reliability");
    /// ```
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            description: description.into(),
            status: OkrStatus::Draft,
            key_results: Vec::new(),
            owner: None,
            tenant_id: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            target_date: None,
        }
    }
}
