//! params for PRD construction and validation.
use super::*;

#[derive(Deserialize)]
pub(super) struct Params {
    pub(super) action: String,

    #[serde(default)]
    pub(super) task_description: Option<String>,

    #[serde(default)]
    pub(super) project: Option<String>,

    #[serde(default)]
    pub(super) feature: Option<String>,

    #[serde(default)]
    pub(super) stories: Option<Vec<StoryInput>>,

    #[serde(default)]
    pub(super) quality_checks: Option<QualityChecksInput>,

    #[serde(default)]
    pub(super) prd_path: Option<String>,

    /// Raw PRD JSON string (optional alternative to `prd_path`).
    #[serde(default)]
    pub(super) prd_json: Option<String>,
}
