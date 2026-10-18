//! Preserve native cognition initialization and cadence execution.
use super::{CognitionRuntimeOptions, ThinkerClient, ThinkerConfig};
use codetether_cognition_core::runtime::InitialState;
use std::sync::Arc;
/// Application service binding used by the public cognition runtime alias.
pub struct Native;
impl codetether_cognition_runtime::Host for Native {
    type Thinker = ThinkerClient;
    type Registry = crate::tool::ToolRegistry;
    fn options_from_env() -> CognitionRuntimeOptions {
        super::options_env::options_from_env()
    }
    fn thinker_config_from_env() -> ThinkerConfig {
        super::thinker_env::thinker_config_from_env()
    }
    fn load_initial_state() -> InitialState {
        super::initial_state::load_initial_state()
    }
    fn init_thinker(config: Option<ThinkerConfig>) -> Option<Arc<ThinkerClient>> {
        super::thinker_init::init_thinker(config)
    }
    async fn drive(context: super::loop_ctx::LoopCtx) {
        codetether_cognition_runtime::drive(context, super::tick_run::Native).await;
    }
}
