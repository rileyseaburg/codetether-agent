//! Check application adapter identities against the shared normalization rules.

pub(super) fn resolve(provider: &str, model: &str) -> String {
    codetether_provider_core::model_identity::resolve(provider, model)
}

#[cfg(test)]
#[path = "routing_identity/tests.rs"]
mod tests;
