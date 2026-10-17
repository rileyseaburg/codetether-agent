//! is hidden path for codebase indexing.

use super::*;

pub(in crate::indexer) fn is_hidden_path(path: &Path) -> bool {
    codetether_util::workspace_scan::path_is_hidden(path)
}
