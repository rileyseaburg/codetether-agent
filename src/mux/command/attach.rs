//! Attach command implementation.

use anyhow::{Context, Result};

pub(super) async fn run(target: &str) -> Result<()> {
    crate::mux::registry::validate_name(target)?;
    let record = crate::mux::registry::load(target)
        .await
        .with_context(|| format!("mux session '{target}' was not found"))?;
    crate::mux::client::attach(&record).await
}
