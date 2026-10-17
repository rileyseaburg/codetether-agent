//! relevance contracts and policy operations.
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
//! use codetether_provider_core::{ContentPart, Message, Role};
//! use codetether_context_policy::relevance::{Bucket, Dependency, Difficulty, RelevanceMeta, ToolUse, extract};
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

use codetether_provider_core::{ContentPart, Message};

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
mod type_relevance_meta;
pub use type_relevance_meta::RelevanceMeta;
mod type_difficulty;
pub use type_difficulty::Difficulty;
mod impl_difficulty_fn_0;
mod type_dependency;
pub use type_dependency::Dependency;
mod impl_dependency_fn_1;
mod type_tool_use;
pub use type_tool_use::ToolUse;
mod impl_tool_use_fn_2;
mod type_bucket;
pub use type_bucket::Bucket;
mod fn_extract;
mod impl_relevance_meta_project_bucket_3;
pub use fn_extract::extract;
mod fn_bucket_for_messages;
pub use fn_bucket_for_messages::bucket_for_messages;
mod fn_append_files;
use fn_append_files::append_files;
mod fn_ends_with_source_ext;
use fn_ends_with_source_ext::ends_with_source_ext;
mod fn_append_error_classes;
use fn_append_error_classes::append_error_classes;
mod fn_dedupe_preserving_order;
use fn_dedupe_preserving_order::dedupe_preserving_order;
#[cfg(test)]
mod tests;
