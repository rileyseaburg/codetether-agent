use super::run_story_verification;
use crate::approval::test_env::lock_env;
use crate::ralph::types::{UserStory, VerificationStep};
use tempfile::tempdir;

#[path = "tests_shell_env.rs"]
mod shell_env;

#[tokio::test]
async fn shell_step_checks_output_and_artifacts() {
    let _lock = lock_env();
    let dir = tempdir().expect("tempdir");
    let _env = shell_env::TrustedShellProject::new(dir.path());
    let step = VerificationStep::Shell {
        name: None,
        command: "mkdir -p out && echo ready > out/result.txt && echo ok".into(),
        cwd: None,
        expect_output_contains: vec!["ok".into()],
        expect_files_glob: vec!["out/*.txt".into()],
    };
    assert!(
        run_story_verification(dir.path(), &story(step))
            .await
            .is_ok()
    );
}

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
