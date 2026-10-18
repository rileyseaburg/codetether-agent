//! Best-effort progress and final outcomes for CLI relay runs.
//!
//! Load [targets], call [advance] after each turn, then [finish] with a [Report].
mod advance;
mod evidence;
mod finish;
mod outcomes;
mod parse;
mod report;
mod targets;
pub use advance::advance;
pub use finish::finish;
pub use report::Report;
pub use targets::targets;
