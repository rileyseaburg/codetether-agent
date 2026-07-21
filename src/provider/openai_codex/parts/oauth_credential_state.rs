struct OAuthCredentialState {
    credentials: OAuthCredentials,
    pending_refresh_token: Option<String>,
}

impl OAuthCredentialState {
    fn new(credentials: OAuthCredentials) -> Self {
        Self {
            credentials,
            pending_refresh_token: None,
        }
    }
}
