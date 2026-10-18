//! Empty initialization for tests that exercise persona operations only.
use codetether_cognition_core::runtime::{InitialState, LoopContext};
use codetether_cognition_core::{CognitionRuntimeOptions, ThinkerConfig};
use std::sync::Arc;

pub(super) struct TestHost;
impl crate::Host for TestHost {
    type Thinker = ();
    type Registry = ();
    fn options_from_env() -> CognitionRuntimeOptions {
        CognitionRuntimeOptions::default()
    }
    fn thinker_config_from_env() -> ThinkerConfig {
        ThinkerConfig::default()
    }
    fn load_initial_state() -> InitialState {
        InitialState::default()
    }
    fn init_thinker(_: Option<ThinkerConfig>) -> Option<Arc<()>> {
        None
    }
    async fn drive(_: LoopContext<(), ()>) {
        panic!("persona operation tests must not start the background driver");
    }
}
