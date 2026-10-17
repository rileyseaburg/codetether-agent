//! project bucket policy implementation.
use super::*;

impl RelevanceMeta {
    /// Project the relevance signals onto a coarse CADMAS-CTX bucket.
    ///
    /// Heuristic (Phase B v1):
    ///
    /// | Bucket field     | Rule                                                |
    /// |------------------|-----------------------------------------------------|
    /// | `tool_use`       | `Yes` when [`Self::tools`] is non-empty             |
    /// | `dependency`     | `Chained` when ≥ 2 files, or any path has `/`       |
    /// | `difficulty`     | errors-per-entry ladder: 0 → Easy, 1–2 → Medium, ≥3 → Hard |
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_context_policy::relevance::{
    ///     Dependency, Difficulty, RelevanceMeta, ToolUse,
    /// };
    ///
    /// let meta = RelevanceMeta {
    ///     files: vec!["src/a.rs".into(), "tests/b.rs".into()],
    ///     tools: vec!["Shell".into()],
    ///     error_classes: vec!["Error:".into(), "panicked".into(), "failed".into()],
    ///     explicit_refs: Vec::new(),
    /// };
    /// let bucket = meta.project_bucket();
    /// assert_eq!(bucket.tool_use, ToolUse::Yes);
    /// assert_eq!(bucket.dependency, Dependency::Chained);
    /// assert_eq!(bucket.difficulty, Difficulty::Hard);
    /// ```
    pub fn project_bucket(&self) -> Bucket {
        let tool_use = if self.tools.is_empty() {
            ToolUse::No
        } else {
            ToolUse::Yes
        };
        let dependency = if self.files.len() >= 2 || self.files.iter().any(|f| f.contains('/')) {
            Dependency::Chained
        } else {
            Dependency::Isolated
        };
        let difficulty = match self.error_classes.len() {
            0 => Difficulty::Easy,
            1 | 2 => Difficulty::Medium,
            _ => Difficulty::Hard,
        };
        Bucket {
            difficulty,
            dependency,
            tool_use,
        }
    }
}
