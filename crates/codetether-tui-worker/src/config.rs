//! Resolve an optional server and bearer token before starting the bridge.
use anyhow::Result;
use codetether_config::Config;
use codetether_user_auth::load_saved_credentials;
pub(crate) async fn resolve(
    server_url: Option<String>,
    token: Option<String>,
) -> Result<Option<(String, String)>> {
    // Try to get server URL from config if not provided
    let server = match server_url {
        Some(url) => url,
        None => {
            let config = Config::load().await?;
            match config.a2a.server_url {
                Some(url) => url,
                None => {
                    tracing::debug!("No A2A server configured, worker bridge disabled");
                    return Ok(None);
                }
            }
        }
    };

    // Try to get token from saved credentials if not provided
    let token = match token {
        Some(t) => Some(t),
        None => load_saved_credentials().map(|c| c.access_token),
    };

    // If no token, we can't register - but we could run in "read-only" mode
    // For now, let's require a token for registration
    let token = match token {
        Some(t) => t,
        None => {
            tracing::debug!("No A2A token available, worker bridge disabled");
            return Ok(None);
        }
    };

    Ok(Some((server, token)))
}
