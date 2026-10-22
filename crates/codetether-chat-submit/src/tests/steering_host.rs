//! In-memory active-turn host for publication and rejection regression tests.
use codetether_chat_model::ChatMessage;
use codetether_image::ImageAttachment;

#[derive(Default)]
pub(super) struct Host {
    pub messages: Vec<ChatMessage>,
    pub accept: bool,
    pub cleared: bool,
    pub queued: String,
}

impl crate::steering::Input for Host {
    fn images(&self) -> Vec<ImageAttachment> {
        Vec::new()
    }
    fn expand(&self, prompt: &str) -> String {
        assert_eq!(prompt, super::COMPACT);
        super::EXPANDED.to_string()
    }
    fn steer(&self, expanded: String, _: Vec<ImageAttachment>) -> bool {
        assert_eq!(expanded, super::EXPANDED);
        self.accept
    }
    fn queue(&mut self, prompt: String) {
        self.queued = prompt;
    }
}

impl crate::steering::View for Host {
    fn clear_attachments(&mut self) {
        self.cleared = true;
    }
    fn messages(&mut self) -> &mut Vec<ChatMessage> {
        &mut self.messages
    }
    fn clear_input(&mut self) {}
    fn status(&mut self, _: String) {}
    fn scroll(&mut self) {}
}
