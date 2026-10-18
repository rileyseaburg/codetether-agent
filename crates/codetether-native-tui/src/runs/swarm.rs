//! Swarm execution used by interactive terminal commands.
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig};
use codetether_swarm_runtime::{ExecutorHost, SwarmExecutor};
use codetether_swarm_view::{SwarmEvent, interactive::Host};
use std::marker::PhantomData;
/// Interactive swarm backend using the application's executor host H.
/// # Examples
/// ```
/// let _: Option<codetether_native_tui::runs::Swarm<()>> = None;
/// ```
pub struct Swarm<H>(PhantomData<fn() -> H>);
impl<H: ExecutorHost> Host for Swarm<H> {
    async fn execute(
        config: SwarmConfig,
        control: SwarmControl,
        events: tokio::sync::mpsc::Sender<SwarmEvent>,
        task: String,
        strategy: DecompositionStrategy,
    ) -> Result<(), String> {
        let mut executor = SwarmExecutor::<H>::new(config)
            .with_event_tx(events)
            .with_control(control);
        if let Some(bus) = codetether_bus_core::global::global() {
            executor = executor.with_bus(bus);
        }
        executor
            .execute(&task, strategy)
            .await
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}
