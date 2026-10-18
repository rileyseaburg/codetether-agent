//! Shared provider selection, request and missing-tool-call policy.
pub use codetether_provider_policy::{
    assistant_claims_imminent_tool_use, choose_default_provider, prefers_temperature_one,
    provider_has_flaky_native_tool_calling, resolve_provider_for_session_request,
    should_retry_missing_native_tool_call, temperature, temperature_is_deprecated,
};
