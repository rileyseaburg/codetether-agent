//! Construct a caller-credentialed request to the configured authority.
use super::ScopeError;
use axum::http::{HeaderMap, header::AUTHORIZATION};
use reqwest::{Client, RequestBuilder, Url, redirect::Policy};
use std::time::Duration;

pub(super) fn build(server: &str, id: &str, headers: &HeaderMap) -> anyhow::Result<RequestBuilder> {
    let token = headers
        .get(AUTHORIZATION)
        .ok_or(ScopeError::Unauthenticated)?;
    let value = token.to_str().map_err(|_| ScopeError::Unauthenticated)?;
    if !value.starts_with("Bearer ") || value[7..].trim().is_empty() {
        return Err(ScopeError::Unauthenticated.into());
    }
    if id.is_empty() || id == "." || id == ".." {
        return Err(ScopeError::Forbidden.into());
    }
    let mut url = Url::parse(server).map_err(|_| ScopeError::Unavailable)?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(ScopeError::Unavailable.into());
    }
    url.set_query(None);
    url.set_fragment(None);
    url.path_segments_mut()
        .map_err(|_| ScopeError::Unavailable)?
        .pop_if_empty()
        .extend(["v1", "agent", "workspaces", id, "session-access"]);
    let client = Client::builder()
        .redirect(Policy::none())
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|_| ScopeError::Unavailable)?;
    Ok(client.post(url).header(AUTHORIZATION, token.clone()))
}
