//! Per-tool filesystem operation records.
//!
//! A [`FileChange`] is attached to a [`super::ToolExecution`] so that
//! downstream consumers (audit log, TUI, persistent stats) can reconstruct
//! exactly which files each tool invocation touched and how.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single filesystem operation performed by a tool.
///
/// The `operation` field is a free-form string (`"read"`, `"create"`,
/// `"modify"`, …) chosen by the emitting tool. Use the helper constructors
/// below rather than building the struct by hand.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::FileChange;
///
/// let r = FileChange::read("src/main.rs", Some((1, 20)));
/// assert_eq!(r.operation, "read");
/// assert!(r.summary().contains("src/main.rs"));
///
/// let c = FileChange::create("new.txt", "hello");
/// assert_eq!(c.size_bytes, Some(5));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// Filesystem path the tool touched.
    pub path: String,
    /// Operation name (`"read"` / `"create"` / `"modify"` / …).
    pub operation: String,
    /// When the operation happened.
    pub timestamp: DateTime<Utc>,
    /// Post-operation size in bytes, when known.
    pub size_bytes: Option<u64>,
    /// `(start_line, end_line)` the operation applied to, when known.
    pub line_range: Option<(u32, u32)>,
    /// Unified-diff or free-form diff text, when available.
    pub diff: Option<String>,
}

mod file_change_ctors;

impl FileChange {
    /// One-line `"path (op)"` summary suitable for logs and the TUI.
    pub fn summary(&self) -> String {
        format!("{} ({})", self.path, self.operation)
    }
}
