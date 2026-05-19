use super::*;
use crate::swarm::{SubTask, SwarmConfig};

#[test]
fn test_validate_configuration() {
    let mut config = SwarmConfig::default();
    config.max_subagents = 0;
    config.subagent_timeout_secs = 0;
    let validator = SwarmValidator::new(config, "test".to_string(), "model".to_string());
    let mut issues = Vec::new();
    validator.validate_configuration(&mut issues);
    assert_eq!(issues.len(), 2);
    assert!(issues.iter().any(|i| i.message.contains("max_subagents")));
    assert!(
        issues
            .iter()
            .any(|i| i.message.contains("subagent_timeout_secs"))
    );
}

#[test]
fn test_token_estimate() {
    let validator = SwarmValidator::new(SwarmConfig::default(), "test".into(), "model".into());
    let provider_status = ProviderStatus {
        provider: "test".to_string(),
        model: "model".to_string(),
        is_available: true,
        has_credentials: true,
        context_window: 1000,
    };
    let subtasks: Vec<SubTask> = (0..10)
        .map(|i| SubTask::new(&format!("Task {i}"), &"x".repeat(1000)))
        .collect();
    let mut issues = Vec::new();
    let estimate = validator.estimate_token_usage(&subtasks, &provider_status, &mut issues);
    assert!(estimate.total_tokens > 0);
    assert!(estimate.prompt_tokens > 0);
    assert!(estimate.completion_tokens > 0);
}
