//! Validate authentication and reject administrator capability before persistence.

use anyhow::{Result, ensure};

use super::facts::{Envelope, Facts};
pub(crate) async fn token(address: &str, token: &str) -> Result<Facts> {
    ensure!(!token.trim().is_empty(), "Vault token is empty");
    let client = super::http::client()?;
    let result: Envelope<Facts> = super::http::json(
        client
            .get(format!("{address}/v1/auth/token/lookup-self"))
            .header("X-Vault-Token", token),
    )
    .await?;
    let facts = result.data;
    ensure!(
        !facts
            .policies
            .iter()
            .any(|p| ["root", "admin", "superadmin"].contains(&p.to_ascii_lowercase().as_str())),
        "Administrator tokens cannot be saved for CodeTether"
    );
    super::capabilities::check(&client, address, token).await?;
    Ok(facts)
}
