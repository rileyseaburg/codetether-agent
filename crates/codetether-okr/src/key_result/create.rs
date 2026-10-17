//! KeyResult create behavior.

use crate::KeyResult;
use crate::{KeyResultStatus, KrMetricType};
use chrono::Utc;
use uuid::Uuid;
impl KeyResult {
    /// Create a new key result
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// assert_eq!(value.current_value, 0.0);
    /// ```
    pub fn new(
        okr_id: Uuid,
        title: impl Into<String>,
        target_value: f64,
        unit: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            okr_id,
            title: title.into(),
            description: String::new(),
            target_value,
            current_value: 0.0,
            unit: unit.into(),
            metric_type: KrMetricType::Progress,
            status: KeyResultStatus::Pending,
            outcomes: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}
