//! Filesystem scanning and shared-prefix calculation for completion.

use std::path::{Path, PathBuf};

pub(super) fn location(argument: &str, workspace: &Path) -> (PathBuf, String) {
    if argument.is_empty() {
        return (workspace.into(), String::new());
    }
    let expanded = super::path::expand(argument, workspace);
    if argument.ends_with(std::path::MAIN_SEPARATOR) {
        return (expanded, String::new());
    }
    let parent = expanded.parent().unwrap_or_else(|| Path::new("")).into();
    let prefix = expanded
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .into();
    (parent, prefix)
}

pub(super) fn directories(search: &Path, prefix: &str) -> std::io::Result<Vec<String>> {
    let mut names = std::fs::read_dir(search)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().into_string().ok()?;
            (entry.file_type().ok()?.is_dir()
                && name.starts_with(prefix)
                && !name.chars().any(char::is_control))
            .then_some(name)
        })
        .collect::<Vec<_>>();
    names.sort();
    Ok(names)
}

pub(super) fn common(values: &[String]) -> String {
    values[1..].iter().fold(values[0].clone(), |prefix, value| {
        prefix
            .chars()
            .zip(value.chars())
            .take_while(|(a, b)| a == b)
            .map(|(value, _)| value)
            .collect()
    })
}
