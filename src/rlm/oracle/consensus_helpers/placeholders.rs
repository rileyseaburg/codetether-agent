//! Placeholder trace builders and git revision helpers.
//!
//! Used by consensus validation when a real analysis result
//! is not yet available, and for embedding the repo SHA in
//! every trace record.
//!
//! # Examples
//!
//! ```ignore
//! let trace = build_placeholder_trace();
//! assert_eq!(trace.verdict, "placeholder");
//! ```

use super::super::trace_types::ValidatedTrace;
use super::super::types::VerificationMethod;

/// Create an empty [`ValidatedTrace`] with verdict
/// `"placeholder"` for consensus scaffolding.
///
/// All counters are zeroed and the prompt is blank.  The
/// `repo_revision` is set from [`get_git_revision`].
///
/// # Examples
///
/// ```ignore
/// let t = build_placeholder_trace();
/// assert_eq!(t.verdict, "placeholder");
/// ```
pub fn build_placeholder_trace() -> ValidatedTrace {
    ValidatedTrace {
        prompt: String::new(),
        trace: vec![],
        final_payload: None,
        verdict: "placeholder".into(),
        oracle_diff: None,
        repo_revision: get_git_revision(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        answer: String::new(),
        iterations: 0,
        subcalls: 0,
        input_tokens: 0,
        output_tokens: 0,
        elapsed_ms: 0,
        source_path: None,
        verification_method: VerificationMethod::None,
        trace_id: uuid::Uuid::new_v4().to_string(),
    }
}

/// Read the current `HEAD` SHA from `git rev-parse`.
///
/// Returns `"unknown"` if git is unavailable or the working
/// directory is not a repository.
///
/// # Examples
///
/// ```ignore
/// let rev = get_git_revision();
/// assert!(!rev.is_empty());
/// ```
pub fn get_git_revision() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
