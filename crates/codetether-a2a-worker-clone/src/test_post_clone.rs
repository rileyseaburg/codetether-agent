use serde_json::json;

#[test]
fn worker_allows_github_app_post_clone_followups() {
    let metadata = json!({"source": "github-app", "post_clone_task": {"title": "Work issue #76", "prompt": "fix it"}}).as_object().cloned().unwrap();
    assert!(super::post_clone_task::worker_should_enqueue_post_clone_task(&metadata));
}

#[test]
fn worker_allows_legacy_post_clone_followups() {
    let metadata =
        json!({"post_clone_task": {"title": "Continue after clone", "prompt": "run build"}})
            .as_object()
            .cloned()
            .unwrap();
    assert!(super::post_clone_task::worker_should_enqueue_post_clone_task(&metadata));
}
