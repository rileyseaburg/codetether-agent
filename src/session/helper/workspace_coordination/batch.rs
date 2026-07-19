//! Atomic aggregation of mutation paths hidden inside batch tool calls.

use serde_json::Value;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub(super) fn paths(input: &Value) -> Option<Vec<PathBuf>> {
    let mut paths = BTreeSet::new();
    for call in input.get("calls")?.as_array()? {
        let tool = call.get("tool").or_else(|| call.get("name"))?.as_str()?;
        let args = call.get("args").or_else(|| call.get("arguments"))?;
        paths.extend(super::paths::mutation_paths(tool, args).unwrap_or_default());
    }
    (!paths.is_empty()).then(|| paths.into_iter().collect())
}
