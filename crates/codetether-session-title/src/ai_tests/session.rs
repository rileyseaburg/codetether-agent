//! In-memory title lifecycle fixture with observable timestamp writes.
use crate::TitleSession;
use codetether_provider_core::{ContentPart, Message, Role};

pub(super) struct Session {
    pub title: Option<String>,
    pub messages: Vec<Message>,
    pub touches: usize,
}

impl Session {
    pub fn new(seed: &str) -> Self {
        Self {
            title: None,
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: seed.into() }],
            }],
            touches: 0,
        }
    }
}

impl TitleSession for Session {
    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn messages(&self) -> &[Message] {
        &self.messages
    }

    fn store_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    fn touch(&mut self) {
        self.touches += 1;
    }
}
