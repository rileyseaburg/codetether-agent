//! A process-isolated check that request building and identity share the live catalog.

#[test]
fn shared_catalog_controls_wire_model_and_reported_identity() {
    const CHILD: &str = "CODETETHER_CATALOG_CONTRACT_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "provider::live_catalog_contract_tests::shared_catalog_controls_wire_model_and_reported_identity", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"));
        return;
    }
    use crate::provider::{CompletionRequest, OpenAiCodexProvider, model_discovery};
    let models = model_discovery::parse_models(
        br#"{"models":[{
        "slug":"gpt-6-contract-probe", "visibility":"list",
        "supported_reasoning_levels":[{"effort":"high"}],
        "service_tiers":[{"id":"priority"}]
    }]}"#,
    )
    .unwrap();
    model_discovery::store(models);
    let model = "gpt-6-contract-probe-fast:high";
    assert_eq!(
        codetether_provider_core::model_identity::resolve("openai-codex", model),
        "gpt-6-contract-probe"
    );
    let request = CompletionRequest {
        model: model.into(),
        messages: Vec::new(),
        tools: Vec::new(),
        temperature: None,
        top_p: None,
        max_tokens: None,
        stop: Vec::new(),
    };
    let body = OpenAiCodexProvider::build_http_responses_body(&request);
    assert_eq!(body["model"], "gpt-6-contract-probe");
    assert_eq!(body["reasoning"]["effort"], "high");
    assert_eq!(body["service_tier"], "priority");
}
