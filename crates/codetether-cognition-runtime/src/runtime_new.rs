//! Constructors for [`CognitionRuntime`](super::CognitionRuntime).

use crate::Host;
use tokio::sync::broadcast;

use super::runtime_assemble::assemble;
use super::{CognitionRuntime, CognitionRuntimeOptions, ThinkerConfig};

/// Capacity of the thought-event broadcast channel.
const EVENT_CHANNEL_CAPACITY: usize = 256;

impl<H: Host> CognitionRuntime<H> {
    /// Build a runtime from environment feature flags.
    pub fn new_from_env() -> Self {
        Self::new_with_options_and_thinker(
            H::options_from_env(),
            Some(H::thinker_config_from_env()),
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
        assemble::<H>(
            options,
            H::load_initial_state(),
            H::init_thinker(thinker_config),
            event_tx,
        )
    }
}
