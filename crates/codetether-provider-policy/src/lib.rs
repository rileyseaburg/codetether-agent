//! Provider request policies independent of native session ownership.
//!
//! [choose_default_provider] and [resolve_provider_for_session_request] retain
//! explicit selection and fallback ordering. Retry helpers handle missing tool
//! calls and upstream failures; [temperature] owns model parameter exclusions.
mod promise;
mod promise_markers;
mod resolve;
mod retry;
mod retry_markers;
mod selection;
pub mod temperature;
mod temperature_one;
mod tool_calls;
pub use promise::assistant_claims_imminent_tool_use;
pub use resolve::resolve_provider_for_session_request;
pub use retry::is_retryable_upstream_error;
pub use selection::choose_default_provider;
pub use temperature::temperature_is_deprecated;
pub use temperature_one::prefers_temperature_one;
pub use tool_calls::{
    provider_has_flaky_native_tool_calling, should_retry_missing_native_tool_call,
};
#[cfg(test)]
mod tests;

mod defaults;
pub use defaults::default_model_for_provider;
