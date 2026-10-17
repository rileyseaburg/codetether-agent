//! Provider creation and credential validation.
use crate::*;
impl StepFunProvider {
    /// Construct a provider using the existing StepFun endpoint.
    ///
    /// # Arguments
    /// * api_key - Bearer credential for future requests.
    ///
    /// # Returns
    /// A provider using the process-wide shared client.
    ///
    /// # Errors
    /// This constructor currently always returns Ok.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_stepfun::StepFunProvider::new(String::new()).is_ok());
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "stepfun",
            api_key_len = api_key.len(),
            "Creating StepFun provider"
        );
        Ok(Self {
            api_key,
            client: codetether_provider_core::shared_http::shared_client().clone(),
        })
    }
    /// Validate that the API key is non-empty
    pub(crate) fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("StepFun API key is empty");
        }
        if self.api_key.len() < 10 {
            tracing::warn!(provider = "stepfun", "API key seems unusually short");
        }
        Ok(())
    }
}
