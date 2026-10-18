//! Parent transcript selection and inheritance for child agents.
//!
//! Parse a [ForkTurns] scope and call [inherit] with a [HistorySource]. The
//! selector preserves parent user-turn boundaries, omits system messages and
//! incomplete assistant tails, and retains trailing developer interruption markers.
mod fork_turns;
mod inherit;
mod select;
mod source;
pub use fork_turns::ForkTurns;
pub use inherit::inherit;
pub use source::HistorySource;

/// Implementation support for the application-local session binding.
#[doc(hidden)]
pub mod session_binding;
