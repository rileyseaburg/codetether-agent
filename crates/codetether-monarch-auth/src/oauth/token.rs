//! Convert a validated token response into a persistent credential record.
use super::{
    credentials::{Credentials, now},
    validation::validate_scope,
    wire::TokenResponse,
};
use anyhow::{Result, anyhow, ensure};

pub(super) fn credentials(
    response: TokenResponse,
    client_id: &str,
    scope: &str,
    previous_refresh: Option<&str>,
) -> Result<Credentials> {
    ensure!(
        response.token_type.eq_ignore_ascii_case("bearer") && response.expires_in > 0,
        "Unsupported Monarch token response"
    );
    let granted_scope = response.scope.as_deref().unwrap_or(scope);
    validate_scope(granted_scope, scope)?;
    let credentials = Credentials {
        client_id: client_id.into(),
        access_token: response.access_token,
        refresh_token: response
            .refresh_token
            .or_else(|| previous_refresh.map(str::to_owned)),
        expires_at: now()?
            .checked_add(response.expires_in)
            .ok_or_else(|| anyhow!("Invalid token expiration"))?,
        scope: granted_scope.into(),
    };
    credentials.validate()?;
    Ok(credentials)
}
