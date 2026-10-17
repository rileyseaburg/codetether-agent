//! Construct [BedrockProvider] with the application HTTP pool and credentials.
//!
//! Constructors preserve bearer/IAM selection and the original tracing fields.
use super::{AwsCredentials, BedrockAuth, BedrockProvider, DEFAULT_REGION};
use anyhow::Result;
impl BedrockProvider {
    /// Create a bearer-token provider in the default region.
    ///
    /// # Errors
    ///
    /// Currently infallible, but returns [`Result`] for API symmetry with
    /// [`BedrockProvider::with_credentials`] and future validation needs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::new("token-abc".into()).unwrap();
    /// assert_eq!(p.region(), "us-east-1");
    /// ```
    ///
    /// # Arguments
    /// * `api_key` - Bearer token used for requests.
    ///
    /// # Returns
    /// A provider in [DEFAULT_REGION].
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_region(api_key, DEFAULT_REGION.to_string())
    }

    /// Create a bearer-token provider with an explicit region.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::with_region("token".into(), "eu-west-1".into()).unwrap();
    /// assert_eq!(p.region(), "eu-west-1");
    /// ```
    ///
    /// # Arguments
    /// * `api_key` - Bearer token used for requests.
    /// * `region` - AWS region used for URLs and signing.
    ///
    /// # Returns
    /// A provider using the shared application HTTP pool.
    ///
    /// # Errors
    /// Currently infallible; preserves the common constructor result type.
    pub fn with_region(api_key: String, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            auth = "bearer_token",
            "Creating Bedrock provider"
        );
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            auth: BedrockAuth::bearer(api_key),
            region,
        })
    }

    /// Create a SigV4 provider from AWS IAM credentials.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-east-1".into()).unwrap();
    /// assert_eq!(p.region(), "us-east-1");
    /// ```
    ///
    /// # Arguments
    /// * `credentials` - IAM credentials used for SigV4 signing.
    /// * `region` - AWS region used for URLs and signing.
    ///
    /// # Returns
    /// A provider using the shared application HTTP pool.
    ///
    /// # Errors
    /// Currently infallible; preserves the common constructor result type.
    pub fn with_credentials(credentials: AwsCredentials, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            auth = "sigv4",
            "Creating Bedrock provider with AWS credentials"
        );
        Ok(Self {
            client: crate::provider::shared_http::shared_client().clone(),
            auth: BedrockAuth::SigV4(credentials),
            region,
        })
    }
}
