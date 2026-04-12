//! Control-plane requests for Git credential material.
//!
//! The worker asks the A2A server for short-lived Git credentials on demand so
//! repository operations do not depend on long-lived local secrets.
//!
//! # Examples
//!
//! ```ignore
//! let creds = request_git_credentials(client, server, &token, None, "ws-1", "get", &query).await?;
//! ```

use anyhow::{Context, Result, anyhow};
use reqwest::{Client, StatusCode};
use serde::Serialize;

use super::{GitCredentialMaterial, GitCredentialQuery};

#[derive(Debug, Serialize)]
struct GitCredentialRequestBody<'a> {
    operation: &'a str,
    protocol: Option<&'a str>,
    host: Option<&'a str>,
    path: Option<&'a str>,
}

/// Requests short-lived Git credentials from the A2A control plane.
///
/// Missing credentials return `Ok(None)` when the server responds with `404`.
///
/// # Examples
///
/// ```ignore
/// let creds = request_git_credentials(client, server, &token, None, "ws-1", "get", &query).await?;
/// ```
pub async fn request_git_credentials(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: Option<&str>,
    workspace_id: &str,
    operation: &str,
    query: &GitCredentialQuery,
) -> Result<Option<GitCredentialMaterial>> {
    let mut request = client.post(format!(
        "{}/v1/agent/workspaces/{workspace_id}/git/credentials",
        server.trim_end_matches('/')
    ));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    if let Some(worker_id) = worker_id.filter(|value| !value.trim().is_empty()) {
        request = request.header("X-Worker-ID", worker_id);
    }
    let response = request
        .json(&GitCredentialRequestBody {
            operation,
            protocol: query.protocol.as_deref(),
            host: query.host.as_deref(),
            path: query.path.as_deref(),
        })
        .send()
        .await
        .context("Failed to request Git credentials from server")?;
    match response.status() {
        StatusCode::OK => Ok(Some(
            response
                .json::<GitCredentialMaterial>()
                .await
                .context("Failed to decode Git credential response")?,
        )),
        StatusCode::NOT_FOUND => Ok(None),
        status => Err(anyhow!(
            "Git credential request failed with {}: {}",
            status,
            response.text().await.unwrap_or_default()
        )),
    }
}
