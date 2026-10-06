use super::fixtures::{render, update};

#[tokio::test]
async fn lagged_receiver_reports_gap_without_partial_output() {
    let body = render(vec![
        update("task.abc", "abc"),
        update("task.abc-2", "abc-2"),
        update("task.abc", "abc"),
    ], 1).await;
    assert_eq!(body.matches("event: lag").count(), 1);
    assert_eq!(body.matches("event: output").count(), 0);
    let lag = body.lines().find_map(|line| line.strip_prefix("data: ")).unwrap();
    let lag: serde_json::Value = serde_json::from_str(lag).unwrap();
    assert_eq!(lag, serde_json::json!({
        "task_id": "abc",
        "error": "output_gap",
        "skipped_bus_events": 2,
        "replay_available": false,
    }));
    assert!(!body.contains("abc-2"));
}

#[tokio::test]
async fn closed_empty_bus_finishes_without_events() {
    assert!(render(vec![], 1).await.is_empty());
}