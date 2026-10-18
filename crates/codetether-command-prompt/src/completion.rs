//! Directory-only completion for `cd` and `new` mux commands.

mod path;
mod scan;

use std::path::Path;

use anyhow::Result;

pub(super) struct Completion {
    pub line: String,
    pub matches: Vec<String>,
}

pub(super) fn directory(line: &str, workspace: &Path) -> Result<Completion> {
    let Some((command, argument)) = line.split_once(' ') else {
        return unchanged(line);
    };
    if !matches!(command, "cd" | "new") {
        return unchanged(line);
    }
    let (search, prefix) = scan::location(argument, workspace);
    let Ok(names) = scan::directories(&search, &prefix) else {
        return unchanged(line);
    };
    if names.is_empty() {
        return unchanged(line);
    }
    let name = scan::common(&names);
    let completed = search.join(&name);
    let suffix = if names.len() == 1 {
        std::path::MAIN_SEPARATOR_STR
    } else {
        ""
    };
    let line = format!("{command} {}{suffix}", completed.display());
    let matches = names
        .iter()
        .map(|name| search.join(name).display().to_string())
        .collect();
    Ok(Completion { line, matches })
}

fn unchanged(line: &str) -> Result<Completion> {
    Ok(Completion {
        line: line.into(),
        matches: Vec::new(),
    })
}

#[cfg(test)]
mod tests;
