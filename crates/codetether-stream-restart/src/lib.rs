//! Whole-request stream recovery with bounded retries and completed checkpoints.
//!
//! [run] opens fresh streams, applies [RestartPolicy], and preserves provider-
//! confirmed output items across attempts. [checkpointed_content] recovers
//! those items from a terminal error so native callers can persist them.
mod attempt;
mod checkpoint;
mod engine;
mod notify;
mod policy;
pub use checkpoint::content_from_error as checkpointed_content;
pub use engine::run;
pub use policy::RestartPolicy;
#[cfg(test)]
mod tests;
