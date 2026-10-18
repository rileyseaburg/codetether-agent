//! Configuration, native tool setup and cognition autostart.
use crate::Host;
use codetether_cognition_api::http::Lifecycle;
use codetether_config::Config;
use codetether_plugin_verification::PluginRegistry;
use codetether_util::environment::env_bool;
use std::{sync::Arc, time::Instant};

pub(crate) async fn prepare<H: Host>(
    t0: Instant,
) -> anyhow::Result<(Config, Arc<H::Runtime>, PluginRegistry)> {
    let config = Config::load().await?;
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] config loaded"
    );
    let mut cognition = H::new_runtime();
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] cognition runtime created"
    );
    let plugin_registry = H::install_tools(&mut cognition);
    tracing::info!(
        elapsed_ms = t0.elapsed().as_millis(),
        "[startup] tools registered"
    );
    let cognition = Arc::new(cognition);
    Ok((config, cognition, plugin_registry))
}

pub(crate) async fn autostart<H: Host>(cognition: &H::Runtime, t0: Instant) {
    if H::runtime_enabled(cognition) && env_bool("CODETETHER_COGNITION_AUTO_START", true) {
        tracing::info!(
            elapsed_ms = t0.elapsed().as_millis(),
            "[startup] auto-starting cognition"
        );
        if let Err(error) = cognition.start(None).await {
            tracing::warn!(%error, "Failed to auto-start cognition loop");
        } else {
            tracing::info!("Perpetual cognition auto-started");
        }
    }
}
