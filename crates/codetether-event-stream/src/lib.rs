//! Chat events, in-process broadcasting, and S3-compatible archival.
//!
//! Construct an [`EventStream`], subscribe before sending [`Event`] values,
//! and use [`s3_sink::S3Sink`] to archive event files.
//!
//! # Examples
//! ```
//! use codetether_event_stream::{Event, EventStream};
//! let stream = EventStream::new();
//! let mut receiver = stream.subscribe();
//! stream.send(Event::AgentMessage { content: "hello".into() }).unwrap();
//! assert!(matches!(receiver.try_recv().unwrap(), Event::AgentMessage { .. }));
//! ```

mod chat;
mod event;
pub mod s3_sink;
mod stream;
pub use chat::ChatEvent;
pub use event::Event;
pub use stream::EventStream;

mod archive;
pub use archive::{archive_event_stream_to_s3, event_stream_path};
