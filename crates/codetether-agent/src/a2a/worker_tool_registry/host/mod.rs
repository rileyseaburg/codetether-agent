//! Application workflow choices for native worker tool assembly.
mod workflow;
/// Worker host using the native registry/workspace services and application workflows.
pub(crate) type Native = codetether_native_worker::tool_registry::Native<
    crate::tool::registry_lifecycle::Native,
    workflow::Native,
>;
