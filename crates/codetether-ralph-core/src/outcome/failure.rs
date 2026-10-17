//! Choose the preserved failure explanation for a parallel story.
/// Keep a supplied reason or select the fallback for the model outcome.
///
/// # Arguments
/// `success` indicates model execution success; `failure_reason` is explicit evidence.
/// # Returns
/// The original reason or the established quality/model failure message.
/// # Examples
/// ```
/// assert_eq!(codetether_ralph_core::outcome::story_failure(false, None), "LLM call failed");
/// ```
pub fn story_failure(success: bool, failure_reason: Option<String>) -> String {
    failure_reason.unwrap_or_else(|| {
        if success {
            "Quality or verification checks failed".to_string()
        } else {
            "LLM call failed".to_string()
        }
    })
}
