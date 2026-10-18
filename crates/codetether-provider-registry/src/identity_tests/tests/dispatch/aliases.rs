//! Resolved upstream identity must reach every dispatch mode, not the requested alias.
use super::{MetricsProvider, Provider, mock, request, text};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn harness_identity_reports_resolved_model_in_all_four_completion_paths() {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let wrapped = MetricsProvider::wrap(Arc::new(mock::Capture(captured.clone())));
    assert_eq!(wrapped.resolved_model_identity("alias:model-a"), "model-a");
    assert!(wrapped.complete(request("alias:model-a")).await.is_err());
    assert!(
        wrapped
            .complete_scoped(request("alias:model-b"), "session-a")
            .await
            .is_err()
    );
    assert!(
        wrapped
            .complete_stream(request("alias:model-c"))
            .await
            .is_ok()
    );
    assert!(
        wrapped
            .complete_stream_scoped(request("alias:model-d"), "session-b")
            .await
            .is_ok()
    );
    let calls = captured.lock().unwrap();
    assert_eq!(calls.len(), 4);
    for ((call, _), resolved) in calls
        .iter()
        .zip(["model-a", "model-b", "model-c", "model-d"])
    {
        let identity = text(&call.messages[0]);
        assert!(identity.contains(r#""provider":"local_cuda""#));
        assert!(identity.contains(&format!("\"model\":\"{resolved}\"")));
        assert!(!identity.contains("alias:"));
        assert_eq!(call.model, format!("alias:{resolved}"));
    }
}
