use super::fixtures::{artifact, render, update};

#[tokio::test]
async fn sse_emits_only_exact_task_updates_and_artifacts_in_order() {
    let body = render(
        vec![
            update("task.abc-2", "abc-2"),
            update("task.abc", "other"),
            update("task.abc", "abc"),
            artifact("task.abc", "abc"),
            artifact("task.abc.output", "abc"),
            artifact("task.abc", "abc-2"),
        ],
        16,
    )
    .await;
    assert_eq!(body.matches("event: output").count(), 2);
    let payloads: Vec<serde_json::Value> = body
        .lines()
        .filter_map(|line| line.strip_prefix("data: "))
        .map(|data| serde_json::from_str(data).unwrap())
        .collect();
    assert_eq!(payloads.len(), 2);
    assert!(payloads.iter().all(|p| p["task_id"] == "abc"));
    assert_eq!(payloads[0]["kind"], "task_update");
    assert_eq!(payloads[0]["state"], "working");
    assert_eq!(payloads[0]["message"], "chunk");
    assert_eq!(payloads[1]["kind"], "artifact_update");
    assert_eq!(payloads[1]["artifact"]["artifactId"], "artifact-1");
}
