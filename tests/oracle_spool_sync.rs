use axum::{Router, http::StatusCode, routing::any};
use std::process::Command;

#[test]
fn oracle_spools_when_remote_unreachable() {
    let temp = tempfile::tempdir().expect("tempdir");
    let spool_dir = temp.path().join("spool");
    let src_path = temp.path().join("sample.rs");
    std::fs::write(&src_path, "pub async fn analyze() {}\n").expect("write source");
    let payload = r#"{"kind":"grep","file":"sample.rs","pattern":"async fn","matches":[{"line":1,"text":"pub async fn analyze() {}"}]}"#;

    let validate = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "oracle",
            "validate",
            "--query",
            "Find async fns",
            "--file",
            src_path.to_str().expect("path utf8"),
            "--payload",
            payload,
            "--json",
            "--persist",
        ])
        .env(
            "CODETETHER_ORACLE_SPOOL_DIR",
            spool_dir.to_str().expect("spool utf8"),
        )
        .env("MINIO_ENDPOINT", "127.0.0.1:9")
        .env("MINIO_ACCESS_KEY", "minio")
        .env("MINIO_SECRET_KEY", "minio123")
        .env("CODETETHER_BUS_S3_BUCKET", "oracle-test")
        .env("CODETETHER_BUS_S3_PREFIX", "training/")
        .output()
        .expect("run oracle validate");

    assert!(
        validate.status.success(),
        "validate stderr: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
    let validate_json = parse_json_from_output(&validate.stdout).expect("validate json");
    assert_eq!(validate_json["persist"]["uploaded"], false);
    assert!(
        validate_json["persist"]["pending_count"]
            .as_u64()
            .unwrap_or(0)
            >= 1
    );

    let sync = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args(["oracle", "sync", "--json"])
        .env(
            "CODETETHER_ORACLE_SPOOL_DIR",
            spool_dir.to_str().expect("spool utf8"),
        )
        .env("MINIO_ENDPOINT", "127.0.0.1:9")
        .env("MINIO_ACCESS_KEY", "minio")
        .env("MINIO_SECRET_KEY", "minio123")
        .env("CODETETHER_BUS_S3_BUCKET", "oracle-test")
        .env("CODETETHER_BUS_S3_PREFIX", "training/")
        .output()
        .expect("run oracle sync");

    assert!(
        sync.status.success(),
        "sync stderr: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let sync_json = parse_json_from_output(&sync.stdout).expect("sync json");
    assert!(sync_json["retained"].as_u64().unwrap_or(0) >= 1);
    assert!(sync_json["pending_after"].as_u64().unwrap_or(0) >= 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn oracle_uploads_when_remote_reachable() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let addr = listener.local_addr().expect("local addr");
    let app = Router::new().fallback(any(|| async { StatusCode::OK }));
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .await;
    });

    let temp = tempfile::tempdir().expect("tempdir");
    let spool_dir = temp.path().join("spool");
    let src_path = temp.path().join("sample.rs");
    std::fs::write(&src_path, "pub async fn analyze() {}\n").expect("write source");
    let payload = r#"{"kind":"grep","file":"sample.rs","pattern":"async fn","matches":[{"line":1,"text":"pub async fn analyze() {}"}]}"#;

    let validate = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args([
            "oracle",
            "validate",
            "--query",
            "Find async fns",
            "--file",
            src_path.to_str().expect("path utf8"),
            "--payload",
            payload,
            "--json",
            "--persist",
        ])
        .env(
            "CODETETHER_ORACLE_SPOOL_DIR",
            spool_dir.to_str().expect("spool utf8"),
        )
        .env("MINIO_ENDPOINT", addr.to_string())
        .env("MINIO_ACCESS_KEY", "minio")
        .env("MINIO_SECRET_KEY", "minio123")
        .env("CODETETHER_BUS_S3_BUCKET", "oracle-test")
        .env("CODETETHER_BUS_S3_PREFIX", "training/")
        .output()
        .expect("run oracle validate");

    assert!(
        validate.status.success(),
        "validate stderr: {}",
        String::from_utf8_lossy(&validate.stderr)
    );
    let validate_json = parse_json_from_output(&validate.stdout).expect("validate json");
    assert_eq!(
        validate_json["persist"]["uploaded"],
        true,
        "validate stdout: {}\nvalidate stderr: {}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr)
    );
    assert_eq!(validate_json["persist"]["pending_count"], 0);

    let sync = Command::new(env!("CARGO_BIN_EXE_codetether"))
        .args(["oracle", "sync", "--json"])
        .env(
            "CODETETHER_ORACLE_SPOOL_DIR",
            spool_dir.to_str().expect("spool utf8"),
        )
        .env("MINIO_ENDPOINT", addr.to_string())
        .env("MINIO_ACCESS_KEY", "minio")
        .env("MINIO_SECRET_KEY", "minio123")
        .env("CODETETHER_BUS_S3_BUCKET", "oracle-test")
        .env("CODETETHER_BUS_S3_PREFIX", "training/")
        .output()
        .expect("run oracle sync");

    assert!(
        sync.status.success(),
        "sync stderr: {}",
        String::from_utf8_lossy(&sync.stderr)
    );
    let sync_json = parse_json_from_output(&sync.stdout).expect("sync json");
    assert_eq!(sync_json["pending_after"], 0);

    let _ = shutdown_tx.send(());
    let _ = server.await;
}

fn parse_json_from_output(stdout: &[u8]) -> Result<serde_json::Value, serde_json::Error> {
    let text = String::from_utf8_lossy(stdout);
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
        return Ok(v);
    }
    if let Some(start) = text.find('{') {
        return serde_json::from_str(&text[start..]);
    }
    serde_json::from_str(&text)
}
