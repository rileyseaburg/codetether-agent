//! Per-turn edit repetition limits independent of native session state.
//!
//! Construct a [RepeatGuard] for each turn and call [RepeatGuard::check] before
//! executing an edit. Stable signatures ignore volatile metadata recursively.
mod filter;
mod guard;
mod limits;
mod signature;
pub use guard::RepeatGuard;
#[cfg(test)]
mod tests;
