use super::session_loader::load_session_for_tui;

#[tokio::test]
async fn loads_env_native_session_with_tail_window() {
    let Some(id) = std::env::var("CODETETHER_REAL_SESSION_ID").ok() else {
        return;
    };
    let start = std::time::Instant::now();
    let loaded = load_session_for_tui(&id).await.unwrap();
    let elapsed = start.elapsed();
    eprintln!(
        "loaded {} messages, dropped {}, file {} bytes in {:?}",
        loaded.session.messages.len(),
        loaded.dropped,
        loaded.file_bytes,
        elapsed
    );
    assert_eq!(loaded.session.id, id);
    assert!(loaded.dropped > 0);
}
