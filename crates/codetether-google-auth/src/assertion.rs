//! Sign a one-hour JWT assertion for the selected token endpoint.
use super::*;
impl ServiceAccountAuth {
    pub(super) fn assertion(&self) -> Result<(String, &str)> {
        // Sign a JWT assertion
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs();

        let token_uri = self.sa_key.token_uri.as_deref().unwrap_or(GOOGLE_TOKEN_URL);

        let claims = JwtClaims {
            iss: self.sa_key.client_email.clone(),
            scope: VERTEX_SCOPE.to_string(),
            aud: token_uri.to_string(),
            iat: now,
            exp: now + 3600,
        };

        let header = Header::new(Algorithm::RS256);
        let assertion = jsonwebtoken::encode(&header, &claims, &self.encoding_key)
            .context("Failed to sign JWT assertion")?;

        Ok((assertion, token_uri))
    }
}
