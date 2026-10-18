use super::super::ThinkerOutput;
use super::{Client, Host, backend::Backend};
use anyhow::Result;
impl<H: Host> Client<H> {
    /// Generate a thinking completion using the configured backend.
    ///
    /// # Errors
    ///
    /// Propagates backend transport, decoding, and inference failures.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let client = Client::<NativeHost>::new(config)?;
    /// let output = client.think("system", "prompt").await?;
    /// ```
    pub async fn think(&self, system_prompt: &str, user_prompt: &str) -> Result<ThinkerOutput> {
        match &self.backend {
            Backend::OpenAICompat { http } => {
                H::http(&self.config, http, system_prompt, user_prompt).await
            }
            Backend::Registry => H::registry(&self.config, system_prompt, user_prompt).await,
            Backend::Bedrock { provider } => {
                H::bedrock_think(&self.config, provider, system_prompt, user_prompt).await
            }
            Backend::Candle { runtime } => {
                H::native_think(runtime, "candle", system_prompt, user_prompt).await
            }
        }
    }
}
