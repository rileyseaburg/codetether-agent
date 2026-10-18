//! Bounded transcript preparation and prompt-time local recall evidence.
//!
//! [flatten_messages] produces a budgeted transcript for recall consumers.
//! [message] checks the caller's prior-context decision before querying other
//! sessions. [Source] supplies native session inputs without storage ownership.
mod flatten;
mod parts;
mod prefetch;
mod source;
pub use flatten::{flatten_messages, flatten_messages_with_budget};
pub use prefetch::message;
pub use source::Source;
#[cfg(test)]
mod tests;
