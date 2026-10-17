//! Original test_record_story_result test.
use super::*;

#[tokio::test]
async fn test_record_story_result() {
    let store = InMemoryStore::new();
    store.create_run(&test_run_state()).await.unwrap();

    let result = StoryResultEntry {
        story_id: "US-001".to_string(),
        title: "Test story".to_string(),
        passed: true,
        iteration: 1,
        error: None,
    };

    store
        .record_story_result("test-run-1", &result)
        .await
        .unwrap();

    let run = store.get_run("test-run-1").await.unwrap().unwrap();
    assert_eq!(run.story_results.len(), 1);
    assert!(run.story_results[0].passed);
}
