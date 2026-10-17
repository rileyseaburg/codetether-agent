//! Reject bearer authentication for the IAM signing path.
use super::*;
impl SigningClient<'_> {
    pub(super) fn credentials(&self) -> Result<&AwsCredentials> {
        let creds = match &self.auth {
            BedrockAuth::SigV4(c) => c,
            BedrockAuth::BearerToken(_) => {
                anyhow::bail!("send_signed_request called with bearer token auth");
            }
        };
        Ok(creds)
    }
}
