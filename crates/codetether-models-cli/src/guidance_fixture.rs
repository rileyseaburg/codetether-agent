//! Empty-provider fixture for recovery-guidance tests.
use super::ProviderCapability;

pub(super) fn configured() -> ProviderCapability {
    ProviderCapability {
        provider: "bedrock".into(),
        available: true,
        source: "configured provider registry",
        error: None,
        models: vec![],
    }
}
