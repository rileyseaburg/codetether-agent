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

use crate::agent::build_guidance::{BUILD_GITHUB_AUTH_GUIDANCE, WORKTREE_GUIDANCE};
use std::path::Path;

use super::project_instructions::append_project_instructions;
use super::prompts::{BUILD_MODE_GUARDRAIL, BUILD_SYSTEM_PROMPT, PLAN_SYSTEM_PROMPT};
use super::vscode_lm_tools::render_section as render_vscode_lm_tools_section;

/// Builds the build-agent prompt for a working directory.
///
/// # Examples
///
/// ```ignore
/// let prompt = build_system_prompt(std::path::Path::new("."));
/// ```
pub fn build_system_prompt(cwd: &Path) -> String {
    let base_prompt = BUILD_SYSTEM_PROMPT.replace("{cwd}", &cwd.display().to_string());
    let prompt = append_project_instructions(base_prompt, cwd);
    let lm_tools_section = render_vscode_lm_tools_section(cwd);
    format!(
        "{prompt}{lm_tools_section}{BUILD_GITHUB_AUTH_GUIDANCE}{WORKTREE_GUIDANCE}{BUILD_MODE_GUARDRAIL}"
    )
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
    let prompt = append_project_instructions(base_prompt, cwd);
    format!("{prompt}{}", render_vscode_lm_tools_section(cwd))
}
