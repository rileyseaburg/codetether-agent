//! Native session preparation for shared request construction.
use super::super::super::Runner;
use super::super::context::Attempt;
use crate::provider::Message;
use codetether_context_request::attempt::{Host, Target};

pub(super) struct Native<'a, 'session> {
    pub runner: &'a mut Runner<'session>,
    pub attempt: &'a mut Attempt,
}
impl Host for Native<'_, '_> {
    fn target(&self) -> Target<'_> {
        Target {
            system_prompt: &self.runner.model.system_prompt,
            provider: &self.runner.model.provider_name,
            model: &self.runner.model.model_id,
            tools: &self.runner.model.advertised,
            temperature: self.runner.model.temperature,
        }
    }
    fn proactive(&self) -> Option<&Message> {
        self.attempt.proactive.as_ref()
    }
    async fn prepare(&mut self, extras: &mut Vec<Message>) {
        super::super::super::super::step_model_restore::step_prepare::prepare_messages(
            extras,
            self.runner.session,
            &mut self.attempt.derived.messages,
        )
        .await;
    }
    fn history(&self) -> &[Message] {
        &self.attempt.derived.messages
    }
}
