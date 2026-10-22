use async_trait::async_trait;
use codetether_context_policy::{
    derivation::DerivePolicy, pages::PageKind, state_header::ConstraintRecord,
    summary_index::SummaryIndex,
};
use codetether_provider_core::{ContentPart, Message, Provider, Role};
use codetether_rlm::RlmConfig;
use std::sync::Arc;
mod provider;
pub use provider::NoCalls;

pub struct Session {
    pub messages: Vec<Message>,
    pub pins: Vec<ConstraintRecord>,
    pub corrections: Vec<codetether_context_policy::state_header::ConstraintCorrection>,
    config: RlmConfig,
}
pub fn message(text: String) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text }],
    }
}
impl Session {
    pub fn new() -> Self {
        let pin = ConstraintRecord::from_message(
            0,
            &message(format!("permanent constraint\n{}", "keep ".repeat(600))),
            PageKind::Constraint,
        )
        .unwrap();
        let mut messages: Vec<_> = (0..40)
            .map(|n| message(format!("old exchange {n} {}", "context ".repeat(120))))
            .collect();
        messages.push(message("Investigate the requested implementation".into()));
        Self {
            messages,
            pins: vec![pin],
            corrections: Vec::new(),
            config: RlmConfig::default(),
        }
    }
}
impl codetether_context_legacy::Source for Session {
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn session_id(&self) -> &str {
        "fixture"
    }
    fn rlm_config(&self) -> &RlmConfig {
        &self.config
    }
}
impl codetether_reset_derivation::Source for Session {
    fn subcall_provider(&self) -> Option<&Arc<dyn Provider>> {
        None
    }
    fn subcall_model_name(&self) -> Option<&str> {
        None
    }
}
#[async_trait]
impl codetether_incremental_derivation::Source for Session {
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn pages(&self) -> &[PageKind] {
        &[]
    }
    fn source_offset(&self) -> usize {
        100
    }
    async fn prepared_index(&self) -> SummaryIndex {
        SummaryIndex::new()
    }
}
impl codetether_context_dispatch::Source for Session {
    fn saved_policy(&self) -> DerivePolicy {
        DerivePolicy::default()
    }
    fn constraint_prefix(&self) -> Vec<ConstraintRecord> {
        self.pins.clone()
    }
    fn constraint_corrections(
        &self,
    ) -> &[codetether_context_policy::state_header::ConstraintCorrection] {
        &self.corrections
    }
    fn message_offset(&self) -> usize {
        100
    }
}
pub fn hooks() -> codetether_context_legacy::Hooks {
    codetether_context_legacy::Hooks {
        apply: |_| Default::default(),
        repair: |_| Default::default(),
        resolver: |provider, model, _, _| {
            let model = model.to_owned();
            Box::pin(async move { (provider, model) })
        },
    }
}
