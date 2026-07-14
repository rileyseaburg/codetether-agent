//! Tests for spawn validation warning behavior.

use super::registry::set_registry_for_test;
use super::spawn_request::SpawnRequest;
use super::spawn_validation::validate_spawn_request;
use crate::provider::ProviderRegistry;
use std::sync::Arc;

fn request(model: &'static str) -> SpawnRequest<'static> {
    SpawnRequest {
        name: "warn_test_agent",
        instructions: "test",
        model,
        ephemeral: true,
        detach: false,
        parent_workspace: None,
        parent_session_id: None,
    }
}

#[tokio::test]
async fn ineligible_model_yields_warning_not_error() {
    set_registry_for_test(Arc::new(ProviderRegistry::new())).await;
    let outcome = validate_spawn_request(&request("anthropic/claude-opus-4"))
        .await
        .expect("ineligible model should not be a hard error");
    let warning = outcome.expect("expected a cost warning");
    assert!(warning.contains("not free/subscription-eligible"));
}

#[tokio::test]
async fn eligible_model_yields_no_warning() {
    set_registry_for_test(Arc::new(ProviderRegistry::new())).await;
    let outcome = validate_spawn_request(&request("openai-codex/gpt-5-mini"))
        .await
        .expect("eligible model should validate");
    assert!(outcome.is_none());
}
