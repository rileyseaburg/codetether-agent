//! Application services used by cognition runtime construction and startup.
use codetether_cognition_core::runtime::{InitialState, LoopContext};
use codetether_cognition_core::{CognitionRuntimeOptions, ThinkerConfig};
use std::{fmt::Debug, future::Future, sync::Arc};
/// Native service bindings for the shared cognition runtime facade.
/// # Implementors
/// Applications supply their thinker, tools, environment options, initial
/// state and a driver for the existing cadence loop.
pub trait Host {
    /// Thinker retained in the shared runtime state.
    type Thinker: Debug;
    /// Tool registry retained in the shared runtime state.
    type Registry: Debug;
    /// Read application cognition options from the environment.
    fn options_from_env() -> CognitionRuntimeOptions;
    /// Read thinker configuration from the environment.
    fn thinker_config_from_env() -> ThinkerConfig;
    /// Load initial state, preserving the application's test-mode policy.
    fn load_initial_state() -> InitialState;
    /// Initialize the optional thinker using the existing best-effort policy.
    fn init_thinker(config: Option<ThinkerConfig>) -> Option<Arc<Self::Thinker>>;
    /// Drive the application tick loop with owned shared handles.
    fn drive(
        context: LoopContext<Self::Thinker, Self::Registry>,
    ) -> impl Future<Output = ()> + Send + 'static;
}
