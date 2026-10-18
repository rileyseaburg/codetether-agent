//! File tool selection and construction for the application catalog.
//!
//! Call [create] with a catalog [ToolKind] and lazy factories for the two
//! provider-bound editors. File access and editing modules own the concrete
//! constructors; unrelated kinds return none without invoking either factory.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
mod access;
mod editing;

/// Construct the file tool selected by a catalog kind.
///
/// # Arguments
/// * `kind` — Tool requested by the ordered application catalog.
/// * `edit` — Factory called only for the single-file edit tool.
/// * `multi_edit` — Factory called only for the multi-file edit tool.
///
/// # Returns
/// The selected tool, or `None` for kinds owned by another catalog.
///
/// # Examples
/// ```
/// use codetether_file_tool_catalog::create;
/// use codetether_tool_registry::catalog::ToolKind;
/// let tool = create(ToolKind::Read, || unreachable!(), || unreachable!());
/// assert!(tool.is_some());
/// ```
pub fn create(
    kind: ToolKind,
    edit: impl FnOnce() -> Arc<dyn Tool>,
    multi_edit: impl FnOnce() -> Arc<dyn Tool>,
) -> Option<Arc<dyn Tool>> {
    access::create(kind).or_else(|| editing::create(kind, edit, multi_edit))
}
