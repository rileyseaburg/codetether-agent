//! Check the actual Gemini Web provider hook without authentication or transport.
use crate::provider::{Provider, gemini_web::GeminiWebProvider, metrics::MetricsProvider};
use std::sync::Arc;

#[test]
fn gemini_web_fallback_identity_matches_its_transport_default() {
    let provider = GeminiWebProvider::new(String::new()).unwrap();
    assert_eq!(provider.name(), "gemini-web");
    for model in ["", "unknown", "gemini-web-fast"] {
        assert_eq!(provider.resolved_model_identity(model), "gemini-web-fast");
    }
    for model in [
        "gemini-web-thinking",
        "gemini-web-pro",
        "gemini-web-deep-think",
    ] {
        assert_eq!(provider.resolved_model_identity(model), model);
    }
}

#[test]
fn gemini_web_metrics_wrapper_preserves_fallback_and_model_switches() {
    let provider = MetricsProvider::wrap(Arc::new(GeminiWebProvider::new(String::new()).unwrap()));
    for (requested, resolved) in [
        ("unknown", "gemini-web-fast"),
        ("gemini-web-pro", "gemini-web-pro"),
        ("gemini-web-thinking", "gemini-web-thinking"),
        ("", "gemini-web-fast"),
    ] {
        let prompt = crate::identity::prompt(
            provider.name(),
            &provider.resolved_model_identity(requested),
        );
        assert!(prompt.contains(&format!("\"model\":\"{resolved}\"")));
    }
}
