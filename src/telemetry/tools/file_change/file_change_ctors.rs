//! Constructors for [`super::FileChange`]. Split out to keep `file_change.rs`
//! under the 50-line per-file limit.

use chrono::Utc;

use super::FileChange;

impl FileChange {
    /// Record a read. `line_range` is optional but recommended when only a
    /// slice of the file was read.
    pub fn read(path: &str, line_range: Option<(u32, u32)>) -> Self {
        Self {
            path: path.to_string(),
            operation: "read".to_string(),
            timestamp: Utc::now(),
            size_bytes: None,
            line_range,
            diff: None,
        }
    }

    /// Record file creation. `content` is used only to compute `size_bytes`.
    pub fn create(path: &str, content: &str) -> Self {
        Self {
            path: path.to_string(),
            operation: "create".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(content.len() as u64),
            line_range: None,
            diff: None,
        }
    }

    /// Record a modify with a coarse byte-count diff string.
    pub fn modify(
        path: &str,
        old_content: &str,
        new_content: &str,
        line_range: Option<(u32, u32)>,
    ) -> Self {
        Self {
            path: path.to_string(),
            operation: "modify".to_string(),
            timestamp: Utc::now(),
            size_bytes: Some(new_content.len() as u64),
            line_range,
            diff: Some(format!(
                "-{} bytes +{} bytes",
                old_content.len(),
                new_content.len()
            )),
        }
    }

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
