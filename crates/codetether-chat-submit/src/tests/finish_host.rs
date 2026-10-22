//! In-memory normal-submission host for publication regression tests.
#[derive(Default)]
pub(super) struct Host {
    pub published: String,
    pub dispatched: String,
}

impl crate::finish::Attachments for Host {
    type Image = ();
    type TextPastes = ();
    fn take_images(&mut self) -> Vec<()> {
        Vec::new()
    }
    fn take_text(&mut self) {}
    fn expand(&self, prompt: &str, _: &()) -> String {
        assert_eq!(prompt, super::COMPACT);
        super::EXPANDED.to_string()
    }
}

impl crate::finish::Actions for Host {
    async fn adopt(&mut self, _: &str) {}
    fn publish(&mut self, prompt: &str, _: &[()]) {
        self.published = prompt.to_string();
    }
    async fn dispatch(&mut self, prompt: &str, _: Vec<()>) {
        self.dispatched = prompt.to_string();
    }
}
