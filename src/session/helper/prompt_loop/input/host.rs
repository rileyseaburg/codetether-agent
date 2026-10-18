//! Native human-message, publication and title services.
use super::super::Runner;
use crate::provider::Message;
impl codetether_prompt_loop::input::Host for Runner<'_> {
    fn append_human(&mut self, message: Message) {
        self.session.add_human_message(message);
    }
    fn turn_id(&self) -> &str {
        &self.progress.turn_id
    }
    fn publish(&self, message: &str, turn_id: String) {
        super::super::super::publish_user_prompt::publish(self.session, message, Some(turn_id));
    }
    fn needs_title(&self) -> bool {
        self.session.title.is_none()
    }
    async fn generate_title(&mut self) -> anyhow::Result<()> {
        self.session
            .generate_ai_title_with(self.model.provider.as_ref(), &self.model.model_id)
            .await
    }
}
