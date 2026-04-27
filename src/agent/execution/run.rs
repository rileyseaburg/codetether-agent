//! Prompt loop for agent execution.
//!
//! This module implements the outer completion loop that alternates between
//! provider responses and tool execution until the model stops calling tools.
//!
//! # Examples
//!
//! ```ignore
//! let response = agent.execute(&mut session, "inspect").await?;
//! ```

use super::context::derive_agent_context;
use super::messages::{collect_tool_calls, response_text};
use crate::agent::{Agent, AgentResponse};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::{Result, bail};

impl Agent {
    /// Executes a prompt against the agent until the run completes or max-steps is hit.
    ///
    /// The agent appends the prompt to the session, loops through tool calls,
    /// and returns the final assistant text plus recorded usage data.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let response = agent.execute(&mut session, "review this file").await?;
    /// ```
    pub async fn execute(&self, session: &mut Session, prompt: &str) -> Result<AgentResponse> {
        session.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: prompt.to_string(),
            }],
        });
        let max_steps = self.info.max_steps.unwrap_or(100);
        for _step in 1..=max_steps {
            let (system_prompt, derived) = derive_agent_context(self, session).await?;
            let request = self.build_completion_request(system_prompt, derived);
            let response = self.provider.complete(request).await?;
            session.add_message(response.message.clone());
            let tool_calls = collect_tool_calls(&response.message);
            if tool_calls.is_empty() {
                return Ok(AgentResponse {
                    text: response_text(&response.message),
                    tool_uses: session.tool_uses.clone(),
                    usage: session.usage.clone(),
                });
            }
            self.execute_tool_calls(session, tool_calls).await;
        }
        bail!("Exceeded maximum steps ({max_steps})");
    }
}
