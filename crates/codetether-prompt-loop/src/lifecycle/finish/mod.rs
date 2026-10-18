//! Ordered persistence, archival, notification and final-result construction.
//! [run] saves before archival and the done event, then gates the final answer.
//! [Host] supplies native persistence and evidence services.
mod host;
mod run;
pub use host::Host;
pub use run::run;
