//! Focused operations for the parent telemetry type.

use super::*;

impl FileChange {
    /// Record a modify with a caller-provided unified diff.
    pub fn modify_with_diff(
        path: &str,
        diff: &str,
        new_size: usize,
        line_range: Option<(u32, u32)>,
    ) -> Self {
        Self {
            path: path.to_string(),
            operation: "modify".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(new_size as u64),
            line_range,
            diff: Some(diff.to_string()),
        }
    }
}
