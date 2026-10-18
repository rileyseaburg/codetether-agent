//! RLM refinement for relay handoffs and shared context.
//!
//! [prepare_autochat_handoff_with_registry] refines a handoff with a selected
//! provider and bounded fallback. [distill_context_delta_with_rlm] produces a
//! shared-context delta or the existing deterministic fallback.
mod context;
mod handoff;
mod semantic;
pub use context::distill_context_delta_with_rlm;
pub use handoff::prepare_autochat_handoff_with_registry;
mod payload;
