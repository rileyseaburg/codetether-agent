use async_trait::async_trait;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::{ContentPart, Message, Role};
use std::path::Path;
static SAVED: std::sync::Mutex<Vec<PageKind>> = std::sync::Mutex::new(Vec::new());
pub fn saved() -> Vec<PageKind> {
    SAVED.lock().unwrap().clone()
}

pub(super) struct Session {
    pub pages: Vec<PageKind>,
    pub messages: Vec<Message>,
    pub saves: usize,
    runtime: bool,
    pub corrections: Vec<codetether_context_policy::state_header::ConstraintCorrection>,
    pub prefix: Vec<codetether_context_policy::state_header::ConstraintRecord>,
    pub fail_save: bool,
}
impl Session {
    pub fn new() -> Self {
        Self {
            pages: vec![
                PageKind::Constraint,
                PageKind::Conversation,
                PageKind::Conversation,
            ],
            messages: [Role::User, Role::Assistant, Role::User]
                .into_iter()
                .map(|role| Message {
                    role,
                    content: vec![ContentPart::Text {
                        text: "source".into(),
                    }],
                })
                .collect(),
            saves: 0,
            runtime: false,
            corrections: Vec::new(),
            prefix: Vec::new(),
            fail_save: false,
        }
    }
}
#[async_trait]
impl codetether_context_loader::Source for Session {
    async fn load_tail(id: &str, _: usize) -> anyhow::Result<Self> {
        anyhow::ensure!(
            matches!(id, "runtime-session" | "runtime-correction"),
            "unexpected session"
        );
        let mut session = Self::new();
        session.runtime = id == "runtime-session";
        Ok(session)
    }
    async fn last_for_directory_tail(_: Option<&Path>, _: usize) -> anyhow::Result<Self> {
        anyhow::bail!("unused")
    }
    async fn last_for_directory(_: Option<&Path>) -> anyhow::Result<Self> {
        anyhow::bail!("unused")
    }
}
#[async_trait]
impl crate::PinSource for Session {
    fn page_count(&self) -> usize {
        self.pages.len()
    }
    fn page_offset(&self) -> usize {
        100
    }
    fn page_kind(&self, index: usize) -> Option<PageKind> {
        self.pages.get(index).copied()
    }
    fn source_message(&self, index: usize) -> Option<Message> {
        self.messages.get(index).cloned()
    }
    fn set_page_kind(&mut self, index: usize, kind: PageKind) {
        self.pages[index] = kind;
    }
    fn corrections(&self) -> &[codetether_context_policy::state_header::ConstraintCorrection] {
        &self.corrections
    }
    fn constraint_source(
        &self,
        absolute: usize,
    ) -> Option<codetether_context_policy::state_header::ConstraintRecord> {
        if let Some(index) = absolute.checked_sub(100) {
            if let (Some(message), Some(page)) = (self.messages.get(index), self.pages.get(index)) {
                return codetether_context_policy::state_header::ConstraintRecord::from_message(
                    absolute, message, *page,
                );
            }
        }
        self.prefix
            .iter()
            .find(|source| source.source_turn == absolute)
            .cloned()
    }
    fn set_corrections(
        &mut self,
        corrections: Vec<codetether_context_policy::state_header::ConstraintCorrection>,
    ) {
        self.corrections = corrections;
    }
    async fn persist(&mut self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.fail_save, "injected save failure");
        self.saves += 1;
        if self.runtime {
            *SAVED.lock().unwrap() = self.pages.clone();
        }
        Ok(())
    }
}
