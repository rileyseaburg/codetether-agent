//! Device authorization via the IdP, followed by Vault JWT authentication.
//!
//! Vault itself has no OAuth device endpoint: the issuer needs a public device
//! client, and Vault needs a JWT-type role for its issuer/audience and group.

mod discovery;
mod poll;
mod start;
mod token_reply;
mod token_request;

use anyhow::{Result, ensure};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginResponse {
    auth: Auth,
}
#[derive(Deserialize)]
struct Auth {
    client_token: String,
}

pub(super) async fn login(address: &str, args: &super::device_args::DeviceArgs) -> Result<String> {
    let path = crate::secrets::login::http::auth_path(&args.mount, &args.role)?;
    ensure!(
        !args.client_id.trim().is_empty(),
        "A public device client ID is required"
    );
    let metadata = discovery::load(&args.issuer).await?;
    let grant = start::request(&metadata, &args.client_id, args.no_browser).await?;
    let jwt = poll::wait(&metadata, &grant, &args.client_id).await?;
    let response: LoginResponse = crate::secrets::login::http::post(
        address,
        &path,
        &serde_json::json!({"role": args.role, "jwt": jwt}),
    )
    .await?;
    ensure!(
        !response.auth.client_token.is_empty(),
        "Vault JWT exchange omitted a token"
    );
    Ok(response.auth.client_token)
}
