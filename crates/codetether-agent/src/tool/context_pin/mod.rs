//! Bind context pinning to the application's tracked session pages.
/// Context pinning using the application's durable session store.
///
/// # Examples
/// ```rust
/// use codetether_agent::tool::context_pin::ContextPinTool;
/// let _tool: ContextPinTool = ContextPinTool;
/// ```
pub type ContextPinTool = codetether_context_pin::ContextPinTool<crate::session::Session>;
/// Preserve unit-like tool construction at existing registration sites.
///
/// # Examples
/// ```rust
/// let _tool=codetether_agent::tool::context_pin::ContextPinTool;
/// ```
#[allow(non_upper_case_globals)]
pub const ContextPinTool: ContextPinTool = codetether_context_pin::ContextPinTool::new();
