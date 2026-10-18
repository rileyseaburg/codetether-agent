use crate::{LoopState, StepFlow};
use codetether_provider_core::Message;
use std::future::Future;
/// Native services used while finishing a response.
/// # Implementors
/// Session runners supply transcript insertion, steering, validation and goal flow.
pub trait Host {
    /// Append through the session's existing message path.
    fn append(&mut self, message: Message);
    /// Apply pending steering input and return how many items were added.
    fn drain_steering(&mut self) -> usize;
    /// Current session agent persona.
    fn agent(&self) -> &str;
    /// Mutable counters and accumulated answer output.
    fn progress(&mut self) -> &mut LoopState;
    /// Return validation instructions, or None when validation succeeds.
    /// # Errors
    /// Propagates the validation service's original error.
    fn validate(&mut self) -> impl Future<Output = anyhow::Result<Option<String>>> + Send;
    /// Decide whether to continue an active goal or finish the turn.
    /// # Errors
    /// Propagates goal-continuation errors.
    fn final_flow(&mut self) -> impl Future<Output = anyhow::Result<StepFlow>> + Send;
}
