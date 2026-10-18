//! Credentialed Bedrock construction retained by the application.
use super::ThinkerConfig;
use crate::provider::bedrock::{AwsCredentials, BedrockProvider};
use anyhow::{Result, anyhow};
use std::sync::Arc;
pub(super) fn bedrock(config: &ThinkerConfig) -> Result<Arc<BedrockProvider>> {
    let creds = AwsCredentials::from_environment().ok_or_else(|| {
        anyhow!(
            "Bedrock thinker requires AWS credentials (AWS_ACCESS_KEY_ID/AWS_SECRET_ACCESS_KEY or ~/.aws/credentials)"
        )
    })?;
    let provider = BedrockProvider::with_credentials(creds, config.bedrock_region.clone())?;
    Ok(Arc::new(provider))
}
