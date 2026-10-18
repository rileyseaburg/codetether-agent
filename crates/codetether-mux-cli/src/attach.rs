//! Attach command implementation.

use anyhow::Result;

pub(super) async fn run(target: &str, host: &impl crate::Host) -> Result<()> {
    let target = codetether_mux_registry::load(target).await?;
    crate::attachment::run(&target, host).await
}
