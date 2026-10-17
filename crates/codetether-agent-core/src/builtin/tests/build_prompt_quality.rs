//! Behavioral invariants for the build agent's operating prompt.

use super::super::{BUILD_SYSTEM_PROMPT, build_system_prompt};
use tempfile::tempdir;

#[test]
fn prompt_defines_codex_grade_repository_discipline() {
    for required in [
        "deeper file overrides a broader one",
        "Preserve user changes and unrelated work",
        "Do not run destructive Git or filesystem commands",
        "Never claim a command passed unless its output was observed",
        "Do not delete or hide artifacts",
    ] {
        assert!(
            BUILD_SYSTEM_PROMPT.contains(required),
            "missing: {required}"
        );
    }
}

#[test]
fn rendered_prompt_includes_working_directory_and_persistence() {
    let tmp = tempdir().expect("tempdir");
    let prompt = build_system_prompt(tmp.path());

    assert!(prompt.contains(&tmp.path().display().to_string()));
    assert!(prompt.contains("until the request is genuinely resolved"));
    assert!(!prompt.contains("{cwd}"));
}
