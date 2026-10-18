//! Forward native runtime and journal access into shared review dispatch.
use super::{Application, Reviewer};
use crate::runtime::ReviewRuntime;
use codetether_application_providers::ProviderRegistry;
impl<H: Application> crate::dispatch::Host for Reviewer<H> {
    fn enabled() -> Option<ReviewRuntime<ProviderRegistry<H::Providers>>> {
        H::enabled()
    }
    async fn goal(session_id: Option<&str>) -> Option<String> {
        H::goal(session_id).await
    }
}
