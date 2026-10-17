//! Compile literal or regular-expression queries with case options.
use anyhow::Result;
use regex::RegexBuilder;
use serde_json::Value;

pub(super) fn matcher(args: &Value, query: &str) -> Result<regex::Regex> {
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
