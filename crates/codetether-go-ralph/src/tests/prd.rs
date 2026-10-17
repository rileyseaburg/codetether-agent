//! Original PRD fixture for outcome mapping.

use codetether_ralph_core::{Prd, QualityChecks};
pub(super) fn create() -> Prd {
    Prd {
        project: "test".to_string(),
        feature: "test-feature".to_string(),
        branch_name: "feature/test".to_string(),
        version: "1.0".to_string(),
        user_stories: super::stories::create(),
        technical_requirements: vec![],
        quality_checks: QualityChecks::default(),
        created_at: String::new(),
        updated_at: String::new(),
    }
}
