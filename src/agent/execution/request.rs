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
use crate::provider::CompletionRequest;
use crate::session::Session;

impl Agent {
    pub(super) fn build_completion_request(&self, session: &Session) -> CompletionRequest {
        CompletionRequest {
            messages: self.build_messages(session),
            tools: self.tools.definitions(),
            model: self.default_model(),
            temperature: self.info.temperature,
            top_p: self.info.top_p,
            max_tokens: None,
            stop: vec![],
        }
    }

    fn default_model(&self) -> String {
        self.info
            .model
            .clone()
            .unwrap_or_else(|| match self.provider.name() {
                "zhipuai" | "zai" => "glm-5".to_string(),
                "openrouter" => "z-ai/glm-5".to_string(),
                _ => "glm-5".to_string(),
            })
    }
}
