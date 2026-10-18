//! Summary-index node production independent of native sessions.
//!
//! [produce_summary] validates a transcript range, formats its summary request,
//! and uses the existing background RLM cache or deterministic chunk fallback.
//! Hosts supply the same model resolver used by other background jobs.
mod background;
mod call;
mod observability;
pub use call::produce_summary;
pub use observability::SummaryObservability;

mod request;
