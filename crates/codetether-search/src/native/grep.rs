//! Native content search combining path collection, matching and rendering.
use anyhow::Result;
use serde_json::Value;
/// Search text files using a literal query or regular expression.
///
/// # Arguments
/// `args` requires `query`; optional `path`, `is_regex` and
/// `case_sensitive` control traversal and matching.
/// # Returns
/// Matching path, one-based line number and content records joined by newlines.
/// # Errors
/// Missing queries or invalid regular expressions. Unreadable files are skipped.
/// # Examples
/// ```rust,no_run
/// let output = codetether_search::native::grep(&serde_json::json!({
///     "query": "example", "path": ".", "case_sensitive": false
/// })).unwrap();
/// ```

pub fn grep(args: &Value) -> Result<String> {
    let query = args
        .get("query")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
    let path = args.get("path").and_then(Value::as_str).unwrap_or(".");
    let matcher = super::matcher::matcher(args, query)?;
    Ok(super::paths::collect(path)
        .into_iter()
        .filter_map(|path| super::lines::file_matches(&path, &matcher).ok())
        .flatten()
        .collect::<Vec<_>>()
        .join("\n"))
}
