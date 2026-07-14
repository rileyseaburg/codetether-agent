//! Shared operand parsing for mux slash-command actions.

use std::path::{Path, PathBuf};

use super::action::Action;

pub(super) fn workspace_action<F>(rest: &str, cwd: &Path, make: F) -> Result<Action, String>
where
    F: FnOnce(String, PathBuf) -> Action,
{
    let (name, path) = split(rest);
    if name.is_empty() {
        return Err("mux session name is required".into());
    }
    let workspace = if path.is_empty() {
        cwd.into()
    } else {
        cwd.join(path)
    };
    Ok(make(name.into(), workspace))
}

pub(super) fn id_action<F>(rest: &str, make: F) -> Result<Action, String>
where
    F: FnOnce(String, u64) -> Action,
{
    let (name, raw_id) = split(rest);
    let id = raw_id.parse().map_err(|_| "window id must be an integer")?;
    Ok(make(name.into(), id))
}

pub(super) fn split(value: &str) -> (&str, &str) {
    value
        .trim()
        .split_once(char::is_whitespace)
        .map_or((value.trim(), ""), |(a, b)| (a, b.trim()))
}
