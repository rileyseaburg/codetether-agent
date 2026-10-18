//! Canonical routing identity and session forwarding at every dispatch boundary.
use super::{Arc, MetricsProvider, Mutex, Provider, mock, request, text};

#[tokio::test]
async fn harness_identity_reaches_all_four_completion_paths() {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let wrapped = MetricsProvider::wrap(Arc::new(mock::Capture(captured.clone())));
    assert!(wrapped.complete(request("model-a")).await.is_err());
    assert!(
        wrapped
            .complete_scoped(request("model-b"), "session-a")
            .await
            .is_err()
    );
    assert!(wrapped.complete_stream(request("model-c")).await.is_ok());
    assert!(
        wrapped
            .complete_stream_scoped(request("model-d"), "session-b")
            .await
            .is_ok()
    );
    let calls = captured.lock().unwrap();
    assert_eq!(calls.len(), 4);
    for ((call, session), model) in calls
        .iter()
        .zip(["model-a", "model-b", "model-c", "model-d"])
    {
        assert!(text(&call.messages[0]).contains(r#""provider":"local_cuda""#));
        assert!(text(&call.messages[0]).contains(&format!("\"model\":\"{model}\"")));
        assert_eq!(call.model, model);
        assert_eq!(call.messages.len(), 3);
        assert_eq!(text(&call.messages[1]), "Preserve instructions");
        assert_eq!(text(&call.messages[2]), "Who are you?");
        assert_eq!(
            session.as_deref(),
            match model {
                "model-b" => Some("session-a"),
                "model-d" => Some("session-b"),
                _ => None,
            }
        );
    }
}
