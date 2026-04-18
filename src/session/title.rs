//! Title generation and context-change hooks.

use anyhow::Result;
use chrono::Utc;

use super::helper::text::truncate_with_ellipsis;
use super::types::Session;

impl Session {
    /// Generate a title from the first user message if one is not already
    /// set.
    pub async fn generate_title(&mut self) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }
        self.set_title_from_first_user_message();
        Ok(())
    }

    /// Regenerate the title from the first user message, even if already
    /// set.
    pub async fn regenerate_title(&mut self) -> Result<()> {
        self.set_title_from_first_user_message();
        Ok(())
    }

    fn set_title_from_first_user_message(&mut self) {
        let Some(msg) = self
            .messages
            .iter()
            .find(|m| m.role == crate::provider::Role::User)
        else {
            return;
        };
        let text: String = msg
            .content
            .iter()
            .filter_map(|p| match p {
                crate::provider::ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        self.title = Some(truncate_with_ellipsis(&text, 47));
    }

    /// Set a custom title for the session.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
        self.updated_at = Utc::now();
    }

    /// Clear the title, allowing it to be regenerated on the next call to
    /// [`Session::generate_title`].
    pub fn clear_title(&mut self) {
        self.title = None;
        self.updated_at = Utc::now();
    }

    /// React to a context change (directory change, model change, etc.).
    /// Bumps `updated_at` and optionally regenerates the title.
    pub async fn on_context_change(&mut self, regenerate_title: bool) -> Result<()> {
        self.updated_at = Utc::now();
        if regenerate_title {
            self.regenerate_title().await?;
        }
        Ok(())
    }
}
