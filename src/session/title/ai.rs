//! AI-generated session titles.
//!
//! Produces a short, human-readable name for a session by asking a cheap
//! model to summarize the first user message. Falls back silently to the
//! existing first-message heuristic on any error so titling never blocks a
//! prompt. The model call itself lives in [`ai_model`](super::ai_model).

use anyhow::Result;

use crate::provider::{ContentPart, ProviderRegistry, Role};

use super::super::types::Session;
use super::ai_model::title_from_model;

impl Session {
    /// Generate an AI title from the first user message, falling back to the
    /// truncated-first-message heuristic when no model is reachable.
    pub async fn generate_ai_title(&mut self, registry: &ProviderRegistry) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }
        let Some(seed) = self.first_user_text() else {
            return Ok(());
        };
        match title_from_model(registry, &seed).await {
            Ok(Some(title)) => self.set_title(title),
            _ => self.generate_title().await?,
        }
        Ok(())
    }

    /// Extract the first user message's plain text, if any.
    pub(crate) fn first_user_text(&self) -> Option<String> {
        let msg = self.messages.iter().find(|m| m.role == Role::User)?;
        let text: String = msg
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        (!text.trim().is_empty()).then_some(text)
    }
}
