//! quality input for PRD construction and validation.
use super::*;

#[derive(Deserialize)]
pub(super) struct QualityChecksInput {
    #[serde(default)]
    pub(super) typecheck: Option<String>,
    #[serde(default)]
    pub(super) test: Option<String>,
    #[serde(default)]
    pub(super) lint: Option<String>,
    #[serde(default)]
    pub(super) build: Option<String>,
}
