use super::*;

#[tokio::test]
async fn test_matches_pattern() {
    let result = SharedResult {
        key: "test/key".to_string(),
        producer_id: "task-1".to_string(),
        value: Value::Null,
        schema: ResultSchema::from_value(&Value::Null),
        published_at: chrono::Utc::now(),
        expires_at: None,
        tags: vec!["tag-a".to_string()],
    };

    assert!(ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::Exact("test/key".to_string())
    ));
    assert!(!ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::Exact("other".to_string())
    ));
    assert!(ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::Prefix("test/".to_string())
    ));
    assert!(ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::Tag(vec!["tag-a".to_string()])
    ));
    assert!(ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::Producer("task-1".to_string())
    ));
    assert!(ResultStore::matches_pattern(
        &result,
        &SubscriptionPattern::All
    ));
}
