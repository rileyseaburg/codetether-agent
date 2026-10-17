//! Per-story execution outcome.

/// Outcome of one generated story.
///
/// # Examples
/// ```
/// let story = codetether_go_ralph::StoryResult {
///     id: "US-001".into(), title: "Implement feature".into(), passed: true,
/// };
/// assert!(story.passed);
/// ```
#[derive(Debug, Clone)]
pub struct StoryResult {
    /// Identifier from the PRD.
    pub id: String,
    /// Human-readable story title.
    pub title: String,
    /// Whether the story passed verification.
    pub passed: bool,
}
