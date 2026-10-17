//! reviewer relay role prompt and capabilities.
use std::path::Path;
pub(super) fn profile(working_dir: &Path) -> (String, String, Vec<String>) {
    let wd = working_dir.display();
    (
        "auto-reviewer".to_string(),
        format!(
            "You are @auto-reviewer.\n\
                     Specialty: Code review and quality verification.\n\
                     Mission: Review implemented changes, run quality checks, fix remaining issues.\n\
                     Working directory: {wd}\n\n\
                     Review the code changes from the incoming handoff. Run `bash` with `cargo check 2>&1` (cwd: {wd}).\n\
                     Fix any remaining issues. Verify the implementation meets the acceptance criteria.\n\
                     If complete, include STORY_COMPLETE in your output. If blocked, include STORY_BLOCKED."
        ),
        vec!["review".into(), "verification".into(), "relay".into()],
    )
}
