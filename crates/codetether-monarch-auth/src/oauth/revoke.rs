//! Best-effort remote revocation of both refresh and access tokens.
use super::credentials::Credentials;
use crate::discovery::{self, REVOKE};
use anyhow::{Result, anyhow, ensure};

pub(crate) async fn revoke(credentials: &Credentials) -> Result<()> {
    let failure =
        || anyhow!("Monarch remote revocation failed. Disconnect CodeTether in Monarch settings");
    let client = discovery::client().map_err(|_| failure())?;
    discovery::discover(&client).await.map_err(|_| failure())?;
    let mut failed = false;
    for (token, hint) in [
        (credentials.refresh_token.as_deref(), "refresh_token"),
        (Some(credentials.access_token.as_str()), "access_token"),
    ] {
        let Some(token) = token else {
            continue;
        };
        let result = client
            .post(REVOKE)
            .form(&[
                ("token", token),
                ("token_type_hint", hint),
                ("client_id", &credentials.client_id),
            ])
            .send()
            .await;
        if !matches!(result, Ok(response) if response.status().is_success()) {
            failed = true;
        }
    }
    ensure!(!failed, failure());
    Ok(())
}
