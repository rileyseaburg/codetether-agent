use super::super::ThinkerOutput;
use super::{Client, Host, backend::Backend};
use anyhow::{Result, anyhow};
impl<H: Host> Client<H> {
    /// Run the loaded native model under its enclosing provider's identity.
    ///
    /// # Errors
    /// Rejects non-native backends and propagates native inference failures.
    pub async fn think_as_provider(
        &self,
        provider: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<ThinkerOutput> {
        let Backend::Candle { runtime } = &self.backend else {
            return Err(anyhow!(
                "provider identity override requires a native backend"
            ));
        };
        H::native_think(runtime, provider, system_prompt, user_prompt).await
    }
}
