//! Select the application runtime for shared Ralph tool execution.
/// Tool host retaining the native Ralph runtime and its Git policy.
pub type ApplicationHost =
    codetether_ralph_runtime::tool::ApplicationHost<crate::ralph::runtime_host::Native>;
