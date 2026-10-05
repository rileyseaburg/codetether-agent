//! Empty discovery is not authenticated availability; JSON remains data-only.
use super::{
    super::{
        render,
        types::{ModelCapability, ProviderCapability},
    },
    empty_models,
};

fn configured() -> ProviderCapability {
    ProviderCapability {
        provider: "bedrock".into(),
        available: true,
        source: "configured provider registry",
        error: None,
        models: vec![],
    }
}

#[test]
fn empty_discovery_has_workflow_without_changing_json() {
    let providers = [configured()];
    let hint = empty_models(&providers).unwrap();
    assert!(hint.contains("codetether vault login token"));
    assert!(hint.contains("codetether vault status"));
    assert!(hint.contains("local AWS credentials"));
    assert!(render::text(&providers).contains("bedrock [configured; no models discovered]"));
    let json = render::json(&providers).unwrap();
    assert!(
        serde_json::from_str::<serde_json::Value>(&json)
            .unwrap()
            .is_array()
    );
    assert!(!json.contains("Vault/provider setup"));
    assert!(empty_models(&[]).is_some());
}

#[test]
fn discovered_models_do_not_emit_recovery_hints() {
    let mut provider = configured();
    provider.models.push(ModelCapability {
        provider: "bedrock".into(),
        canonical_id: "model".into(),
        selectable_id: "bedrock/model".into(),
        aliases: vec![],
        qualifiers: vec![],
        available: true,
        source: "provider.list_models",
    });
    assert!(empty_models(&[provider]).is_none());
}
