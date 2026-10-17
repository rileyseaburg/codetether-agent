//! Prd serializable contract.
use super::*;

/// The full PRD structure
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let prd = codetether_ralph_core::Prd { project: "sample".into(), feature: "example".into(), branch_name: String::new(), version: "1.0".into(), user_stories: vec![], technical_requirements: vec![], quality_checks: Default::default(), created_at: String::new(), updated_at: String::new() };
/// assert!(prd.is_complete());
/// ```
pub struct Prd {
    /// Project name
    pub project: String,

    /// Feature being implemented
    pub feature: String,

    /// Git branch name for this PRD
    #[serde(default)]
    pub branch_name: String,

    /// Version of the PRD format
    #[serde(default = "default_version")]
    pub version: String,

    /// User stories to implement
    #[serde(default)]
    pub user_stories: Vec<UserStory>,

    /// Technical requirements
    #[serde(default)]
    pub technical_requirements: Vec<String>,

    /// Quality checks to run
    #[serde(default)]
    pub quality_checks: QualityChecks,

    /// Created timestamp
    #[serde(default)]
    pub created_at: String,

    /// Last updated timestamp
    #[serde(default)]
    pub updated_at: String,
}
