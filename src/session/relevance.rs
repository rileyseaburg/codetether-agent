//! Per-entry relevance metadata and CADMAS-CTX bucket projection.
//!
//! ## Phase B foundation
//!
//! The Liu et al. paper (arXiv:2512.22087) calls for a per-entry
//! sidecar of relevance signals that an incremental derivation policy
//! can score against the current task. The CADMAS-CTX paper
//! (arXiv:2604.17950) independently needs a coarse **context bucket**
//! `z = (difficulty, dependency, tool_use)` to key its per-(agent,
//! skill, bucket) posteriors.
//!
//! Both consumers share ~80 % of the extraction work — file paths,
//! tool names, error-class markers — so this module emits a single
//! [`RelevanceMeta`] that projects down to a [`Bucket`] via
//! [`RelevanceMeta::project_bucket`]. Phase B's `DerivePolicy::Incremental`
//! and Phase C's `DelegationState` both read from the same sidecar.
//!
//! ## Scope in Phase B step 15
//!
//! Extraction is **pure and syntactic** — no LLM calls, no IO. Heuristics:
//!
//! * `files`: regex over text parts for path-like tokens.
//! * `tools`: names of `ToolCall` / `ToolResult` content parts.
//! * `error_classes`: leading tokens of common error markers
//!   (`Error:`, `error[E`, `failed`, `panicked`, `traceback`).
//! * `explicit_refs`: left for future turns-N-reference extraction
//!   (empty in this first cut).
//!
//! Keeping it syntactic means the extractor can run in the append hot
//! path (`Session::add_message`) without blocking.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::provider::{ContentPart, Message, Role};
//! use codetether_agent::session::relevance::{Bucket, Dependency, Difficulty, RelevanceMeta, ToolUse, extract};
//!
//! let msg = Message {
//!     role: Role::Assistant,
//!     content: vec![ContentPart::Text {
//!         text: "Edited src/lib.rs and tests/smoke.rs".to_string(),
//!     }],
//! };
//! let meta: RelevanceMeta = extract(&msg);
//! assert_eq!(meta.files.len(), 2);
//!
//! let bucket: Bucket = meta.project_bucket();
//! assert_eq!(bucket.tool_use, ToolUse::No);
//! assert_eq!(bucket.dependency, Dependency::Chained);
//! assert_eq!(bucket.difficulty, Difficulty::Easy);
//! ```

use serde::{Deserialize, Serialize};

use crate::provider::{ContentPart, Message};

/// Per-entry syntactic relevance signals.
///
/// Parallel-array friendly: one [`RelevanceMeta`] per entry in
/// [`Session::messages`](crate::session::Session). Lives in a
/// `<session-id>.relevance.jsonl` sidecar once wired into
/// `Session::save` (a future commit).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelevanceMeta {
    /// Path-like tokens found in the message's text parts.
    #[serde(default)]
    pub files: Vec<String>,
    /// Names of any `ToolCall` or `ToolResult` content parts.
    #[serde(default)]
    pub tools: Vec<String>,
    /// Short error-class tags surfaced by common error markers
    /// (`Error:`, `error[E`, `failed`, `panicked`, `Traceback`).
    #[serde(default)]
    pub error_classes: Vec<String>,
    /// Message indices this entry explicitly references
    /// (reserved for future N-back extraction; empty in Phase B v1).
    #[serde(default)]
    pub explicit_refs: Vec<usize>,
}

/// CADMAS-CTX difficulty axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// Stable snake_case encoding. Never renamed — used as part of the
    /// persisted [`crate::session::delegation::DelegationState`] key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}

/// CADMAS-CTX dependency axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dependency {
    /// Single file or a small set of files in the same module.
    Isolated,
    /// Cross-module / multi-file reach.
    Chained,
}

impl Dependency {
    /// Stable snake_case encoding — see [`Difficulty::as_str`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Dependency::Isolated => "isolated",
            Dependency::Chained => "chained",
        }
    }
}

/// CADMAS-CTX tool-use axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolUse {
    No,
    Yes,
}

impl ToolUse {
    /// Stable snake_case encoding — see [`Difficulty::as_str`].
    pub const fn as_str(self) -> &'static str {
        match self {
            ToolUse::No => "no",
            ToolUse::Yes => "yes",
        }
    }
}

/// Coarse context bucket — CADMAS-CTX Section 3.1.
///
/// Start with 3–4 active cells in practice (the paper shows over-
/// bucketing hurts — bias-variance collapses at 12 cells on GAIA).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bucket {
    pub difficulty: Difficulty,
    pub dependency: Dependency,
    pub tool_use: ToolUse,
}

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
    /// use codetether_agent::session::relevance::{
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

/// Short list of error-marker prefixes (lower-cased) we match literally.
///
/// Kept tiny and conservative — false positives in Phase C's delegation
/// posteriors are more expensive than false negatives.
const ERROR_MARKERS: &[&str] = &[
    "error:",
    "error[e",
    "failed",
    "panicked",
    "traceback",
    "stack trace",
];

/// Extract [`RelevanceMeta`] for a single chat-history entry.
///
/// Pure and fast: no LLM calls, no IO. Safe to call from
/// [`Session::add_message`](crate::session::Session::add_message).
pub fn extract(msg: &Message) -> RelevanceMeta {
    let mut meta = RelevanceMeta::default();
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                append_files(text, &mut meta.files);
                append_error_classes(text, &mut meta.error_classes);
            }
            ContentPart::ToolCall { name, .. } => {
                if !meta.tools.contains(name) {
                    meta.tools.push(name.clone());
                }
            }
            ContentPart::ToolResult { content, .. } => {
                append_error_classes(content, &mut meta.error_classes);
            }
            _ => {}
        }
    }
    dedupe_preserving_order(&mut meta.files);
    dedupe_preserving_order(&mut meta.error_classes);
    meta
}

/// Extract path-like tokens from `text` and append unique ones to `out`.
///
/// Heuristic: tokens that look like filesystem paths (contain `/` but
/// not `://`, so URLs are excluded) or end with a common source-file
/// extension. Intentionally conservative — this feeds
/// [`Bucket`] projection, so false positives directly harm delegation
/// calibration (over-reporting `Chained` dependency).
fn append_files(text: &str, out: &mut Vec<String>) {
    for raw in text.split(|c: char| c.is_whitespace() || matches!(c, ',' | ';' | '(' | ')' | '`')) {
        let trimmed = raw.trim_matches(|c: char| matches!(c, '"' | '\'' | '.'));
        if trimmed.is_empty() || trimmed.len() < 3 {
            continue;
        }
        let looks_like_path =
            (trimmed.contains('/') && !trimmed.contains("://") && trimmed.len() > 3)
                || ends_with_source_ext(trimmed);
        if looks_like_path && !out.contains(&trimmed.to_string()) {
            out.push(trimmed.to_string());
        }
    }
}

fn ends_with_source_ext(s: &str) -> bool {
    [
        ".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go", ".md", ".json", ".toml", ".yaml",
        ".yml", ".html", ".css", ".c", ".cpp", ".h", ".hpp",
    ]
    .iter()
    .any(|ext| s.ends_with(ext))
}

fn append_error_classes(text: &str, out: &mut Vec<String>) {
    let lower = text.to_lowercase();
    for marker in ERROR_MARKERS {
        if lower.contains(marker) {
            let tag = marker.trim_end_matches(':').to_string();
            if !out.contains(&tag) {
                out.push(tag);
            }
        }
    }
}

fn dedupe_preserving_order(items: &mut Vec<String>) {
    let mut seen: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(items.len());
    items.retain(|item| seen.insert(item.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Role};

    fn text(s: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: s.to_string(),
            }],
        }
    }

    fn tool_call(name: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call-1".to_string(),
                name: name.to_string(),
                arguments: "{}".to_string(),
                thought_signature: None,
            }],
        }
    }

    fn tool_result(body: &str) -> Message {
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call-1".to_string(),
                content: body.to_string(),
            }],
        }
    }

    #[test]
    fn extract_picks_up_paths_and_dedupes() {
        let meta = extract(&text(
            "Edited src/lib.rs and src/lib.rs again, plus tests/a.rs",
        ));
        assert_eq!(meta.files.len(), 2);
        assert!(meta.files.contains(&"src/lib.rs".to_string()));
        assert!(meta.files.contains(&"tests/a.rs".to_string()));
    }

    #[test]
    fn extract_recognises_source_extensions_without_slash() {
        let meta = extract(&text("check lib.rs and index.tsx"));
        assert!(meta.files.iter().any(|f| f == "lib.rs"));
        assert!(meta.files.iter().any(|f| f == "index.tsx"));
    }

    #[test]
    fn extract_captures_tool_names_from_tool_calls() {
        let meta = extract(&tool_call("Shell"));
        assert_eq!(meta.tools, vec!["Shell".to_string()]);
    }

    #[test]
    fn extract_tags_error_markers_from_tool_results() {
        let meta = extract(&tool_result(
            "Error: file not found\n  panicked at main.rs:12",
        ));
        assert!(meta.error_classes.contains(&"error".to_string()));
        assert!(meta.error_classes.contains(&"panicked".to_string()));
    }

    #[test]
    fn project_bucket_maps_axes_correctly() {
        let meta = RelevanceMeta {
            files: vec!["src/a.rs".into()],
            tools: Vec::new(),
            error_classes: Vec::new(),
            explicit_refs: Vec::new(),
        };
        let bucket = meta.project_bucket();
        assert_eq!(bucket.tool_use, ToolUse::No);
        assert_eq!(bucket.dependency, Dependency::Chained); // single path contains '/'
        assert_eq!(bucket.difficulty, Difficulty::Easy);
    }

    #[test]
    fn project_bucket_escalates_difficulty_with_error_count() {
        let meta = RelevanceMeta {
            error_classes: vec!["error".into(), "failed".into(), "panicked".into()],
            ..Default::default()
        };
        assert_eq!(meta.project_bucket().difficulty, Difficulty::Hard);
    }
}
