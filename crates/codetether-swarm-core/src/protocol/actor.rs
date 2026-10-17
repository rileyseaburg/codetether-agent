use super::*;

/// Actor trait for swarm participants
///
/// An Actor is an entity that can participate in the swarm by receiving
/// and processing messages. This is the base trait for all swarm participants.
///
/// # Implementors
/// Execution adapters implement lifecycle and stable identity; implementations
/// must be safe to send and share between threads.
#[async_trait]
pub trait Actor: Send + Sync {
    /// Get the unique identifier for this actor
    fn actor_id(&self) -> &str;

    /// Get the actor's current status
    fn actor_status(&self) -> ActorStatus;

    /// Initialize the actor for swarm participation.
    ///
    /// # Errors
    /// Returns an error if the implementation cannot initialize.
    async fn initialize(&mut self) -> Result<()>;

    /// Shutdown the actor gracefully.
    ///
    /// # Errors
    /// Returns an error if the implementation cannot shut down.
    async fn shutdown(&mut self) -> Result<()>;
}
