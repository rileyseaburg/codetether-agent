//! Opt-in real rust-analyzer regression for clearing Cargo diagnostics on save.
use codetether_lsp_core::{LspClient, get_language_server_config, path_to_uri};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires installed rust-analyzer, Cargo, rust-src; isolated workspace"]
async fn saved_diagnostics_clear_real_rust_analyzer() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();
    std::fs::write(root.join("Cargo.toml"), "[workspace]\n[package]\nname=\"save-fixture\"\nversion=\"0.1.0\"\nedition=\"2024\"\n[lib]\npath=\"lib.rs\"\n").unwrap();
    let source = root.join("lib.rs");
    std::fs::write(&source, "pub fn broken( {\n").unwrap();
    let uri = path_to_uri(&source);
    let mut config = get_language_server_config("rust").unwrap();
    config.root_uri = Some(path_to_uri(&root));
    config.timeout_ms = 60_000;
    let client = LspClient::new(config).await.unwrap();
    client
        .initialize_with_identity("save-regression", |_| {})
        .await
        .unwrap();
    client.diagnostics(&source).await.unwrap();
    tokio::time::timeout(Duration::from_secs(60), async {
        loop {
            let snapshot = client.transport().diagnostics_snapshot().await;
            if snapshot
                .get(&uri)
                .is_some_and(|items| items.iter().any(|d| d.source.as_deref() == Some("rustc")))
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    })
    .await
    .expect("initial Cargo-check diagnostic");
    std::fs::write(&source, "pub fn answer() -> u32 { 42 }\n").unwrap();
    client.diagnostics(&source).await.unwrap();
    tokio::time::timeout(Duration::from_secs(60), async {
        loop {
            let snapshot = client.transport().diagnostics_snapshot().await;
            if snapshot.get(&uri).is_some_and(Vec::is_empty) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
    })
    .await
    .expect("saved correction clears Cargo diagnostics");
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        client
            .transport()
            .diagnostics_snapshot()
            .await
            .get(&uri)
            .unwrap()
            .is_empty()
    );
    client.shutdown().await.unwrap();
}
