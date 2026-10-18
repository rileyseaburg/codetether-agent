//! Opt-in read-only proof against a real repository, not a tiny language fixture.

use crate::{
    config::Config,
    lsp::{LspManager, path_to_uri},
};
use std::{path::PathBuf, sync::Arc};
use tokio::time::{Duration, Instant};

#[tokio::test]
#[ignore = "requires CODETETHER_LSP_PROBE_WORKSPACE and CODETETHER_LSP_PROBE_FILE"]
async fn real_project_preflight_retains_cold_server_and_returns_warm_diagnostics() {
    let _ = tracing_subscriber::fmt().with_test_writer().try_init();
    let root = PathBuf::from(std::env::var("CODETETHER_LSP_PROBE_WORKSPACE").unwrap());
    let path = root.join(std::env::var("CODETETHER_LSP_PROBE_FILE").unwrap());
    let content = std::fs::read_to_string(&path).unwrap();
    let config = Config::load_for_workspace(&root).await.unwrap();
    let manager = Arc::new(LspManager::with_config(
        Some(path_to_uri(&root)),
        config.lsp,
    ));
    let files = vec![(path, content)];
    let started = Instant::now();
    let (_, cold_warnings) =
        super::scan::run(&root, "apply_patch", files.clone(), manager.clone()).await;
    let cold_ms = started.elapsed().as_millis();
    let client = manager.get_client("typescript").await.unwrap();
    super::test_wait::ready(&root, Duration::from_secs(60)).await;
    let warmed = manager.get_client("typescript").await.unwrap();
    assert!(Arc::ptr_eq(&client, &warmed), "cold server was evicted");
    let started = Instant::now();
    let (blocked, warnings) = super::scan::run(&root, "apply_patch", files, manager.clone()).await;
    let warm_ms = started.elapsed().as_millis();
    tracing::info!(
        cold_ms,
        warm_ms,
        cold_unavailable = !cold_warnings.is_empty(),
        warm_unavailable = !warnings.is_empty(),
        reported_errors = blocked.is_some(),
        "Real project preflight timings"
    );
    assert!(
        warnings.is_empty(),
        "warm diagnostics unavailable: {warnings:?}"
    );
    assert!(
        warm_ms < 5000,
        "warm checks still consume the interactive deadline"
    );
    manager.shutdown_all().await;
}
