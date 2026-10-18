//! Effective directory claims for tools that mutate through subprocesses.

use serde_json::Value;
use std::path::PathBuf;

pub(super) fn shell(input: &Value, field: &str) -> Option<Vec<PathBuf>> {
    let command = input.get(field)?.as_str()?;
    (!super::shell::read_only(command)).then(|| directory(input))
}

pub(super) fn directory(input: &Value) -> Vec<PathBuf> {
    let directory = input
        .get("workdir")
        .or_else(|| input.get("cwd"))
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_default();
    vec![directory]
}
