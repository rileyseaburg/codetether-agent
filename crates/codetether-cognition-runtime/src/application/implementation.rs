//! Environment configuration, thinker startup and cadence composition.
use super::{ApplicationHost, TickServices, initialization};
use codetether_cognition_core::{
    CognitionRuntimeOptions, ThinkerConfig,
    execution::dispatch::Registry,
    runtime::{InitialState, LoopContext},
    thinker::client::{Client, Host as ClientHost},
};
use std::{fmt::Debug, sync::Arc};

impl<H: ClientHost + 'static, R: Registry + Debug + 'static, const EMPTY_STATE: bool> crate::Host
    for ApplicationHost<H, R, EMPTY_STATE>
{
    type Thinker = Client<H>;
    type Registry = R;
    fn options_from_env() -> CognitionRuntimeOptions {
        codetether_cognition_core::configuration::options_from_env()
    }
    fn thinker_config_from_env() -> ThinkerConfig {
        codetether_cognition_core::thinker::thinker_config_from_env()
    }
    fn load_initial_state() -> InitialState {
        initialization::load::<EMPTY_STATE>()
    }
    fn init_thinker(config: Option<ThinkerConfig>) -> Option<Arc<Client<H>>> {
        codetether_cognition_core::thinker::client::initialize(config)
    }
    async fn drive(context: LoopContext<Client<H>, R>) {
        crate::drive(context, TickServices).await;
    }
}
