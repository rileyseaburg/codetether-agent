//! Existing completed-result presentation regression test.

use crate::{GoRalphResult, StoryResult, format_go_ralph_result};
use codetether_ralph_core::RalphStatus;
use std::path::PathBuf;

#[test]
fn format_result_shows_status() {
    let result = GoRalphResult {
        prd_path: PathBuf::from("prd_test.json"),
        feature_branch: "feature/test".to_string(),
        passed: 2,
        total: 3,
        all_passed: false,
        iterations: 5,
        max_iterations: 10,
        status: RalphStatus::MaxIterations,
        stories: vec![
            StoryResult {
                id: "US-001".to_string(),
                title: "Story one".to_string(),
                passed: true,
            },
            StoryResult {
                id: "US-002".to_string(),
                title: "Story two".to_string(),
                passed: true,
            },
            StoryResult {
                id: "US-003".to_string(),
                title: "Story three".to_string(),
                passed: false,
            },
        ],
    };

    let output = format_go_ralph_result(&result, "test task");
    assert!(output.contains("2/3 stories"));
    assert!(output.contains("US-003"));
    assert!(output.contains("Incomplete"));
}
