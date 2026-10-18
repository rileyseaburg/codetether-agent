//! Alternate completion and tool replay until final output or step exhaustion.
use crate::*;
use anyhow::{Result, bail};
use codetether_agent_core::execution::messages::{collect_tool_calls, response_text};
/// Executes a prompt against the agent until the run completes or max-steps is hit.
///
/// The agent appends the prompt to the session, loops through tool calls,
/// and returns the final assistant text plus recorded usage data.
///
/// # Arguments
/// Receives the agent host, mutable native session and human prompt.
/// # Returns
/// Final assistant text with the session's recorded tool usage and token usage.
/// # Errors
/// Returns context/provider errors or step-limit exhaustion.
/// # Examples
/// ```rust,no_run
/// # async fn example<H: codetether_agent_completion::Host>(host: &H, session: &mut H::Session) -> anyhow::Result<()> {
/// let response = codetether_agent_completion::execute(host, session, "review this file").await?;
/// let _ = response.text;
/// # Ok(()) }
/// ```
pub async fn execute<H: Host>(
    host: &H,
    session: &mut H::Session,
    prompt: &str,
) -> Result<AgentResponse> {
    H::add_human_message(
        session,
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        },
    );
    let max_steps = host.info().max_steps.unwrap_or(100);
    for _step in 1..=max_steps {
        let response = crate::request::complete_with_context(host, session).await?;
        H::add_message(session, response.message.clone());
        let tool_calls = collect_tool_calls(&response.message);
        if tool_calls.is_empty() {
            return Ok(AgentResponse {
                text: response_text(&response.message),
                tool_uses: H::tool_uses(session).to_vec(),
                usage: H::usage(session).clone(),
            });
        }
        codetether_agent_replay::execute(host, session, tool_calls).await;
    }
    bail!("Exceeded maximum steps ({max_steps})");
}
