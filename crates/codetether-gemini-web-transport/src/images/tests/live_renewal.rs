//! Opt-in native renewal proof; never prints or exports cookie contents.
use crate::images::{ImageClient, cookies::now, rotation_fresh::usable};
use std::path::PathBuf;

#[tokio::test]
#[ignore = "requires GEMINI_WEB_COOKIE_FILE, GEMINI_RUST_EVIDENCE_DIR and live Google HTTP"]
async fn native_renewal_persists_and_reuses_cookies() {
    let file = std::env::var_os("GEMINI_WEB_COOKIE_FILE").expect("explicit cookie file");
    let directory = PathBuf::from(
        std::env::var_os("GEMINI_RUST_EVIDENCE_DIR").expect("explicit evidence directory"),
    );
    let export = std::fs::read_to_string(file).unwrap();
    let home = PathBuf::from(std::env::var_os("HOME").unwrap());
    let node = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".cache"))
        .join("codetether/gemini-web");
    let client = ImageClient::new(&export, directory.join("native-cache"))
        .unwrap()
        .with_node_cache(&node);
    let mut state = client.state.lock().await;
    let result: anyhow::Result<()> = async {
        assert!(!client.refresh(&mut state, true).await?);
        let saved = client.cache.load().expect("persisted private cache");
        assert!(usable(saved.updated, &saved.cookies, now()));
        // A new client must reuse the native cache without the Node migration.
        let next = ImageClient::new(&export, directory.join("native-cache"))?;
        let mut reloaded = next.state.lock().await;
        assert!(next.refresh(&mut reloaded, false).await?);
        next.page(&reloaded).await?;
        Ok(())
    }
    .await;
    let report = serde_json::json!({
        "level": "static/local native Rust transport against live Google HTTP",
        "success": result.is_ok(),
        "error": result.as_ref().err().map(ToString::to_string),
        "native_renewal_persisted_and_reused": result.is_ok(),
        "authenticated_bootstrap": result.is_ok(),
    });
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(
        directory.join("renewal.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
    println!("{report}");
    assert!(result.is_ok(), "native renewal failed; see renewal.json");
}
