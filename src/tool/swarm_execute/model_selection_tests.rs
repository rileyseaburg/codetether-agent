use super::{model_request, model_selection::resolve};
use serde_json::json;

#[test]
fn preserves_luna_fast_high_model_reference() {
    let providers = vec!["openai-codex"];
    let selected = resolve(Some("openai-codex/gpt-5.6-luna-fast:high"), &providers)
        .expect("explicit provider should resolve");

    assert_eq!(
        selected.requested_model.as_deref(),
        Some("openai-codex/gpt-5.6-luna-fast:high")
    );
    assert_eq!(selected.resolved_provider, "openai-codex");
    assert_eq!(selected.resolved_model, "gpt-5.6-luna-fast:high");
}

#[test]
fn routes_tool_json_into_serialized_model_diagnostics() {
    let params = json!({ "model": "openai-codex/gpt-5.6-luna-fast:high" });
    let selected = resolve(model_request::requested(&params), &["openai-codex"])
        .expect("tool model should resolve");
    let diagnostics = serde_json::to_value(selected).expect("selection should serialize");

    assert_eq!(diagnostics["requested_model"], params["model"]);
    assert_eq!(diagnostics["resolved_provider"], "openai-codex");
    assert_eq!(diagnostics["resolved_model"], "gpt-5.6-luna-fast:high");
}

#[test]
fn inherits_the_parent_model_when_model_is_omitted() {
    let params = json!({ "__ct_current_model": "openai-codex/gpt-5.6-luna-fast:high" });
    assert_eq!(
        model_request::requested(&params),
        Some("openai-codex/gpt-5.6-luna-fast:high")
    );
}

#[test]
fn rejects_an_unavailable_explicit_provider() {
    let error = resolve(Some("openai-codex/gpt-5.6-luna-fast:high"), &["zai".into()])
        .expect_err("missing explicit provider should fail");

    assert!(error.to_string().contains("openai-codex"));
    assert!(error.to_string().contains("zai"));
}

#[path = "provider_default_tests.rs"]
mod provider_default_tests;
