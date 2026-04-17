//! AWS credential loading and auth mode selection for the Bedrock provider.
//!
//! Supports credentials from environment variables, `~/.aws/credentials`, and
//! region detection via env vars or `~/.aws/config`.
//!
//! # Examples
//!
//! ```rust,no_run
//! use codetether_agent::provider::bedrock::AwsCredentials;
//!
//! // Load from env vars, falling back to ~/.aws/credentials
//! let creds = AwsCredentials::from_environment()
//!     .expect("no AWS credentials found");
//! let region = AwsCredentials::detect_region()
//!     .unwrap_or_else(|| "us-east-1".to_string());
//! assert!(!creds.access_key_id.is_empty());
//! assert!(!region.is_empty());
//! ```

/// AWS credentials used for SigV4 signing of Bedrock requests.
///
/// Typically constructed via [`AwsCredentials::from_environment`] rather than
/// directly, so that env vars and the shared credentials file are both
/// respected.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::AwsCredentials;
///
/// let creds = AwsCredentials {
///     access_key_id: "AKIA...".to_string(),
///     secret_access_key: "secret".to_string(),
///     session_token: None,
/// };
/// assert_eq!(creds.access_key_id, "AKIA...");
/// assert!(creds.session_token.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct AwsCredentials {
    /// AWS access key ID (e.g. `AKIA...`).
    pub access_key_id: String,
    /// AWS secret access key (keep confidential).
    pub secret_access_key: String,
    /// Optional session token (present for STS / assumed-role credentials).
    pub session_token: Option<String>,
}

impl AwsCredentials {
    /// Load credentials from `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY`
    /// env vars, then fall back to `~/.aws/credentials` (default or named
    /// profile via `AWS_PROFILE`).
    ///
    /// # Returns
    ///
    /// `Some(creds)` on success, `None` if neither env vars nor a readable
    /// credentials file yielded a complete key pair.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::bedrock::AwsCredentials;
    ///
    /// match AwsCredentials::from_environment() {
    ///     Some(c) => println!("loaded key id: {}", c.access_key_id),
    ///     None => eprintln!("no AWS credentials available"),
    /// }
    /// ```
    pub fn from_environment() -> Option<Self> {
        if let (Ok(key_id), Ok(secret)) = (
            std::env::var("AWS_ACCESS_KEY_ID"),
            std::env::var("AWS_SECRET_ACCESS_KEY"),
        ) && !key_id.is_empty()
            && !secret.is_empty()
        {
            return Some(Self {
                access_key_id: key_id,
                secret_access_key: secret,
                session_token: std::env::var("AWS_SESSION_TOKEN")
                    .ok()
                    .filter(|s| !s.is_empty()),
            });
        }

        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        Self::from_credentials_file(&profile)
    }

    /// Parse `~/.aws/credentials` INI file for the given profile.
    ///
    /// # Arguments
    ///
    /// * `profile` — Profile name (e.g. `"default"`, `"prod"`).
    ///
    /// # Returns
    ///
    /// `Some(creds)` if a matching `[profile]` section with both
    /// `aws_access_key_id` and `aws_secret_access_key` was found.
    fn from_credentials_file(profile: &str) -> Option<Self> {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()?;
        let path = std::path::Path::new(&home).join(".aws").join("credentials");
        let content = std::fs::read_to_string(&path).ok()?;

        let section_header = format!("[{profile}]");
        let mut in_section = false;
        let mut key_id = None;
        let mut secret = None;
        let mut token = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_section = trimmed == section_header;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                match k {
                    "aws_access_key_id" => key_id = Some(v.to_string()),
                    "aws_secret_access_key" => secret = Some(v.to_string()),
                    "aws_session_token" => token = Some(v.to_string()),
                    _ => {}
                }
            }
        }

        Some(Self {
            access_key_id: key_id?,
            secret_access_key: secret?,
            session_token: token,
        })
    }

    /// Detect the target AWS region.
    ///
    /// Precedence: `AWS_REGION` → `AWS_DEFAULT_REGION` → `~/.aws/config`
    /// (default or `[profile NAME]` per `AWS_PROFILE`).
    ///
    /// # Returns
    ///
    /// `Some(region)` on success, `None` if no region could be determined.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::bedrock::AwsCredentials;
    ///
    /// let region = AwsCredentials::detect_region()
    ///     .unwrap_or_else(|| "us-east-1".to_string());
    /// assert!(!region.is_empty());
    /// ```
    pub fn detect_region() -> Option<String> {
        if let Ok(r) = std::env::var("AWS_REGION")
            && !r.is_empty()
        {
            return Some(r);
        }
        if let Ok(r) = std::env::var("AWS_DEFAULT_REGION")
            && !r.is_empty()
        {
            return Some(r);
        }
        let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()?;
        let path = std::path::Path::new(&home).join(".aws").join("config");
        let content = std::fs::read_to_string(&path).ok()?;

        let section_header = if profile == "default" {
            "[default]".to_string()
        } else {
            format!("[profile {profile}]")
        };
        let mut in_section = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') {
                in_section = trimmed == section_header;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=')
                && k.trim() == "region"
            {
                let v = v.trim();
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
        None
    }
}

/// Authentication mode for the Bedrock provider.
///
/// Bedrock supports two auth modes:
/// - **SigV4**: standard AWS IAM credentials (access key + secret).
/// - **Bearer token**: an opaque token from an API Gateway fronting Bedrock or
///   a Vault-managed key dispensary.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockAuth};
///
/// let sigv4 = BedrockAuth::SigV4(AwsCredentials {
///     access_key_id: "AKIA...".into(),
///     secret_access_key: "secret".into(),
///     session_token: None,
/// });
/// let bearer = BedrockAuth::BearerToken("token-abc".into());
///
/// match sigv4 {
///     BedrockAuth::SigV4(_) => (),
///     BedrockAuth::BearerToken(_) => panic!("unexpected"),
/// }
/// match bearer {
///     BedrockAuth::BearerToken(t) => assert_eq!(t, "token-abc"),
///     BedrockAuth::SigV4(_) => panic!("unexpected"),
/// }
/// ```
#[derive(Debug, Clone)]
pub enum BedrockAuth {
    /// Standard AWS SigV4 signing with IAM credentials.
    SigV4(AwsCredentials),
    /// Bearer token (API Gateway or custom auth layer).
    BearerToken(String),
}
