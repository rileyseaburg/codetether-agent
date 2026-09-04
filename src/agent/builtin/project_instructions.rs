//! Project-instruction composition for built-in prompts.

use std::path::Path;

use super::agents_md::load_all_agents_md;

const HEADER: &str = "\n\n## Project Instructions (AGENTS.md)\n\n\
The following instructions were loaded from AGENTS.md or AGENTS.override.md files in the project.\n\
Follow these project-specific guidelines when working on this codebase.\n\n";

/// Append all workspace-scoped instruction files to a base prompt.
///
/// Instructions are loaded root-to-leaf so deeper files appear after the
/// broader rules they override.
pub(crate) fn append_project_instructions(mut prompt: String, cwd: &Path) -> String {
    let agents_files = load_all_agents_md(cwd);
    if agents_files.is_empty() {
        return prompt;
    }
    prompt.push_str(HEADER);
    for (content, path) in agents_files {
        prompt.push_str("### From ");
        prompt.push_str(&path.display().to_string());
        prompt.push_str("\n\n");
        prompt.push_str(&content);
        prompt.push_str("\n\n");
    }
    prompt
}
