//! The calling session, not the newest workspace session, owns context tools.

use super::load_calling_session;
use crate::session::Session;
use serde_json::json;
use tempfile::TempDir;

#[tokio::test]
async fn loads_injected_session_even_when_a_newer_workspace_session_exists() {
    let temp = TempDir::new().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let workspace = temp.path().join("checkout");
    std::fs::create_dir_all(&workspace).unwrap();

    let mut older = Session::new().await.unwrap();
    older.metadata.directory = Some(workspace.clone());
    older.save().await.unwrap();

    let mut newer = Session::new().await.unwrap();
    newer.metadata.directory = Some(workspace);
    newer.save().await.unwrap();

    let loaded = load_calling_session(&json!({"__ct_session_id": older.id}))
        .await
        .unwrap()
        .expect("injected session exists");
    assert_eq!(loaded.id, older.id);
    assert_ne!(loaded.id, newer.id);
}

#[tokio::test]
async fn unknown_injected_session_is_reported_as_missing() {
    let temp = TempDir::new().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let loaded = load_calling_session(&json!({"__ct_session_id": "does-not-exist"}))
        .await
        .unwrap();
    assert!(loaded.is_none());
}
