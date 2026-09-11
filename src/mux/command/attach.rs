//! Attach command implementation.

use anyhow::Result;

pub(super) async fn run(target: &str) -> Result<()> {
    let target = crate::mux::registry::load(target).await?;
    crate::mux::client::attach(&target).await
}
