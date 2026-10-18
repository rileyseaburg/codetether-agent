//! Native human-message, publication and title services.
use super::super::Runner;
use crate::provider::Message;
impl codetether_prompt_loop::input::Host for Runner<'_> {
    fn append_human(&mut self, message: Message) {
        self.inner.session.add_human_message(message);
    }
    fn turn_id(&self) -> &str {
        &self.inner.progress.turn_id
    }
    fn publish(&self, message: &str, turn_id: String) {
        super::super::super::publish_user_prompt::publish(
            self.inner.session,
            message,
            Some(turn_id),
        );
    }
    fn needs_title(&self) -> bool {
        self.inner.session.title.is_none()
    }
    async fn generate_title(&mut self) -> anyhow::Result<()> {
        self.inner
            .session
            .generate_ai_title_with(
                self.inner.model.provider.as_ref(),
                &self.inner.model.model_id,
            )
            .await
    }
}
