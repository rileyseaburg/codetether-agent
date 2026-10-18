//! Cancellable workspace-symbol discovery and query runtime.
//!
//! [schedule] debounces queries from picker state with the host's LSP tool.
//! [drain] accepts current results; [close] and [cancel] stop pending work.
//! The host supplies a [codetether_lsp_tool::Host] to preserve client identity.
mod completion;
mod drain;
mod gate;
mod languages;
mod lifecycle;
mod query;
mod runtime;
mod schedule;
mod search;
pub use drain::drain;
pub use lifecycle::{active, close};
pub use runtime::cancel;
pub use schedule::schedule;
