//! Target-path extraction from unified diffs.

use serde_json::Value;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub(super) fn paths(input: &Value) -> Option<Vec<PathBuf>> {
    let patch = input.get("patch")?.as_str()?;
    let paths = patch
        .lines()
        .filter_map(line_path)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    (!paths.is_empty()).then_some(paths)
}

fn line_path(line: &str) -> Option<PathBuf> {
    let raw = line
        .strip_prefix("+++ ")
        .or_else(|| line.strip_prefix("--- "))
        .or_else(|| line.strip_prefix("*** Update File: "))
        .or_else(|| line.strip_prefix("*** Add File: "))
        .or_else(|| line.strip_prefix("*** Delete File: "))
        .or_else(|| line.strip_prefix("*** Move to: "))?
        .split('\t')
        .next()?
        .trim();
    if raw == "/dev/null" {
        return None;
    }
    let relative = raw
        .strip_prefix("a/")
        .or_else(|| raw.strip_prefix("b/"))
        .unwrap_or(raw);
    Some(relative.into())
}
