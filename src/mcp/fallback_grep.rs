//! Native content search for fallback MCP tools.

use anyhow::Result;
use regex::RegexBuilder;
use serde_json::Value;
use std::path::Path;

pub(super) fn run(args: &Value) -> Result<String> {
    let query = args
        .get("query")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing query"))?;
    let path = args.get("path").and_then(Value::as_str).unwrap_or(".");
    let matcher = matcher(args, query)?;
    Ok(super::fallback_grep_paths::collect(path)
        .into_iter()
        .filter_map(|path| file_matches(&path, &matcher).ok())
        .flatten()
        .collect::<Vec<_>>()
        .join("\n"))
}

fn matcher(args: &Value, query: &str) -> Result<regex::Regex> {
    let is_regex = args
        .get("is_regex")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let case = args
        .get("case_sensitive")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let pattern = if is_regex {
        query.into()
    } else {
        regex::escape(query)
    };
    Ok(RegexBuilder::new(&pattern)
        .case_insensitive(!case)
        .build()?)
}

fn file_matches(path: &Path, matcher: &regex::Regex) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content
        .lines()
        .enumerate()
        .filter(|(_, line)| matcher.is_match(line))
        .map(|(idx, line)| format!("{}:{}:{}", path.display(), idx + 1, line))
        .collect())
}
