//! Glob-based native filename search.

use anyhow::Result;
use glob::Pattern;
use serde_json::Value;
use walkdir::WalkDir;

/// Find filenames matching a glob below the requested root.
///
/// # Arguments
/// `args` requires `pattern`; `path` defaults to the current directory.
/// # Returns
/// Sorted paths joined with newlines.
/// # Errors
/// Missing or invalid glob patterns; unreadable traversal entries are skipped.
/// # Examples
/// ```rust,no_run
/// let output = codetether_search::native::search_files(&serde_json::json!({
///     "pattern": "*.rs", "path": "."
/// })).unwrap();
/// ```
pub fn search_files(args: &Value) -> Result<String> {
    let pattern = args
        .get("pattern")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing pattern"))?;
    let path = args.get("path").and_then(Value::as_str).unwrap_or(".");
    let glob = Pattern::new(pattern)?;
    let mut matches = Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry
            .file_name()
            .to_str()
            .is_some_and(|name| glob.matches(name))
        {
            matches.push(entry.path().display().to_string());
        }
    }
    matches.sort();
    Ok(matches.join("\n"))
}
