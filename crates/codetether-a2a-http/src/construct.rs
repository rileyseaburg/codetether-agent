//! Construct and configure an HTTP A2A client.
use super::*;
impl A2AClient {
    /// Create a new A2A client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            auth: None,
        }
    }

    /// Set authentication token
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(AuthenticationInfo {
            schemes: vec!["bearer".to_string()],
            credentials: Some(token.into()),
        });
        self
    }
}
