#![cfg(feature = "tetherscript")]

//! Regression test: Cerebras is routed through the OpenAI-compatible provider
//! so it supports tool-calling and structured streaming (required for GLM-4.7
//! agentic use). Previously it used a text-only adapter that dropped tools and
//! could not stream.

use codetether_agent::provider::Provider;

#[tokio::test]
async fn cerebras_provider_supports_tools_and_streaming() {
    let provider =
        codetether_agent::provider::tetherscript_provider::cerebras::new("test-key", None)
            .expect("cerebras provider should initialize");

    assert_eq!(provider.name(), "cerebras");
    assert!(
        provider.supports_structured_streaming(),
        "cerebras must support structured streaming for tool calls"
    );

    let models = provider
        .list_models()
        .await
        .expect("cerebras list_models should return defaults");

    assert!(!models.is_empty());
    for model in models {
        assert_eq!(model.provider, "cerebras");
        assert!(model.supports_tools);
        assert!(model.supports_streaming);
    }
}
