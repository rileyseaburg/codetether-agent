use super::{ImageClient, ImageGeneration};
use anyhow::{Result, anyhow, ensure};
use codetether_gemini_web_protocol::images::{self, Continuation};

impl ImageClient {
    /// Generate images, optionally refining a previous image conversation.
    ///
    /// # Arguments
    /// * `prompt` - Nonempty image creation or editing instruction.
    /// * `previous` - Private continuation returned by an earlier generation.
    ///
    /// # Returns
    /// Checked image bytes and any new continuation metadata. Images have at
    /// most 100 million pixels and each response is capped at 32 MiB.
    ///
    /// # Errors
    /// Returns a credential-free error on cookie renewal, authentication,
    /// protocol, unsafe redirect, oversized response, or invalid image failure.
    /// Generation is not retried automatically, avoiding duplicate requests.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example(client: codetether_gemini_web_transport::images::ImageClient)
    /// # -> anyhow::Result<()> {
    /// let original = client.generate("Draw a red teapot", None).await?;
    /// let edited = client.generate("Make it blue", original.continuation.as_ref()).await?;
    /// assert!(!edited.images.is_empty());
    /// # Ok(()) }
    /// ```
    pub async fn generate(
        &self,
        prompt: &str,
        previous: Option<&Continuation>,
    ) -> Result<ImageGeneration> {
        if let Some(previous) = previous {
            previous
                .validate()
                .map_err(|_| anyhow!("image_continuation_invalid"))?;
        }
        let payload =
            images::build_payload(prompt, previous).map_err(|_| anyhow!("image_prompt_invalid"))?;
        let mut state = self.state.lock().await;
        let tokens = self.authenticate(&mut state).await?;
        let response = self.generate_request(&state, tokens, payload).await?;
        if let Some(previous) = previous {
            ensure!(
                response
                    .continuation
                    .as_ref()
                    .is_some_and(|c| c.conversation_id == previous.conversation_id),
                "image_refinement_lost_context"
            );
        }
        ensure!(
            !response.urls.is_empty() && response.urls.len() <= 8,
            "image_attachment_count_invalid"
        );
        let mut images = Vec::new();
        for url in response.urls {
            images.push(self.download(&url, &state).await?);
        }
        Ok(ImageGeneration {
            images,
            continuation: response.continuation,
        })
    }
}
#[path = "generate_request.rs"]
mod request;
