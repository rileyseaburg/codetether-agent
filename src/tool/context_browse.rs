//! Bind bounded context browsing to the application's durable session store.
pub use codetether_context_browse::{ContextBrowseAction, format_turn_path, parse_browse_action};
/// Context browser using the existing session implementation.
pub type ContextBrowseTool = codetether_context_browse::ContextBrowseTool<crate::session::Session>;
/// Preserve the existing unit-like tool construction at registration sites.
#[allow(non_upper_case_globals)]
pub const ContextBrowseTool: ContextBrowseTool =
    codetether_context_browse::ContextBrowseTool::new();
