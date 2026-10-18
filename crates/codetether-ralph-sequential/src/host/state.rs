//! Borrowed run state and iteration persistence inputs.
use codetether_ralph_core::{RalphConfig, RalphState, state_store::RalphStateStore};
use std::sync::Arc;
/// Live state and configuration of the current run.
/// # Implementors
/// Native runtimes return their existing fields without cloning state.
pub trait State: Send + Sync {
    /// Borrow the current state.
    fn state(&self) -> &RalphState;
    /// Mutate the current state.
    fn state_mut(&mut self) -> &mut RalphState;
    /// Borrow configuration for quality checks and automatic commits.
    fn config(&self) -> &RalphConfig;
    /// Borrow the optional persistence service.
    fn store(&self) -> Option<&Arc<dyn RalphStateStore>>;
    /// Borrow the current run identity.
    fn run_id(&self) -> &str;
}
