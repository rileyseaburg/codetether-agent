//! Session recall bound to the existing indexed search and RLM router.
mod host;
/// Local-first session recall using the application's existing backends.
///
/// # Examples
/// ```rust
/// # fn example(provider:std::sync::Arc<dyn codetether_agent::provider::Provider>) {
/// let _tool=codetether_agent::tool::session_recall::SessionRecallTool::new(provider,"model".into(),codetether_agent::rlm::RlmConfig::default());
/// # }
/// ```
pub type SessionRecallTool = codetether_session_recall::SessionRecallTool<host::ApplicationHost>;
