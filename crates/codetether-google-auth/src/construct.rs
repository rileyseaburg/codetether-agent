//! Parse credentials and resolve the project before constructing auth state.
use super::*;
impl ServiceAccountAuth {
    /// Parse service-account credentials and resolve their project.
    ///
    /// # Arguments
    /// * sa_json - Service-account JSON containing email and RSA private key.
    /// * project_id - Optional override for the project in the credential file.
    ///
    /// # Returns
    /// Authentication state and the selected project ID.
    ///
    /// # Errors
    /// Returns errors for invalid JSON, a missing project, or an invalid RSA key,
    /// in that order.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_google_auth::ServiceAccountAuth::from_json("{}", None).is_err());
    pub fn from_json(sa_json: &str, project_id: Option<String>) -> Result<(Self, String)> {
        let sa_key: ServiceAccountKey =
            serde_json::from_str(sa_json).context("Failed to parse service account JSON key")?;

        let project_id = project_id
            .or_else(|| sa_key.project_id.clone())
            .ok_or_else(|| anyhow::anyhow!("No project_id found in SA key or Vault config"))?;

        let encoding_key = EncodingKey::from_rsa_pem(sa_key.private_key.as_bytes())
            .context("Failed to parse RSA private key from service account")?;

        Ok((
            Self {
                sa_key,
                encoding_key,
                cached_token: Arc::new(RwLock::new(None)),
            },
            project_id,
        ))
    }
}
