//! Project quality policy loaded from the nearest `AGENTS.md`.

use std::path::Path;

pub(crate) struct ProjectQuality {
    pub(crate) policy: String,
    pub(crate) instructions: String,
    pub(crate) line_limit: Option<usize>,
}

pub(crate) fn load(working_dir: &Path) -> ProjectQuality {
    let content = crate::agent::builtin::load_agents_md(working_dir)
        .map(|(content, _)| content)
        .unwrap_or_default();
    let line_limit = super::quality_contract::line_limit(&content);
    let instructions = if content.is_empty() {
        String::new()
    } else {
        format!("\n\nPROJECT INSTRUCTIONS (from AGENTS.md):\n{content}")
    };
    ProjectQuality {
        policy: content,
        instructions,
        line_limit,
    }
}
