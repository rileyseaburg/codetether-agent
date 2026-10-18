//! Begin device authorization without a confidential client secret.

pub(super) use super::grant::Grant;
use anyhow::{Result, ensure};

pub(super) async fn request(
    metadata: &super::discovery::Metadata,
    client_id: &str,
    no_browser: bool,
) -> Result<Grant> {
    let grant: Grant = codetether_secrets::login::http::json(
        codetether_secrets::login::http::client()?
            .post(&metadata.device_authorization_endpoint)
            .form(&[("client_id", client_id), ("scope", "openid profile")]),
    )
    .await?;
    let url = reqwest::Url::parse(&grant.verification_uri)
        .map_err(|_| anyhow::anyhow!("Invalid device verification URI"))?;
    ensure!(
        url.origin() == reqwest::Url::parse(&metadata.issuer)?.origin()
            && url.username().is_empty()
            && url.password().is_none(),
        "Unexpected device verification origin"
    );
    ensure!(
        !grant.device_code.is_empty() && !grant.user_code.is_empty() && grant.expires_in > 0,
        "Incomplete device authorization response"
    );
    eprintln!(
        "Open {} and enter code {}",
        grant.verification_uri, grant.user_code
    );
    if !no_browser {
        let _ = open::that(&grant.verification_uri);
    }
    Ok(grant)
}
