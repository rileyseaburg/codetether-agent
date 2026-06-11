use super::run_story_verification;
use crate::ralph::types::{UserStory, VerificationStep};
use tempfile::tempdir;

fn story(step: VerificationStep) -> UserStory {
    UserStory {
        id: "US-1".into(),
        title: "story".into(),
        description: "desc".into(),
        acceptance_criteria: vec![],
        verification_steps: vec![step],
        passes: false,
        priority: 1,
        depends_on: vec![],
        complexity: 1,
    }
}

#[tokio::test]
async fn failing_step_blocks_completion_gate() {
    let dir = tempdir().expect("tempdir");
    let step = VerificationStep::FileExists {
        name: None,
        path: "missing.txt".into(),
        glob: false,
    };
    assert!(
        run_story_verification(dir.path(), &story(step))
            .await
            .is_err()
    );
}
