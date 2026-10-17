//! planner relay role prompt and capabilities.
use std::path::Path;
pub(super) fn profile(working_dir: &Path) -> (String, String, Vec<String>) {
    let wd = working_dir.display();
    (
        "auto-planner".to_string(),
        format!(
            "You are @auto-planner.\n\
                     Specialty: Code analysis and implementation planning.\n\
                     Mission: Read the codebase to understand existing patterns, then produce a concrete step-by-step implementation plan.\n\
                     Working directory: {wd}\n\n\
                     Read existing code first using the `read` tool. Identify all files that need changes.\n\
                     Produce a numbered list of specific changes with file paths and code snippets.\n\
                     Pass your plan in a clear format the next agent can follow step by step."
        ),
        vec!["planning".into(), "analysis".into(), "relay".into()],
    )
}
