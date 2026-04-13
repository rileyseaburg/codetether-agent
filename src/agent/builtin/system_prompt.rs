//! Prompt builders for built-in agents.
//!
//! This module merges raw prompt templates with AGENTS.md content and extra
//! guidance snippets.
//!
//! # Examples
//!
//! ```ignore
//! let prompt = build_system_prompt(std::path::Path::new("."));
//! ```

use crate::agent::build_guidance::BUILD_GITHUB_AUTH_GUIDANCE;
use std::path::Path;

use super::agents_md::load_all_agents_md;
use super::prompts::{BUILD_MODE_GUARDRAIL, BUILD_SYSTEM_PROMPT, PLAN_SYSTEM_PROMPT};

/// Builds the build-agent prompt for a working directory.
///
/// # Examples
///
/// ```ignore
/// let prompt = build_system_prompt(std::path::Path::new("."));
/// ```
pub fn build_system_prompt(cwd: &Path) -> String {
    let base_prompt = BUILD_SYSTEM_PROMPT.replace("{cwd}", &cwd.display().to_string());
    let agents_section = render_agents_section(cwd);
    format!("{base_prompt}{agents_section}{BUILD_GITHUB_AUTH_GUIDANCE}{BUILD_MODE_GUARDRAIL}")
}

/// Builds the plan-agent prompt for a working directory.
///
/// # Examples
///
/// ```ignore
/// let prompt = build_plan_system_prompt(std::path::Path::new("."));
/// ```
#[allow(dead_code)]
pub fn build_plan_system_prompt(cwd: &Path) -> String {
    let base_prompt = PLAN_SYSTEM_PROMPT.replace("{cwd}", &cwd.display().to_string());
    format!("{base_prompt}{}", render_agents_section(cwd))
}

fn render_agents_section(cwd: &Path) -> String {
    let agents_files = load_all_agents_md(cwd);
    if agents_files.is_empty() {
        return String::new();
    }
    let mut section = String::from(
        "\n\n## Project Instructions (AGENTS.md)\n\nThe following instructions were loaded from AGENTS.md files in the project.\nFollow these project-specific guidelines when working on this codebase.\n\n",
    );
    for (content, path) in agents_files.iter().rev() {
        section.push_str(&format!("### From {}\n\n{}\n\n", path.display(), content));
    }
    section
}
