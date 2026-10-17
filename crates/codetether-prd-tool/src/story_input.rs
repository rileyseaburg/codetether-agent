//! story input for PRD construction and validation.
use super::*;

#[derive(Deserialize)]
pub(super) struct StoryInput {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) description: String,

    #[serde(default)]
    pub(super) acceptance_criteria: Vec<String>,

    /// Optional explicit story verification (BDD/TDD/E2E evidence).
    #[serde(default)]
    pub(super) verification_steps: Vec<VerificationStep>,

    #[serde(default)]
    pub(super) priority: Option<u8>,

    #[serde(default)]
    pub(super) depends_on: Vec<String>,

    #[serde(default)]
    pub(super) complexity: Option<u8>,
}
