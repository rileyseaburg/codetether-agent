//! Credentialed Bedrock construction under the existing thinker policy.
use anyhow::{Result, anyhow};
use codetether_cognition_core::ThinkerConfig;
use codetether_provider_bedrock::provider::{AwsCredentials, BedrockProvider};
use std::sync::Arc;

pub(super) fn build(config: &ThinkerConfig) -> Result<Arc<BedrockProvider>> {
    let creds = AwsCredentials::from_environment().ok_or_else(|| {
        anyhow!(
            "Bedrock thinker requires AWS credentials (AWS_ACCESS_KEY_ID/AWS_SECRET_ACCESS_KEY or ~/.aws/credentials)"
        )
    })?;
    let provider = BedrockProvider::with_credentials(creds, config.bedrock_region.clone())?;
    Ok(Arc::new(provider))
}
