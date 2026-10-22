use anyhow::Result;
use async_trait::async_trait;
use codetether_context_policy::summary_index::{
    Granularity, SummaryIndex, SummaryNode, SummaryProvenance, SummaryRange,
};
use codetether_context_summarize::SummarySource;
use codetether_provider_core::{ContentPart, Message, Provider, Role};
use std::{path::Path, sync::Arc};

pub struct Session {
    messages: Vec<Message>,
    index: SummaryIndex,
}
fn fixture(id: &str) -> Session {
    let mut messages = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Do not publish".into(),
        }],
    }];
    let mut provenance =
        Some(SummaryProvenance::new(&messages, "fixture", Some("model-before-switch")).unwrap());
    match id {
        "edited" => {
            messages[0].content = vec![ContentPart::Text {
                text: "Publish later".into(),
            }]
        }
        "legacy" => provenance = None,
        "policy" => provenance.as_mut().unwrap().policy_version = "old".into(),
        "valid" => {}
        _ => panic!("unknown fixture"),
    }
    let mut index = SummaryIndex::new();
    index.insert(
        SummaryRange::new(0, 1).unwrap(),
        SummaryNode {
            content: "CACHED EVIDENCE".into(),
            target_tokens: 128,
            granularity: Granularity::Turn,
            generation: 0,
            provenance,
        },
    );
    Session { messages, index }
}
#[async_trait]
impl codetether_context_loader::Source for Session {
    async fn load_tail(id: &str, _: usize) -> Result<Self> {
        Ok(fixture(id))
    }
    async fn last_for_directory_tail(_: Option<&Path>, _: usize) -> Result<Self> {
        unreachable!()
    }
    async fn last_for_directory(_: Option<&Path>) -> Result<Self> {
        unreachable!()
    }
}
#[async_trait]
impl SummarySource for Session {
    type Prepared = ();
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn summary_index(&self) -> &SummaryIndex {
        &self.index
    }
    fn summary_index_mut(&mut self) -> &mut SummaryIndex {
        &mut self.index
    }
    fn prepare(&self) {}
    async fn produce(
        _: (),
        _: SummaryRange,
        _: usize,
        _: Arc<dyn Provider>,
        _: &str,
    ) -> Result<SummaryNode> {
        unreachable!()
    }
    async fn persist_summary(&mut self, _: SummaryRange, _: &SummaryNode) -> Result<()> {
        unreachable!()
    }
}
