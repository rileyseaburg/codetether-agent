use super::Attempt;
use codetether_context_policy::{derivation::DerivePolicy, projection::helpers::DerivedContext};
use codetether_provider_core::Message;
use codetether_session_events::SessionEvent;
use std::future::Future;
use tokio::sync::mpsc;
/// Native derivation, delegation and model-switching services.
/// # Implementors
/// A completion attempt adapter owns access to the runner and attempt state.
pub trait Host: Send {
    /// Snapshot carried by native session events.
    type Snapshot: Send;
    /// Mutable context and counters for the current attempt.
    fn attempt(&mut self) -> &mut Attempt;
    /// Derive history using a selected policy and optional keep-last override.
    /// # Errors
    /// Propagates native derivation failures.
    fn derive(
        &mut self,
        policy: DerivePolicy,
        keep: Option<usize>,
    ) -> impl Future<Output = anyhow::Result<DerivedContext>> + Send;
    /// Record an unsuccessful model call in delegation statistics.
    fn failed(&mut self);
    /// Switch to an alternate target, returning whether one was selected.
    /// # Errors
    /// Propagates missing-provider or target-setup errors.
    fn failover(&mut self) -> anyhow::Result<bool>;
    /// Collect proactive context after changing the model target.
    fn proactive(&mut self, step: usize) -> impl Future<Output = Option<Message>> + Send;
    /// Optional event sender for interactive retry breaks.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
}
