//! In-memory proposed content for `edit` and `multiedit` approvals.
//!
//! [single] reconstructs one edit; [many] chains edits by path without writing.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

use super::matcher::MatchPlan;
use codetether_tool_core::proposed::{resolve, text};

/// Proposed content for one `edit` call.
///
/// # Arguments
/// * `root` - Directory used to resolve relative paths.
/// * `args` - The path, old_string, new_string and optional replace_all fields.
/// # Returns
/// The target path and proposed text, without writing it.
/// # Errors
/// Returns errors for invalid arguments, unreadable files or failed matching.
/// # Examples
/// ```rust,no_run
/// # fn example() -> anyhow::Result<()> {
/// let files = codetether_edit::proposed::single(std::path::Path::new("."),
///     &serde_json::json!({"path":"file.txt","old_string":"old","new_string":"new"}))?;
/// assert_eq!(files.len(), 1);
/// # Ok(())
/// # }
/// ```
pub fn single(root: &Path, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    let path = resolve(root, text(args, "path")?);
    let content = read(&path)?;
    let replace_all = args["replace_all"].as_bool().unwrap_or(false);
    let updated = replaced(&content, args, "old_string", "new_string", replace_all)?;
    Ok(vec![(path, updated)])
}

/// Proposed content for every file touched by one `multiedit` call.
///
/// # Arguments
/// * `root` - Directory used to resolve relative paths.
/// * `args` - An edits array with file, old_string and new_string fields.
/// # Returns
/// One final proposed text per target path.
/// # Errors
/// Returns errors for invalid arguments, unreadable files or failed matching.
/// # Examples
/// ```rust
/// let files = codetether_edit::proposed::many(std::path::Path::new("."),
///     &serde_json::json!({"edits":[]})).unwrap();
/// assert!(files.is_empty());
/// ```
pub fn many(root: &Path, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    let edits = args["edits"].as_array().context("missing `edits` array")?;
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for edit in edits {
        let path = resolve(root, text(edit, "file")?);
        let slot = match files.iter().position(|(item, _)| *item == path) {
            Some(index) => index,
            None => {
                files.push((path.clone(), read(&path)?));
                files.len() - 1
            }
        };
        files[slot].1 = replaced(&files[slot].1, edit, "old_string", "new_string", false)?;
    }
    Ok(files)
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))
}

fn replaced(content: &str, args: &Value, old: &str, new: &str, all: bool) -> Result<String> {
    let old_string = text(args, old)?;
    let new_string = text(args, new)?;
    match MatchPlan::find(content, old_string, all) {
        Ok(plan) => Ok(plan.apply(content, new_string)),
        Err(result) => bail!("{}", result.output),
    }
}
