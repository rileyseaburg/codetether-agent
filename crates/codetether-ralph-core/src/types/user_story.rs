//! UserStory serializable contract.
use super::*;

/// A user story in the PRD
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let story = codetether_ralph_core::UserStory { id: "US-1".into(), title: "Example".into(), description: String::new(), acceptance_criteria: vec![], verification_steps: vec![], passes: false, priority: 1, depends_on: vec![], complexity: 3 };
/// assert!(!story.passes);
/// ```
pub struct UserStory {
    /// Unique identifier (e.g., "US-001")
    pub id: String,

    /// Short title
    pub title: String,

    /// Full description
    pub description: String,

    /// Acceptance criteria
    #[serde(default)]
    pub acceptance_criteria: Vec<String>,

    /// Explicit verification steps that must pass for this story to be marked complete.
    ///
    /// Example: run Cypress E2E and assert that a video artifact exists.
    #[serde(default)]
    pub verification_steps: Vec<VerificationStep>,

    /// Whether this story passes all tests
    #[serde(default)]
    pub passes: bool,

    /// Story priority (1=highest)
    #[serde(default = "default_priority")]
    pub priority: u8,

    /// Dependencies on other story IDs
    #[serde(default, alias = "dependencies")]
    pub depends_on: Vec<String>,

    /// Estimated complexity (1-5)
    #[serde(default = "default_complexity")]
    pub complexity: u8,
}
