//! Shared cognition runtime specialized with the application services.
/// Application cognition runtime with native thinker and tool bindings.
/// # Examples
/// ```
/// use codetether_agent::cognition::{CognitionRuntime, CognitionRuntimeOptions};
/// let runtime = CognitionRuntime::new_with_options(CognitionRuntimeOptions::default());
/// assert!(!runtime.is_enabled());
/// ```
pub type CognitionRuntime =
    codetether_cognition_runtime::CognitionRuntime<super::runtime_host::Native>;
