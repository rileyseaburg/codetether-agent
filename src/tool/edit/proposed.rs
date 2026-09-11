//! In-memory proposed content for `edit` and `multiedit` approvals.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::Value;

use super::matcher::MatchPlan;
use crate::tool::proposed_content::{resolve, text};

/// Proposed content for one `edit` call.
pub(crate) fn single(root: &Path, args: &Value) -> Result<Vec<(PathBuf, String)>> {
    let path = resolve(root, text(args, "path")?);
    let content = read(&path)?;
    let replace_all = args["replace_all"].as_bool().unwrap_or(false);
    let updated = replaced(&content, args, "old_string", "new_string", replace_all)?;
    Ok(vec![(path, updated)])
}

/// Proposed content for every file touched by one `multiedit` call.
pub(crate) fn many(root: &Path, args: &Value) -> Result<Vec<(PathBuf, String)>> {
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
