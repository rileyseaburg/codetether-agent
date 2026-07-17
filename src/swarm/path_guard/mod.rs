//! Worktree path policy for sub-agent tool calls.

use anyhow::Result;
use serde_json::Value;
use std::path::Path;

mod authoritative;
mod batch;
mod inspect;
mod normalize;
mod spec;

pub fn normalize_tool_args(tool: &str, args: &mut Value, root: &Path) -> Result<()> {
    if tool == "batch" {
        return batch::normalize(args, root);
    }
    authoritative::apply(tool, args, root);
    let specs = spec::field_specs(tool, args)?;
    for field in &specs {
        normalize::field(args, field, root)?;
    }
    inspect::reject_unknown_path_fields(args, &specs, String::new())?;
    Ok(())
}

#[cfg(test)]
#[path = "../path_guard_authoritative_tests.rs"]
mod authoritative_tests;
#[cfg(test)]
#[path = "../path_guard_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "../path_guard_batch_tests.rs"]
mod batch_tests;
