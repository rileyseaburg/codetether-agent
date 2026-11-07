//! Explicit Groq configuration is registered through the normal config path.
use crate::Registry;
use codetether_provider_core::Provider;
use std::sync::Arc;

#[derive(Default)]
struct Providers(Vec<Arc<dyn Provider>>);
impl Registry for Providers {
    fn contains(&self, name: &str) -> bool {
        self.0.iter().any(|p| p.name() == name)
    }
    fn register(&mut self, provider: Arc<dyn Provider>) {
        self.0.push(provider);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn groq_config_registers_without_a_custom_url() {
    let mut config = codetether_config::Config::default();
    config.providers.clear();
    config.providers.insert(
        "groq".into(),
        codetether_config::ProviderConfig {
            api_key: Some("fixture-key".into()),
            ..Default::default()
        },
    );
    let mut registry = Providers::default();
    super::compatible::register(&mut registry, &config).unwrap();
    assert!(registry.contains("groq"));
    assert_eq!(registry.len(), 1);
    config.providers.get_mut("groq").unwrap().api_key = Some(" ".into());
    assert!(super::compatible::register(&mut Providers::default(), &config).is_err());
}
