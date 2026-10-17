//! Prepare the initial model conversation and record its trace.
use super::*;
impl RlmExecutor {
    pub(super) fn prepare_analysis_messages(&mut self, query: &str) -> Vec<Message> {
        let system_prompt = crate::legacy_support::prompts::system_prompt(&self.repl);

        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt,
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("Analyze and answer: {}", query),
                }],
            },
        ];

        self.trace_analysis_initial(&messages, query);
        messages
    }
}
