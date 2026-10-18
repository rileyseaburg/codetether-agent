//! A prepared completion request borrowed for one context-aware call.
use crate::*;
/// Values forwarded to the native session-context completion service.
/// # Examples
/// ```rust,no_run
/// # fn example<S>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, session: &S) {
/// let request = codetether_agent_completion::Request {
///     provider, session, model: "model", system_prompt: "system", tools: &[],
///     temperature: None, top_p: None,
/// };
/// assert!(request.tools.is_empty());
/// # }
/// ```
pub struct Request<'a, S> {
    /// Provider retained for the completion call.
    pub provider: Arc<dyn Provider>,
    /// Session whose history supplies the context.
    pub session: &'a S,
    /// Selected model or provider fallback.
    pub model: &'a str,
    /// Goal/peer-enriched system prompt.
    pub system_prompt: &'a str,
    /// Tools remaining after session policy filtering.
    pub tools: &'a [ToolDefinition],
    /// Existing agent temperature setting.
    pub temperature: Option<f32>,
    /// Existing agent top-p setting.
    pub top_p: Option<f32>,
}
