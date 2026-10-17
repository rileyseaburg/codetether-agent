//! In-process event broadcasting to independent subscribers.

use super::Event;
use anyhow::Result;
use tokio::sync::broadcast;

/// Broadcasts cloned events to subscribed receivers with a capacity of 256 events.
///
/// Slow receivers observe Tokio's lag error; sending without receivers fails.
///
/// # Examples
/// ```
/// use codetether_event_stream::{Event, EventStream};
/// let stream = EventStream::new();
/// let mut receiver = stream.subscribe();
/// stream.send(Event::Error { message: "example".into() }).unwrap();
/// assert!(receiver.try_recv().is_ok());
/// ```
#[allow(dead_code)]
pub struct EventStream {
    sender: broadcast::Sender<Event>,
}

impl EventStream {
    #[allow(dead_code)]
    /// Create a new event stream
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    #[allow(dead_code)]
    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }

    #[allow(dead_code)]
    /// Send an event to all current subscribers.
    ///
    /// # Arguments
    /// * `event` - Notification to clone into each subscriber's queue.
    ///
    /// # Returns
    /// Unit after successful delivery to the broadcast channel.
    ///
    /// # Errors
    /// Returns an error when no subscribed receivers remain.
    ///
    /// # Examples
    /// ```
    /// use codetether_event_stream::{Event, EventStream};
    /// assert!(EventStream::new().send(Event::Error { message: "x".into() }).is_err());
    /// ```
    pub fn send(&self, event: Event) -> Result<()> {
        self.sender.send(event)?;
        Ok(())
    }
}

impl Default for EventStream {
    fn default() -> Self {
        Self::new()
    }
}
