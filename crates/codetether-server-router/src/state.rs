//! State requirements for shared application HTTP routes.
/// State implementing each API crate's focused service-access contract.
/// # Implementors
/// Native applications implement the individual API contracts and this marker.
pub trait HttpState:
    codetether_audit_http::http::HttpState
    + codetether_bus_sse::http::HttpState
    + codetether_cognition_api::http::HttpState
    + codetether_k8s_api::http::HttpState
    + codetether_plugin_http::http::HttpState
    + codetether_task_api::http::HttpState
    + codetether_task_ingest::http::HttpState
    + codetether_tool_discovery::HttpState
    + codetether_voice_api::http::HttpState
{
}
