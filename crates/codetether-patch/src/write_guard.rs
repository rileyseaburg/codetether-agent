//! Validate every patch destination before approval or the first write.
use anyhow::Result;
use codetether_tool_core::{ToolResult, temp_write_guard};
use std::path::Path;

pub(super) fn check(root: &Path, files: &[String]) -> Result<Option<ToolResult>> {
    for file in files {
        let path = super::path_guard::resolve(root, file)?;
        if let Some(blocked) =
            temp_write_guard::denied_result("apply_patch", &path.to_string_lossy())
        {
            return Ok(Some(blocked));
        }
        // Resolve the nearest existing ancestor, including symlinked parents
        // of new files, so an alias cannot hide a temporary destination.
        for ancestor in path.ancestors() {
            if let Ok(canonical) = ancestor.canonicalize() {
                let resolved = canonical.join(path.strip_prefix(ancestor)?);
                if let Some(blocked) =
                    temp_write_guard::denied_result("apply_patch", &resolved.to_string_lossy())
                {
                    return Ok(Some(blocked));
                }
                break;
            }
        }
    }
    Ok(None)
}
