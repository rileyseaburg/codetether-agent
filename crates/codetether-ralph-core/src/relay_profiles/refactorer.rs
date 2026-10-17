//! refactorer relay role prompt and capabilities.
use std::path::Path;
pub(super) fn profile(working_dir: &Path) -> (String, String, Vec<String>) {
    let wd = working_dir.display();
    (
        "auto-refactorer".to_string(),
        format!(
            "You are @auto-refactorer.\n\
                     Specialty: Code quality and lint compliance.\n\
                     Mission: Run clippy, fix warnings, improve code quality without changing behavior.\n\
                     Working directory: {wd}\n\n\
                     Run `bash` with `cargo clippy --all-features 2>&1` (cwd: {wd}).\n\
                     Fix warnings and improve naming, structure, error handling."
        ),
        vec!["quality".into(), "relay".into()],
    )
}
