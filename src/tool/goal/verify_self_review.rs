//! Detection of a verifier grading its own worker's output.
//!
//! A verifier running on the worker's own model is a weak second opinion:
//! both share the same blind spots. This module flags that case so it is
//! surfaced in logs and in the verdict record.

/// Whether the verifier and worker resolve to the same model.
///
/// Comparison ignores surrounding whitespace, ASCII case, and a leading
/// `provider/` segment, so `bedrock/x` and `x` match.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::is_self_review;
///
/// assert!(is_self_review("bedrock/m1", Some("M1")));
/// assert!(!is_self_review("bedrock/m1", Some("bedrock/m2")));
/// assert!(!is_self_review("bedrock/m1", None));
/// ```
pub fn is_self_review(verifier: &str, worker: Option<&str>) -> bool {
    worker.is_some_and(|worker| bare(verifier) == bare(worker))
}

fn bare(model: &str) -> String {
    let model = model.trim();
    let model = model.split_once('/').map_or(model, |(_, id)| id);
    model.to_ascii_lowercase()
}

/// Log a warning when the verifier would review its own worker.
///
/// # Returns
///
/// `true` when the review is a self-review.
pub fn warn_if_self_review(verifier: &str, worker: Option<&str>) -> bool {
    let same = is_self_review(verifier, worker);
    if same {
        tracing::warn!(
            verifier = %verifier,
            env = super::VERIFIER_MODEL_ENV,
            "Goal verifier is using the worker's own model; set a different verifier model"
        );
    }
    same
}
