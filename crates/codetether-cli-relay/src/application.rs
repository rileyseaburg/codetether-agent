//! Application provider cache and bus sink setup for relay sessions.
use crate::{Host, Session};
use codetether_application_providers::{Application, ProviderRegistry};
use codetether_bus_core::AgentBus;
use std::{marker::PhantomData, sync::Arc};
/// Relay host bound to an application registry and session implementation.
/// # Examples
/// ```no_run
/// # fn example<P: codetether_application_providers::Application, S: codetether_cli_relay::Session>() {
/// fn accepts_host<H: codetether_cli_relay::Host>() {}
/// accepts_host::<codetether_cli_relay::application::ApplicationHost<codetether_application_providers::ProviderRegistry<P>, S>>();
/// # }
/// ```
pub struct ApplicationHost<P, S>(PhantomData<fn() -> (P, S)>);
#[async_trait::async_trait(?Send)]
impl<P: Application, S: Session> Host for ApplicationHost<ProviderRegistry<P>, S> {
    type Registry = ProviderRegistry<P>;
    type Session = S;
    fn spawn_bus_sink(bus: Arc<AgentBus>) {
        codetether_bus_s3::spawn_bus_s3_sink(bus);
    }
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry<P>>> {
        ProviderRegistry::shared_from_vault().await
    }
}
