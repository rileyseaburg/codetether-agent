//! Coordinate public-client authorization over a loopback callback.
use super::{
    credentials::Credentials, exchange::exchange, registration::register, url::authorization_url,
};
use crate::callback;
use anyhow::{Result, anyhow};
use reqwest::Client;
use tokio::net::TcpListener;

pub(crate) async fn authorize(client: &Client, write: bool) -> Result<Credentials> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|_| anyhow!("Could not bind Monarch loopback callback"))?;
    let address = listener
        .local_addr()
        .map_err(|_| anyhow!("Loopback address unavailable"))?;
    let redirect = format!("http://{address}/callback");
    let scope = if write {
        "mcp:read mcp:write"
    } else {
        "mcp:read"
    };
    let client_id = register(client, &redirect, scope).await?;
    let verifier = callback::random_secret()?;
    let state = callback::random_secret()?;
    let url = authorization_url(&client_id, &redirect, scope, &state, &verifier)?;
    // CLI-only output: no token, authorization code or verifier is printed.
    eprintln!("Open this URL in your browser to connect Monarch:\n{url}");
    let code = callback::receive(listener, &state).await?;
    exchange(client, &client_id, &redirect, &code, &verifier, scope).await
}
