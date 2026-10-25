//! Explicit credentialed protocol comparison; no rotation or generation.
use crate::images::ImageClient;
use std::{path::PathBuf, time::Duration};

#[tokio::test]
#[ignore = "requires GEMINI_WEB_COOKIE_FILE and live Google HTTP"]
async fn compare_bootstrap_protocols() {
    let file = std::env::var_os("GEMINI_WEB_COOKIE_FILE").expect("explicit cookie file");
    let export = std::fs::read_to_string(file).unwrap();
    let home = PathBuf::from(std::env::var_os("HOME").unwrap());
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".cache"))
        .join("codetether/gemini-web");
    for http1 in [false, true] {
        let temporary = tempfile::tempdir().unwrap();
        let mut client = ImageClient::new(&export, temporary.path().join("native"))
            .unwrap()
            .with_node_cache(&cache);
        let builder = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(crate::images::USER_AGENT)
            .timeout(Duration::from_secs(30));
        client.http = if http1 { builder.http1_only() } else { builder }
            .build()
            .unwrap();
        let state = client.state.lock().await;
        match client.page(&state).await {
            Ok(_) => println!("http1_only={http1}: authenticated"),
            Err(error) => println!("http1_only={http1}: {error}"),
        }
    }
}

#[tokio::test]
async fn network_error_does_not_expose_url() {
    let error = reqwest::Client::new()
        .get("http://[invalid]/secret")
        .send()
        .await
        .unwrap_err();
    let safe = crate::images::network_error::classify(&error);
    assert_eq!(safe, "request_builder:");
}
