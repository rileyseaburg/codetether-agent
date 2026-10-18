//! Process-wide approval reviewer state bound to native provider handles.
use crate::{config::ReviewConfig, provider::ProviderRegistry};
use codetether_approval_review::runtime::RuntimeStore;
use std::sync::Arc;
/// Installed native reviewer settings and providers.
pub type ReviewRuntime = codetether_approval_review::runtime::ReviewRuntime<ProviderRegistry>;
static RUNTIME: RuntimeStore<ProviderRegistry> = RuntimeStore::new();
/// Install reviewer settings and providers for this process.
pub fn install(config: ReviewConfig, registry: Arc<ProviderRegistry>) {
    RUNTIME.install(config, registry);
}
/// Read the runtime only when reviewing is enabled.
pub fn enabled() -> Option<ReviewRuntime> {
    RUNTIME.enabled()
}
/// Forget the installed runtime (tests).
pub fn reset() {
    RUNTIME.reset();
}
