//! Registration of the code-search tool cluster (grep, ripgrep, codesearch,
//! token_count) shared by every `ToolRegistry` builder.

use super::{ToolRegistry, codesearch, ripgrep, search, token_count};
use std::sync::Arc;

/// Register the search tool cluster onto `registry`.
pub(super) fn register(registry: &mut ToolRegistry) {
    registry.register(Arc::new(search::GrepTool::new()));
    registry.register(Arc::new(ripgrep::RipgrepTool::new()));
    registry.register(Arc::new(codesearch::CodeSearchTool::new()));
    registry.register(Arc::new(token_count::TokenCountTool));
}
