//! Standard ordinary-run bus creation for an application session type.
use crate::{Host, Session};
use codetether_bus_core::AgentBus;
use std::{marker::PhantomData, sync::Arc};
/// Ordinary-run host that installs its bus globally before starting archival.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_run_runtime::Session>() {
/// fn accepts_host<H: codetether_run_runtime::Host>() {}
/// accepts_host::<codetether_run_runtime::application::ApplicationHost<S>>();
/// # }
/// ```
pub struct ApplicationHost<S>(PhantomData<fn() -> S>);
impl<S: Session> Host for ApplicationHost<S> {
    type Session = S;
    fn create_bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        codetether_bus_core::global::set_global(bus.clone());
        codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
        bus
    }
}
