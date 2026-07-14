//! Resolution of model-authored dependency names to internal task IDs.

use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub(super) fn resolve(
    dependencies: &[String],
    name_to_id: &HashMap<String, String>,
) -> Result<Vec<String>> {
    dependencies
        .iter()
        .map(|name| {
            name_to_id
                .get(name)
                .cloned()
                .ok_or_else(|| anyhow!("Unknown swarm dependency name: {name}"))
        })
        .collect()
}

#[cfg(test)]
#[path = "dependencies_tests.rs"]
mod tests;
