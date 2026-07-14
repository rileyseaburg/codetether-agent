//! Completion request construction for agents.
//!
//! This module isolates provider request assembly and default-model selection
//! from the outer execution loop.
//!
//! # Examples
//!
//! ```ignore
//! let request = agent.build_completion_request(&session);
//! ```

use crate::agent::Agent;
use crate::provider::CompletionResponse;
use crate::session::Session;
use anyhow::Result;

impl Agent {
    pub(super) async fn complete_with_context(
        &self,
        session: &Session,
    ) -> Result<CompletionResponse> {
        let model = super::model::default_for(self);
        let tools: Vec<_> = self
            .tools
            .definitions()
            .into_iter()
            .filter(|tool| {
                crate::session::helper::runtime::prior_context_tool_available_for_session(
                    session, &tool.name,
                )
            })
            .collect();
        let system_prompt = super::messages::compose_system_prompt(&self.system_prompt, session);
        crate::session::context::complete_with_context(
            std::sync::Arc::clone(&self.provider),
            session,
            &model,
            &system_prompt,
            &tools,
            crate::session::context::RequestOptions {
                temperature: self.info.temperature,
                top_p: self.info.top_p,
                max_tokens: None,
                force_keep_last: None,
                policy_override: None,
            },
        )
        .await
    }
}
