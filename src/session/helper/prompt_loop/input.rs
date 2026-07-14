//! Ingestion of user text and image attachments.

use super::Runner;
use crate::provider::{ContentPart, Message, Role};
use crate::session::ImageAttachment;
use anyhow::Result;

impl Runner<'_> {
    /// Adds user content, publishes it, and generates a title when needed.
    ///
    /// # Errors
    ///
    /// Returns an error when automatic title generation fails.
    pub(crate) async fn accept(
        &mut self,
        message: &str,
        images: Vec<ImageAttachment>,
    ) -> Result<()> {
        let mut content = vec![ContentPart::Text {
            text: message.to_string(),
        }];
        content.extend(images.iter().map(|image| ContentPart::Image {
            url: image.data_url.clone(),
            mime_type: image.mime_type.clone(),
        }));
        if !images.is_empty() {
            tracing::info!(image_count = images.len(), "Adding image attachments");
        }
        self.session.add_human_message(Message {
            role: Role::User,
            content,
        });
        super::super::publish_user_prompt::publish(self.session, message);
        if self.session.title.is_none() {
            self.session.generate_ai_title(&self.registry).await?;
        }
        Ok(())
    }
}
