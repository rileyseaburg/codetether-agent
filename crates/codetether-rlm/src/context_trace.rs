//! Context tracing for RLM iterations.
//!
//! Tracks token budget and context events per RLM iteration to enable
//! analysis of context window usage and optimization opportunities.

mod event;
mod event_meta;
mod summary;
mod summary_format;
mod token_estimate;
mod token_factories;
mod trace;
mod trace_budget;
mod trace_events;
mod trace_summary;

pub use event::ContextEvent;
pub use summary::ContextTraceSummary;
pub use trace::ContextTrace;

#[cfg(test)]
mod tests;
