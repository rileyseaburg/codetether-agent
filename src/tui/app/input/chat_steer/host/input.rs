use super::Native;
use crate::{
    session::{ImageAttachment, helper::steering::SteeringInput},
    tui::app::{input::pasted_text::expand_paste_placeholders, state::prompt_queue},
};
impl codetether_chat_submit::steering::Input for Native<'_> {
    fn images(&self) -> Vec<ImageAttachment> {
        self.app.state.pending_images.clone()
    }
    fn expand(&self, prompt: &str) -> String {
        expand_paste_placeholders(prompt, &self.app.state.pending_text_pastes)
    }
    fn steer(&self, expanded: String, images: Vec<ImageAttachment>) -> bool {
        self.runtime
            .steer_current(SteeringInput::new(expanded, images))
    }
    fn queue(&mut self, prompt: String) {
        prompt_queue::store(self.app, prompt);
    }
}
