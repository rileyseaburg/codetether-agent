//! Application-owned Vault/SSO recovery for Bedrock client request loops.
use crate::provider::{BedrockProvider, auth_recover};
#[async_trait::async_trait]
impl crate::recovery::CredentialRecovery for BedrockProvider {
    async fn recover(&self) -> bool {
        auth_recover::recover(self).await
    }
}
