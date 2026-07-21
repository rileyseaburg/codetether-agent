fn refresh_required(
    credentials: &OAuthCredentials,
    now: u64,
    rejected_token: Option<&str>,
) -> bool {
    match rejected_token {
        Some(rejected) => credentials.access_token == rejected,
        None => credentials.expires_at <= now.saturating_add(300),
    }
}
