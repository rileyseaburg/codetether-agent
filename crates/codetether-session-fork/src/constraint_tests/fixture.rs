use crate::ForkSession;
use codetether_context_policy::{
    pages::PageKind,
    state_header::{ConstraintCorrection, ConstraintRecord},
};
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_session_metadata::SessionMetadata;

#[derive(Default)]
pub(crate) struct Session {
    pub messages: Vec<Message>,
    pub pages: Vec<PageKind>,
    pub metadata: SessionMetadata,
    pub offset: usize,
}
pub fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
pub fn parent() -> Session {
    let messages = vec![
        message("Keep evidence. Use red."),
        message("Use blue."),
        message("Continue"),
    ];
    let old = ConstraintRecord::from_message(100, &messages[0], PageKind::Constraint).unwrap();
    let next = ConstraintRecord::from_message(101, &messages[1], PageKind::Constraint).unwrap();
    let mut metadata = SessionMetadata::default();
    metadata.constraint_corrections.push(ConstraintCorrection {
        source_turn: 100,
        source_hash: old.text_hash,
        start: 15,
        end: 23,
        replacement_turn: 101,
        replacement_hash: next.text_hash,
    });
    Session {
        messages,
        pages: vec![
            PageKind::Constraint,
            PageKind::Constraint,
            PageKind::Conversation,
        ],
        metadata,
        offset: 100,
    }
}
#[async_trait::async_trait]
impl ForkSession for Session {
    async fn new() -> anyhow::Result<Self> {
        Ok(Self::default())
    }
    async fn load(_: &str) -> anyhow::Result<Self> {
        anyhow::bail!("not used")
    }
    async fn save(&self) -> anyhow::Result<()> {
        Ok(())
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn message_offset(&self) -> usize {
        self.offset
    }
    fn page_kinds(&self) -> &[PageKind] {
        &self.pages
    }
    fn replace_pages(&mut self, pages: Vec<PageKind>) {
        self.pages = pages;
    }
    fn title(&self) -> Option<&str> {
        None
    }
    fn metadata(&self) -> &SessionMetadata {
        &self.metadata
    }
    fn id(&self) -> &str {
        "fixture"
    }
    fn agent(&self) -> &str {
        "build"
    }
    fn set_title(&mut self, _: Option<String>) {}
    fn set_metadata(&mut self, metadata: SessionMetadata) {
        self.metadata = metadata;
    }
    fn set_agent_name(&mut self, _: &str) {}
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.pages.push(PageKind::Conversation);
    }
}
impl crate::detached::Session for Session {
    fn replace_transcript(&mut self, messages: Vec<Message>) {
        self.messages = messages;
        self.pages = vec![PageKind::Conversation; self.messages.len()];
    }
    fn set_detached_agent(&mut self) {}
    fn attach_bus_if_missing(&mut self) {}
}
impl crate::interactive::Session for Session {
    fn pages(&self) -> &[PageKind] {
        &self.pages
    }
    fn install_history(&mut self, messages: Vec<Message>, pages: Vec<PageKind>) {
        self.messages = messages;
        self.pages = pages;
    }
    fn metadata_mut(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
    fn attach_global_bus_if_missing(&mut self) {}
}
