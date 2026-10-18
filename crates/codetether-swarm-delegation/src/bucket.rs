//! Common specialty projection for provider ranking and feedback.
use codetether_context_policy::relevance::{Bucket, Dependency, Difficulty, ToolUse};
/// Compute the shared calibration bucket for a swarm specialty.
/// # Arguments
/// `specialty` is case-insensitive task-role text.
/// # Returns
/// A bucket with isolated dependency context and the matching difficulty/tool use.
/// # Examples
/// ```
/// let bucket = codetether_swarm_delegation::bucket_for_specialty("security review");
/// assert!(matches!(bucket.difficulty, codetether_context_policy::relevance::Difficulty::Hard));
/// ```
pub fn bucket_for_specialty(specialty: &str) -> Bucket {
    let s = specialty.to_ascii_lowercase();
    let difficulty = if s.contains("security") || s.contains("architect") {
        Difficulty::Hard
    } else if s.contains("review") || s.contains("test") {
        Difficulty::Medium
    } else {
        Difficulty::Easy
    };
    let tool_use = if s.contains("deploy") || s.contains("infra") || s.contains("debug") {
        ToolUse::Yes
    } else {
        ToolUse::No
    };
    Bucket {
        difficulty,
        dependency: Dependency::Isolated,
        tool_use,
    }
}
