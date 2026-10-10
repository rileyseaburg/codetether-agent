//! Loopback companion relay entry point (`127.0.0.1:4099` by default).
//!
//! Environment: `CODETETHER_AUTH_TOKEN` (owner credential, required),
//! `CODETETHER_COMPANION_UPSTREAM` (default `https://server.codetether.run`),
//! `CODETETHER_COMPANION_ORIGIN`, `CODETETHER_COMPANION_ADDR`, and
//! `CODETETHER_COMPANION_ASSETS` (optional web shell directory).
use codetether_companion_relay::{
    Relay, router, shutdown, spawn_sweeper, terminate, vision_analyzer,
};

const HOSTED: &str = "https://server.codetether.run";

fn env(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().json().init();
    let token = std::env::var("CODETETHER_AUTH_TOKEN").unwrap_or_default();
    let analyze = vision_analyzer(&token, &env("CODETETHER_COMPANION_UPSTREAM", HOSTED))?;
    let assets = std::env::var_os("CODETETHER_COMPANION_ASSETS").map(Into::into);
    let origin = env("CODETETHER_COMPANION_ORIGIN", HOSTED);
    let relay = Relay::new(&token, analyze, &origin, assets)
        .map_err(|_| anyhow::anyhow!("A persistent API credential is required"))?;
    let sweeper = spawn_sweeper(relay.clone());
    let addr = env("CODETETHER_COMPANION_ADDR", "127.0.0.1:4099");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(event = "screen_relay_listening", address = %addr, "Relay listening");
    let stopping = relay.clone();
    axum::serve(listener, router(relay))
        .with_graceful_shutdown(async move {
            terminate().await;
            shutdown(&stopping);
        })
        .await?;
    sweeper.abort();
    Ok(())
}
