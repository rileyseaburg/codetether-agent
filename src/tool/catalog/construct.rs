//! Resolve one scalar capability to its native constructor.
use super::*;
pub(super) fn create(kind: ToolKind) -> Arc<dyn Tool> {
    files::create(kind)
        .or_else(|| context::create(kind))
        .or_else(|| execution::create(kind))
        .or_else(|| media::create(kind))
        .or_else(|| search_tools::create(kind))
        .or_else(|| integrations::create(kind))
        .expect("native catalog constructor")
}
