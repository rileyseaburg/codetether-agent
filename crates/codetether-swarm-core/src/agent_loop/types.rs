use super::*;

/// Tool invocation extracted from a provider response.
///
/// # Examples
/// ```
/// let call = codetether_swarm_core::agent_loop::ToolCall { id: "call".into(), name: "read".into(), arguments: "{}".into() };
/// assert_eq!(call.name, "read");
/// ```
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

/// Parsed response components used by the turn driver.
///
/// # Examples
/// ```
/// # fn example(message: codetether_provider_core::Message) {
/// let turn = codetether_swarm_core::agent_loop::Turn { message, finish: codetether_provider_core::FinishReason::Stop, text: String::new(), thinking: String::new(), tools: vec![] };
/// assert!(turn.tools.is_empty());
/// # }
/// ```
pub struct Turn {
    pub message: Message,
    pub finish: FinishReason,
    pub text: String,
    pub thinking: String,
    pub tools: Vec<ToolCall>,
}

/// Provider request result or deadline expiration.
///
/// # Examples
/// ```
/// assert!(matches!(codetether_swarm_core::agent_loop::RequestOutcome::TimedOut, codetether_swarm_core::agent_loop::RequestOutcome::TimedOut));
/// ```
pub enum RequestOutcome {
    Response(CompletionResponse),
    TimedOut,
}
