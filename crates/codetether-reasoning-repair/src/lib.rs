//! Post-prune reasoning repair through the bundled TetherScript hook.
//!
//! [repair_reasoning] applies the hook with the tetherscript feature enabled;
//! otherwise it returns unchanged messages and default transform statistics.
#[cfg(not(feature = "tetherscript"))]
mod disabled;
#[cfg(feature = "tetherscript")]
mod enabled;
use codetether_context_policy::transforms::ExperimentalStats;
use codetether_provider_core::Message;
/// Repair reasoning metadata after context pruning.
/// # Arguments
/// Receives messages to inspect and repair in place.
/// # Returns
/// Default statistics, preserving the existing hook accounting behavior.
/// # Examples
/// ```
/// let mut messages = Vec::new();
/// let _stats = codetether_reasoning_repair::repair_reasoning(&mut messages);
/// assert!(messages.is_empty());
/// ```
pub fn repair_reasoning(messages: &mut Vec<Message>) -> ExperimentalStats {
    #[cfg(feature = "tetherscript")]
    return enabled::repair_reasoning(messages);
    #[cfg(not(feature = "tetherscript"))]
    {
        disabled::repair_reasoning(messages)
    }
}
