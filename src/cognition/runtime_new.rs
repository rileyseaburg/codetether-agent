//! Constructors for [`CognitionRuntime`](super::CognitionRuntime).

use tokio::sync::broadcast;

use super::initial_state::load_initial_state;
use super::runtime_assemble::assemble;
use super::{
    CognitionRuntime, CognitionRuntimeOptions, ThinkerConfig, options_env, thinker_env,
    thinker_init,
};

/// Capacity of the thought-event broadcast channel.
const EVENT_CHANNEL_CAPACITY: usize = 256;

impl CognitionRuntime {
    /// Build a runtime from environment feature flags.
    pub fn new_from_env() -> Self {
        Self::new_with_options_and_thinker(
            options_env::options_from_env(),
            Some(thinker_env::thinker_config_from_env()),
        )
    }

    /// Build a runtime from explicit options, with no thinker configured.
    pub fn new_with_options(options: CognitionRuntimeOptions) -> Self {
        Self::new_with_options_and_thinker(options, None)
    }

    pub(super) fn new_with_options_and_thinker(
        options: CognitionRuntimeOptions,
        thinker_config: Option<ThinkerConfig>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        assemble(
            options,
            load_initial_state(),
            thinker_init::init_thinker(thinker_config),
            event_tx,
        )
    }
}
