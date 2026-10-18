//! Quality-gated compaction output with deterministic chunk fallback.
//!
//! [gated_summary_or_chunk] validates a successful RLM result. [run_keep_last]
//! also handles routing errors and reports successful usage through a callback.
mod gate;
mod result;
pub use gate::gated_summary_or_chunk;
pub use result::run_keep_last;
