//! Fail-closed discovery and connection to the inherited mux authority.
//!
//! Leases guard the checkout, so the connection is server-scoped: every
//! session on the same workspace coordinates through one lease table.

use anyhow::{Context, Result};

pub(super) async fn connect() -> Result<Option<codetether_mux_client::MuxConnection>> {
    let Some(name) = std::env::var_os(super::SESSION_ENV) else {
        return Ok(None);
    };
    let name = name
        .into_string()
        .map_err(|_| anyhow::anyhow!("mux session identity is not valid UTF-8"))?;
    anyhow::ensure!(!name.is_empty(), "mux session identity is empty");
    let target = codetether_mux_registry::load(&name)
        .await
        .with_context(|| format!("load inherited mux session {name}"))?;
    codetether_mux_client::MuxConnection::connect_server(&target.record)
        .await
        .with_context(|| format!("connect to inherited mux session {name}"))
        .map(Some)
}
