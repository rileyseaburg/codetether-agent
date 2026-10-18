//! Native cognition runtime and contracts specialized for application tools.
pub use codetether_native_cognition::*;
/// Application cognition runtime, with empty initial state only in unit tests.
pub type CognitionRuntime =
    codetether_native_cognition::CognitionRuntime<crate::tool::ToolRegistry, { cfg!(test) }>;
