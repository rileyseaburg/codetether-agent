//! Per-turn edit repetition limits independent of native session state.
//!
//! Construct a [RepeatGuard] for each turn and call [RepeatGuard::check] before
//! executing an edit. Stable signatures ignore volatile metadata recursively.
//!
//! Production session loops use [`ProgressLedger`] instead: it observes results,
//! persists across continuations, and permits successful edits and new evidence.
//! [`RepeatGuard`] remains the legacy per-call compatibility API.
mod filter;
mod guard;
mod limits;
mod signature;
pub use guard::RepeatGuard;
mod ledger;
mod ledger_edit;
mod ledger_human;
mod ledger_nudge;
mod ledger_observe;
mod ledger_record;
mod ledger_result;
mod ledger_signature;
mod result_output;
mod review;
mod review_guidance;
pub use ledger::ProgressLedger;
#[cfg(test)]
mod tests;
