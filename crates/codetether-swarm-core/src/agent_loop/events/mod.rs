//! Live tool events, assistant output accumulation, and UTF-8-safe previews.
//!
//! Supply an optional sender and subtask identity to [started], [finished], and
//! [record_output]. Events use nonblocking delivery; output accumulation does
//! not depend on a connected receiver. [preview] shares truncation with logs.
mod output;
#[path = "preview.rs"]
mod preview_text;
mod tools;
pub use output::record_output;
pub use preview_text::text as preview;
pub use tools::{finished, started};
