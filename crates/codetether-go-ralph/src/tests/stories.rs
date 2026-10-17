//! Original story fixtures for outcome mapping.

use codetether_ralph_core::UserStory;
pub(super) fn create() -> Vec<UserStory> {
    vec![
        UserStory {
            id: "US-001".to_string(),
            title: "Story one".to_string(),
            description: "First story".to_string(),
            acceptance_criteria: vec![],
            verification_steps: vec![],
            passes: true,
            priority: 1,
            depends_on: vec![],
            complexity: 2,
        },
        UserStory {
            id: "US-002".to_string(),
            title: "Story two".to_string(),
            description: "Second story".to_string(),
            acceptance_criteria: vec![],
            verification_steps: vec![],
            passes: false,
            priority: 2,
            depends_on: vec![],
            complexity: 3,
        },
    ]
}
