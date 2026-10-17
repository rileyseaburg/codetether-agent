//! Shared original test fixtures.
use super::*;

pub(super) fn test_prd() -> Prd {
    Prd {
        project: "test".to_string(),
        feature: "test-feature".to_string(),
        branch_name: "feature/test".to_string(),
        version: "1.0".to_string(),
        user_stories: vec![UserStory {
            id: "US-001".to_string(),
            title: "Test story".to_string(),
            description: "A test story".to_string(),
            acceptance_criteria: vec![],
            verification_steps: vec![],
            passes: false,
            priority: 1,
            depends_on: vec![],
            complexity: 2,
        }],
        technical_requirements: vec![],
        quality_checks: QualityChecks::default(),
        created_at: String::new(),
        updated_at: String::new(),
    }
}

pub(super) fn test_run_state() -> RalphRunState {
    RalphRunState {
        run_id: "test-run-1".to_string(),
        okr_id: Some("okr-1".to_string()),
        prd: test_prd(),
        config: RalphConfig::default(),
        status: RalphStatus::Pending,
        current_iteration: 0,
        max_iterations: 10,
        progress_log: vec![],
        story_results: vec![],
        error: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        started_at: None,
        completed_at: None,
    }
}
