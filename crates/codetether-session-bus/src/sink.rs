//! Shared synchronous callback used for durable event writes.
use crate::SessionEvent;
use std::{io, sync::Arc};
/// A shared write callback, invoked before other event transports.
/// # Examples
/// ```
/// use codetether_session_bus::SharedSink;
/// let sink: SharedSink<()> = std::sync::Arc::new(|_| Ok(()));
/// sink(&codetether_session_bus::SessionEvent::Done).unwrap();
/// ```
pub type SharedSink<S> = Arc<dyn Fn(&SessionEvent<S>) -> io::Result<()> + Send + Sync>;
