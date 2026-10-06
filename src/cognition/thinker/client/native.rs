//! Native provider dispatch retaining the enclosing provider identity.

use super::super::{ThinkerOutput, candle_dispatch};
use super::{ThinkerClient, ThinkerClientBackend};
use anyhow::{Result, anyhow};

impl ThinkerClient {
    /// Run the loaded native model under its enclosing provider's identity.
    ///
    /// # Errors
    /// Rejects non-native backends and propagates native inference failures.
    pub(crate) async fn think_as_provider(
        &self,
        provider: &str,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<ThinkerOutput> {
        let ThinkerClientBackend::Candle { runtime } = &self.backend else {
            return Err(anyhow!(
                "provider identity override requires a native backend"
            ));
        };
        candle_dispatch::think(runtime, provider, system_prompt, user_prompt).await
    }
}

#[cfg(test)]
#[path = "native_tests.rs"]
mod tests;
