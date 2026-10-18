//! Mocked parallel execution covers run_one, ordered recording, and compaction.

use crate::session::helper::image_recording_fixture as fixture;
use crate::tool::ToolRegistry;
use serde_json::json;
use std::sync::{Arc, atomic::Ordering};
use tokio::sync::mpsc;

#[tokio::test]
async fn parallel_image_recording_preserves_success_and_failure_attachments() {
    let cwd = tempfile::tempdir().unwrap();
    let mut session = fixture::session(cwd.path()).await;
    let provider = Arc::new(fixture::RejectProvider::default());
    let tool = Arc::new(fixture::ImageTool::default());
    let mut registry = ToolRegistry::new();
    registry.register(tool.clone());
    let calls = [("image-success", true), ("image-failure", false)].map(|(id, success)| {
        (
            id.into(),
            "read".into(),
            json!({"path": "mock-image", "success": success}),
        )
    });
    let (tx, _rx) = mpsc::channel(64);
    let mut no_match_count = 0;
    assert!(
        super::try_execute(
            &mut session,
            &calls,
            &registry,
            cwd.path(),
            "test",
            provider.clone(),
            &tx,
            &mut no_match_count,
        )
        .await
    );
    fixture::assert_recorded(&session);
    assert_eq!(tool.0.load(Ordering::SeqCst), 2);
    assert_eq!(provider.0.load(Ordering::SeqCst), 0);
}
