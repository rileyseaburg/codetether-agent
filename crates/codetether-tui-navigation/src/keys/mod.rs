//! Key dispatch after modifier shortcuts and modal overlays are handled.
//!
//! [dispatch] selects navigation and input operations through [Host]. Symbol
//! selection takes precedence over modified Enter and paste-newline handling.
mod host;
mod run;
pub use host::{Host, Input, State};
pub use run::dispatch;
