//! AGENTS.md discovery helpers for built-in prompts.
//!
//! This module searches for project instruction files while respecting the git
//! repository boundary.
//!
//! # Examples
//!
//! ```rust,no_run
//! use codetether_agent::agent::builtin::load_all_agents_md;
//!
//! let all = load_all_agents_md(std::path::Path::new("."));
//! assert!(all.iter().all(|(_, path)| path.is_absolute() || path.exists()));
//! ```

use std::path::{Path, PathBuf};

mod dirs;
mod global;
mod merge;
mod read;

const DEFAULT_AGENTS_MD_BYTE_CAP: usize = 32 * 1024;

/// Loads the closest instruction file at or above `start_dir`, stopping at the git root.
///
/// `AGENTS.override.md` takes precedence over `AGENTS.md` within the same
/// directory.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::agent::builtin::load_agents_md;
///
/// let loaded = load_agents_md(std::path::Path::new("."));
/// assert!(loaded.is_none() || loaded.as_ref().is_some_and(|(content, _)| !content.is_empty()));
/// ```
pub fn load_agents_md(start_dir: &Path) -> Option<(String, PathBuf)> {
    dirs::root_to_leaf(start_dir)
        .into_iter()
        .rev()
        .find_map(|dir| read::preferred_instruction_file(&dir, DEFAULT_AGENTS_MD_BYTE_CAP))
        .or_else(|| global::instruction_file(DEFAULT_AGENTS_MD_BYTE_CAP))
}

/// Loads all instruction files between `start_dir` and the git root.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::agent::builtin::load_all_agents_md;
///
/// let files = load_all_agents_md(std::path::Path::new("."));
/// assert!(files.iter().all(|(content, _)| !content.is_empty()));
/// ```
pub fn load_all_agents_md(start_dir: &Path) -> Vec<(String, PathBuf)> {
    load_all_agents_md_with_byte_cap(start_dir, DEFAULT_AGENTS_MD_BYTE_CAP)
}

/// Loads instruction files with an explicit cumulative byte cap.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::agent::builtin::load_all_agents_md_with_byte_cap;
///
/// let files = load_all_agents_md_with_byte_cap(std::path::Path::new("."), 8192);
/// assert!(files.iter().all(|(content, _)| content.len() <= 8192));
/// ```
pub fn load_all_agents_md_with_byte_cap(
    start_dir: &Path,
    max_bytes: usize,
) -> Vec<(String, PathBuf)> {
    merge::load_all(start_dir, max_bytes, global::instruction_file(max_bytes))
}

#[cfg(test)]
pub(super) fn load_all_agents_md_with_codex_home(
    start_dir: &Path,
    max_bytes: usize,
    codex_home: &Path,
) -> Vec<(String, PathBuf)> {
    merge::load_all(
        start_dir,
        max_bytes,
        global::instruction_file_in(codex_home, max_bytes),
    )
}
