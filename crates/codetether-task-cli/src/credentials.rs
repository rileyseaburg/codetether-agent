use anyhow::Result;
use codetether_user_auth::load_saved_credentials;

pub(crate) fn resolve_server_and_token(
    default_server: Option<String>,
    default_token: Option<String>,
) -> Result<(String, Option<String>)> {
    let saved = load_saved_credentials();

    let server = default_server
        .or_else(|| saved.as_ref().map(|creds| creds.server.clone()))
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No CodeTether server configured. Pass --server or run `codetether auth login`."
            )
        })?;

    let token = match default_token {
        Some(token) => Some(token),
        None => saved.and_then(|creds| {
            let saved_server = creds.server.trim().trim_end_matches('/');
            if saved_server == server {
                Some(creds.access_token)
            } else {
                None
            }
        }),
    };

    Ok((server, token))
}
