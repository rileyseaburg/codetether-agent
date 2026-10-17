use super::*;

/// Handler trait for processing messages in the swarm
///
/// A Handler can receive and process messages of a specific type.
/// This enables actors to respond to different message types.
///
/// # Implementors
/// Actors implement this trait for each accepted message type and define the
/// corresponding Send + Sync response.
#[async_trait]
pub trait Handler<M>: Actor {
    /// The response type for this handler
    type Response: Send + Sync;

    /// Handle a message and return a response.
    ///
    /// # Errors
    /// Returns an error if the implementation cannot process this message.
    async fn handle(&mut self, message: M) -> Result<Self::Response>;
}
