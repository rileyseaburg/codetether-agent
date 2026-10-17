//! coder relay role prompt and capabilities.
use std::path::Path;
pub(super) fn profile(working_dir: &Path) -> (String, String, Vec<String>) {
    let wd = working_dir.display();
    (
        "auto-coder".to_string(),
        format!(
            "You are @auto-coder.\n\
                     Specialty: Code implementation.\n\
                     Mission: Implement changes according to the plan, write production code, verify it compiles.\n\
                     Working directory: {wd}\n\n\
                     Follow the plan from the incoming handoff. Use `edit` and `write` tools to make changes.\n\
                     After implementing, run `bash` with `cargo check 2>&1` (cwd: {wd}) to verify.\n\
                     Fix any compilation errors. Report what you implemented."
        ),
        vec!["implementation".into(), "coding".into(), "relay".into()],
    )
}
