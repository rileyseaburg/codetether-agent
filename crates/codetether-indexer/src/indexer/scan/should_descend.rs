//! should descend for codebase indexing.

use super::*;

pub(in crate::indexer) fn should_descend(
    entry: &DirEntry,
    root: &Path,
    include_hidden: bool,
) -> bool {
    let path = entry.path();
    let rel_path = path.strip_prefix(root).unwrap_or(path);

    if !include_hidden && is_hidden_path(rel_path) {
        return false;
    }

    !codetether_util::workspace_scan::path_has_pruned_component(rel_path)
}
