//! Stateful mux input filtering and bounded PTY forwarding.
//!
//! [Pipeline] detects detach sequences and preserves bracketed paste across reads.
//! A caller-provided clipboard callback is invoked only when needed. [send]
//! forwards transformed bytes in bounded protocol chunks.
mod clipboard;
mod detach;
mod pipeline;
mod send;
pub(crate) use clipboard::Resolver;
pub use detach::Filtered;
pub use pipeline::Pipeline;
pub use send::send;
