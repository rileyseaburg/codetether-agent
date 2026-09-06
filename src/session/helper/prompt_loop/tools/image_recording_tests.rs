//! Mocked serial execution covers metadata through invoke, publish, and compaction.

use crate::provider::ProviderRegistry;
use crate::session::helper::{image_recording_fixture as fixture, prompt_loop};
use crate::tool::ToolRegistry;
use serde_json::json;
use std::sync::{Arc, atomic::Ordering};

#[tokio::test]
async fn serial_image_recording_preserves_success_and_failure_attachments() {
    let cwd = tempfile::tempdir().unwrap();
    let mut session = fixture::session(cwd.path()).await;
    let provider = Arc::new(fixture::RejectProvider::default());
    let mut providers = ProviderRegistry::new();
    providers.register(provider.clone());
    let tool = Arc::new(fixture::ImageTool::default());
    let mut registry = ToolRegistry::new();
    registry.register(tool.clone());
    let mut runner = prompt_loop::initialize(&mut session, None, Arc::new(providers))
        .await
        .unwrap();
    runner.model.registry = Arc::new(registry);
    for (id, success) in [("image-success", true), ("image-failure", false)] {
        let call = super::call::Call::new(
            id.into(),
            "read".into(),
            json!({"path": "mock-image", "success": success}),
        );
        super::call::run(&mut runner, 0, call).await.unwrap();
    }
    fixture::assert_recorded(runner.session);
    assert_eq!(tool.0.load(Ordering::SeqCst), 2);
    assert_eq!(provider.0.load(Ordering::SeqCst), 0);
}
