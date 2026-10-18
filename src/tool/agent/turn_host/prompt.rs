//! Native plain and image-capable prompt APIs used by child tasks.
use super::ApplicationHost;
use crate::{
    provider::ProviderRegistry,
    session::{Session, SessionEvent},
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_child_turn::PromptHost;
use codetether_collaboration_input::MessageImage;
use std::sync::Arc;
use tokio::sync::mpsc;
#[async_trait]
impl PromptHost for ApplicationHost {
    type Registry = ProviderRegistry;
    async fn registry() -> Result<Arc<ProviderRegistry>> {
        crate::tool::agent::helpers::get_registry().await
    }
    async fn prompt(
        mut session: Session,
        text: String,
        images: Option<Vec<MessageImage>>,
        tx: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> Result<Session> {
        match images {
            Some(images) => session
                .prompt_with_events_and_images(
                    &text,
                    images.into_iter().map(Into::into).collect(),
                    tx,
                    registry,
                )
                .await
                .map(|_| ()),
            None => session
                .prompt_with_events(&text, tx, registry)
                .await
                .map(|_| ()),
        }
        .map(|_| session)
    }
}
