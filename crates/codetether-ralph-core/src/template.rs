//! Create a starting PRD with one placeholder story.
use crate::{Prd, QualityChecks, UserStory};
/// Create the standard sample PRD.
///
/// # Arguments
/// * `project` - Project name.
/// * `feature` - Feature title used to derive the branch.
/// # Returns
/// A timestamped PRD containing one story and standard Rust quality checks.
/// # Examples
/// ```
/// let prd = codetether_ralph_core::create_prd_template("project", "Feature Name");
/// assert_eq!(prd.branch_name, "feature/feature-name");
/// ```

/// Create a sample PRD template
pub fn create_prd_template(project: &str, feature: &str) -> Prd {
    Prd {
        project: project.to_string(),
        feature: feature.to_string(),
        branch_name: format!("feature/{}", feature.to_lowercase().replace(' ', "-")),
        version: "1.0".to_string(),
        user_stories: vec![UserStory {
            id: "US-001".to_string(),
            title: "First user story".to_string(),
            description: "Description of what needs to be implemented".to_string(),
            acceptance_criteria: vec!["Criterion 1".to_string(), "Criterion 2".to_string()],
            verification_steps: Vec::new(),
            passes: false,
            priority: 1,
            depends_on: Vec::new(),
            complexity: 3,
        }],
        technical_requirements: Vec::new(),
        quality_checks: QualityChecks {
            typecheck: Some("cargo check".to_string()),
            test: Some("cargo test".to_string()),
            lint: Some("cargo clippy".to_string()),
            build: Some("cargo build".to_string()),
        },
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }
}
