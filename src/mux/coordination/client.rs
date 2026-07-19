//! Fail-closed discovery and connection to the inherited mux authority.

use anyhow::{Context, Result};

pub(super) async fn connect() -> Result<Option<crate::mux::client::MuxConnection>> {
    let Some(name) = std::env::var_os(super::SESSION_ENV) else {
        return Ok(None);
    };
    let name = name
        .into_string()
        .map_err(|_| anyhow::anyhow!("mux session identity is not valid UTF-8"))?;
    anyhow::ensure!(!name.is_empty(), "mux session identity is empty");
    let record = crate::mux::registry::load(&name)
        .await
        .with_context(|| format!("load inherited mux session {name}"))?;
    crate::mux::client::MuxConnection::connect(&record)
        .await
        .with_context(|| format!("connect to inherited mux session {name}"))
        .map(Some)
}
