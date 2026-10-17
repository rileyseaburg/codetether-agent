//! Borrow configured provider state for HTTP and discovery operations.
use super::BedrockProvider;
impl BedrockProvider {
    pub(super) fn bedrock_client(&self) -> codetether_provider_bedrock::BedrockClient<'_> {
        codetether_provider_bedrock::BedrockClient::new(&self.client, &self.auth, &self.region)
    }
}

#[path = "client_recovery.rs"]
mod recovery;
