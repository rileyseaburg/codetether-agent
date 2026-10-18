//! Interpret an authoritative workspace binding, failing closed on errors.
use super::ScopeError;
use reqwest::Response;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct WorkspaceBinding {
    id: String,
    path: PathBuf,
}

pub(super) async fn binding(response: Response, id: &str) -> anyhow::Result<PathBuf> {
    match response.status().as_u16() {
        200 => (),
        401 => return Err(ScopeError::Unauthenticated.into()),
        403 | 404 => return Err(ScopeError::Forbidden.into()),
        _ => return Err(ScopeError::Unavailable.into()),
    }
    let binding = response
        .json::<WorkspaceBinding>()
        .await
        .map_err(|_| ScopeError::Unavailable)?;
    if binding.id != id || !binding.path.is_absolute() {
        return Err(ScopeError::Forbidden.into());
    }
    Ok(binding.path)
}
