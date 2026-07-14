#[derive(Deserialize)]
struct OAuthTokenResponse {
    #[serde(default)]
    id_token: Option<String>,
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}
