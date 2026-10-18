//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_bedrock<H: crate::Host>(
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    use codetether_bedrock_auth::AwsCredentials;
    let region = secrets
        .extra
        .get("region")
        .and_then(|v| v.as_str())
        .unwrap_or("us-east-1")
        .to_string();
    let key_id = secrets
        .extra
        .get("aws_access_key_id")
        .and_then(|v| v.as_str());
    let secret = secrets
        .extra
        .get("aws_secret_access_key")
        .and_then(|v| v.as_str());
    let result = if let (Some(kid), Some(sec)) = (key_id, secret) {
        let creds = AwsCredentials {
            access_key_id: kid.into(),
            secret_access_key: sec.into(),
            session_token: secrets
                .extra
                .get("aws_session_token")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
        H::bedrock_with_credentials(creds, region)
    } else if let Some(ref key) = secrets.api_key {
        H::bedrock_with_region(key.clone(), region)
    } else if !crate::fallback_policy::env_fallback_disabled()
        && let Some(creds) = AwsCredentials::from_environment()
    {
        H::bedrock_with_credentials(creds, region)
    } else {
        tracing::warn!("No AWS credentials or API key found for Bedrock");
        return None;
    };
    match result {
        Ok(p) => Some(p),
        Err(e) => {
            tracing::warn!("Failed to init bedrock: {e}");
            None
        }
    }
}
