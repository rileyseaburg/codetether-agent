//! Session storage bindings for context browsing, budgets, pinning and summaries.
//! [crate::impl_session_context_tools!] retains the selected bus and model resolver.
mod browse;
mod budget;
mod load;
mod pin;
mod summary;
/// Add context-tool storage contracts to a local session schema.
///
/// The bus type and resolver retain application ownership of summary processing.
/// Existing session load, save and indexed read methods are reused.
/// # Usage
/// ```text
/// codetether_session_schema::impl_session_context_tools!(Session, SessionBus, resolve_model);
/// ```
#[macro_export]
macro_rules! impl_session_context_tools {
    ($name:ident, $bus:ty, $resolver:path) => {
        $crate::__session_context_tool_load!($name, $bus, $resolver);
        $crate::__session_context_tool_browse!($name, $bus, $resolver);
        $crate::__session_context_tool_budget!($name, $bus, $resolver);
        $crate::__session_context_tool_pin!($name, $bus, $resolver);
        $crate::__session_context_tool_summary!($name, $bus, $resolver);
    };
}
