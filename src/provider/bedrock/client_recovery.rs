//! Application-owned Vault/SSO recovery for Bedrock client request loops.
use crate::provider::bedrock::{BedrockProvider, auth_recover};
#[async_trait::async_trait]
impl codetether_provider_bedrock::recovery::CredentialRecovery for BedrockProvider {
    async fn recover(&self) -> bool {
        auth_recover::recover(self).await
    }
}
